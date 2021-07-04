BINDGEN_ARGS="--no-doc-comments --no-prepend-enum-name"
bindgen $BINDGEN_ARGS ./synthizer-vendored/include/synthizer.h > ./src/bindgen/synthizer.rs
bindgen $BINDGEN_ARGS ./synthizer-vendored/include/synthizer_constants.h > ./src/bindgen/synthizer_constants.rs
