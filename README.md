# minesweeper

minesweeper game in rust for learning purposes

game logic based on https://www.youtube.com/watch?v=HBrF8LJ0Hfg / https://github.com/zigurous/unity-minesweeper-tutorial
the sprites are from the tutorial as well.


## reference
- https://docs.rs/comfy/latest/comfy/
- https://comfyengine.org/book/releasing/
- https://github.com/darthdeus/comfy/tree/master/comfy/examples

## build

    cargo run

    cargo build --release --features comfy/ci-release

## build for web

    trunk serve --open

    trunk build --release --features comfy/ci-release


## other

- https://rustwasm.github.io/wasm-bindgen/api/web_sys/
- https://rustwasm.github.io/wasm-bindgen/examples/dom.html

TODO: https://github.com/trunk-rs/trunk/tree/main/examples/vanilla