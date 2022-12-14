all:
	cargo build

node:
	npm run build

wasm:
	wasm-pack build --target web --release --features wasm

is_ok:
	cargo fmt --check
	cargo clippy --fix --all-features
