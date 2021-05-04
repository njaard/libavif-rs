 
// bindgen bindings.h --no-prepend-enum-name --whitelist-type="(avif|AVIF).*" --whitelist-function="(avif|AVIF).*" --whitelist-var="(avif|AVIF).*" --no-layout-tests --size_t-is-usize --with-derive-default  -- -Ilibavif/include | sed -E 's/ ?\\\\brief ?// ' | sed -E 's/doc = " ?< /doc = "/' > src/ffi.rs

#include "avif/avif.h"
