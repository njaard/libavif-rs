// bindgen bindings.h --no-prepend-enum-name --whitelist-type="(aom|av1|OBU|AOM|AV).*" --whitelist-function="(aom|av1|OBU|AOM|AV).*" --whitelist-var="(aom|av1|OBU|AOM|AV).*" --no-layout-tests --size_t-is-usize  -- -Ivendor | sed -E 's/ ?\\\\brief ?// ' | sed -E 's/doc = " ?< /doc = "/' > src/ffi.rs

#include "aom/aom.h"
#include "aom/aom_encoder.h"
#include "aom/aom_decoder.h"
#include "aom/aomcx.h"
#include "aom/aomdx.h"
