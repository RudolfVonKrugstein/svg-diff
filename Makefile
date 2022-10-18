all:
	cargo build

node:
	npm run build

wasm:
	wasm-pack build --release --features wasm
