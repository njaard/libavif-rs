#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum YuvFormat {
    Yuv400 = 4,
    Yuv420 = 3,
    Yuv422 = 2,
    Yuv444 = 1,
}
