BINDGEN_ARGS="--no-doc-comments --no-prepend-enum-name --with-derive-default"
bindgen $BINDGEN_ARGS ./synthizer-vendored/include/synthizer.h > ./src/synthizer.rs
bindgen $BINDGEN_ARGS ./synthizer-vendored/include/synthizer_constants.h > ./src/synthizer_constants.rs
