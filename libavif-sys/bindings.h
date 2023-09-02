 
// bindgen bindings.h --no-prepend-enum-name --allowlist-type="(avif|AVIF).*" --allowlist-function="(avif|AVIF).*" --allowlist-var="(avif|AVIF).*" --no-layout-tests --with-derive-default  -- -Ilibavif/include | sed -E 's/ ?\\\\brief ?// ' | sed -E 's/doc = " ?< /doc = "/' > src/ffi.rs

#include "avif/avif.h"
