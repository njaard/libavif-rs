// bindgen bindings.h --no-prepend-enum-name --allowlist-type="(aom|av1|OBU|AOM|AV).*" --allowlist-function="(aom|av1|OBU|AOM|AV).*" --allowlist-var="(aom|av1|OBU|AOM|AV).*" --no-layout-tests -- -Ivendor | sed -E 's/ ?\\\\brief ?// ' | sed -E 's/doc = " ?< /doc = "/' > src/ffi.rs

#include "aom/aom.h"
#include "aom/aom_encoder.h"
#include "aom/aom_decoder.h"
#include "aom/aomcx.h"
#include "aom/aomdx.h"
