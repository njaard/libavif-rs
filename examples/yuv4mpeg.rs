use libavif::{AddImageFlags, BorrowedAvifImage, ChromaSamplePosition, Encoder, YuvFormat};
use std::time::Instant;
use std::{env, fs, io};
use y4m::Colorspace;

#[derive(Debug)]
struct Y4MFrameConfig {
    dimensions: (i32, i32),
    duration: u64,
    timescale: u64,
    row_bytes: (u32, u32, u32),
    bit_depth: i32,
    yuv_format: YuvFormat,
    chroma_sample_position: ChromaSamplePosition,
}

impl Y4MFrameConfig {
    fn new<R: io::Read>(decoder: &y4m::Decoder<R>) -> Self {
        let (y_bytes, u_bytes, v_bytes) =
            get_plane_row_bytes(decoder.get_width(), decoder.get_colorspace());
        let (yuv_format, chroma_sample_position) = convert_colorspace(decoder.get_colorspace());
        Self {
            dimensions: (decoder.get_width() as _, decoder.get_height() as _),
            duration: decoder.get_framerate().den as _,
            timescale: decoder.get_framerate().num as _,
            row_bytes: (y_bytes as _, u_bytes as _, v_bytes as _),
            bit_depth: decoder.get_bit_depth() as _,
            yuv_format,
            chroma_sample_position,
        }
    }

    fn create_image<'y, 'u, 'v, 'a>(&self) -> Option<BorrowedAvifImage<'y, 'u, 'v, 'a>> {
        let mut img = BorrowedAvifImage::new(
            self.dimensions.0,
            self.dimensions.1,
            self.bit_depth,
            self.yuv_format,
        )?;
        img.set_y_row_bytes(self.row_bytes.0)
            .set_u_row_bytes(self.row_bytes.1)
            .set_v_row_bytes(self.row_bytes.2)
            .set_chroma_sample_position(self.chroma_sample_position);
        Some(img)
    }
}

/// Converts the colorspace to a yuv-format and chroma-sample-position for the specific format
fn convert_colorspace(colorspace: Colorspace) -> (YuvFormat, ChromaSamplePosition) {
    match colorspace {
        Colorspace::C420 | Colorspace::C420p10 | Colorspace::C420p12 | Colorspace::C420jpeg => {
            (YuvFormat::Yuv420, Default::default())
        }
        Colorspace::C420mpeg2 => (YuvFormat::Yuv420, ChromaSamplePosition::Vertical),
        Colorspace::C420paldv => (YuvFormat::Yuv420, ChromaSamplePosition::Colocated),
        Colorspace::C422 | Colorspace::C422p10 | Colorspace::C422p12 => {
            (YuvFormat::Yuv422, Default::default())
        }
        Colorspace::C444 | Colorspace::C444p10 | Colorspace::C444p12 => {
            (YuvFormat::Yuv444, Default::default())
        }
        Colorspace::Cmono => (YuvFormat::Yuv400, Default::default()),
    }
}

/// From y4m [get_plane_sizes][1]. Adapted to return the bytes per row.
///
/// [1]: https://github.com/image-rs/y4m/blob/7d1024083e84603cbd171fc849154035ff0592b8/src/lib.rs#L264-L286
fn get_plane_row_bytes(width: usize, colorspace: Colorspace) -> (usize, usize, usize) {
    let y_plane_size = width * colorspace.get_bytes_per_sample();

    let c420_chroma_size = ((width + 1) / 2) * colorspace.get_bytes_per_sample();
    let c422_chroma_size = ((width + 1) / 2) * colorspace.get_bytes_per_sample();

    let c420_sizes = (y_plane_size, c420_chroma_size, c420_chroma_size);
    let c422_sizes = (y_plane_size, c422_chroma_size, c422_chroma_size);
    let c444_sizes = (y_plane_size, y_plane_size, y_plane_size);

    match colorspace {
        Colorspace::Cmono => (y_plane_size, 0, 0),
        Colorspace::C420
        | Colorspace::C420p10
        | Colorspace::C420p12
        | Colorspace::C420jpeg
        | Colorspace::C420paldv
        | Colorspace::C420mpeg2 => c420_sizes,
        Colorspace::C422 | Colorspace::C422p10 | Colorspace::C422p12 => c422_sizes,
        Colorspace::C444 | Colorspace::C444p10 | Colorspace::C444p12 => c444_sizes,
    }
}

fn main() {
    let input = env::args().nth(1).expect("input filename or --stdin");
    let output = env::args().nth(2).expect("output filename");
    let input: Box<dyn io::Read> = match input.as_str() {
        "--stdin" => {
            if cfg!(windows) {
                eprintln!("WARNING: Rust's implementation of Stdin on Windows doesn't support non UTF-8 strings and piping from ffmpeg will almost always fail!");
            }
            Box::new(io::stdin().lock())
        }
        input => Box::new(fs::File::open(input).expect("couldn't open input file")),
    };
    let mut decoder = y4m::decode(input).expect("couldn't create decoder");

    let config = Y4MFrameConfig::new(&decoder);
    let mut encoder = Encoder::new();
    encoder.set_timescale(config.timescale);

    let start_ts = Instant::now();
    let mut frame_counter = 1;
    loop {
        let frame = match decoder.read_frame() {
            Ok(frame) => frame,
            Err(y4m::Error::EOF) => break,
            Err(e) => panic!("y4m decoder cannot read another frame: {}", e),
        };

        let frame_start_ts = Instant::now();
        let mut image = config.create_image().expect("couldn't create image");
        image
            .set_y(frame.get_y_plane())
            .set_u(frame.get_u_plane())
            .set_v(frame.get_v_plane());

        encoder
            .add_image(&image, config.duration, AddImageFlags::NONE)
            .expect("couldn't add image");
        eprintln!(
            "Encoded frame {} in {:?}",
            frame_counter,
            Instant::now().duration_since(frame_start_ts)
        );
        frame_counter += 1;
    }

    let finish_start_ts = Instant::now();
    eprintln!("Finishing encoding");
    let data = encoder.finish().expect("couldn't finish encoding");
    let finished_ts = Instant::now();
    eprintln!(
        "Finished encoding in {:?}, total time: {:?}",
        finished_ts.duration_since(finish_start_ts),
        finished_ts.duration_since(start_ts)
    );

    fs::write(&output, &*data).expect("couldn't write output");
}
