
run:
	@cd www && npm run start

build-release:
	@wasm-pack build --release

build:
	@wasm-pack build

