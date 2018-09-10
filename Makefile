build:
	cargo build --release --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/experiment.wasm .

test-add: build
	cargo run --example add
