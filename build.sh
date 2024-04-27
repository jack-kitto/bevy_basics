#/bin/bash

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
	--out-dir ./out/ \
	--out-name "BevyBasics" \
	./target/wasm32-unknown-unknown/release/bevy_basics.wasm
