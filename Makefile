.PHONY: install
install:
	cargo install --path . --offline

.PHONY: publish
publish:
	@if [ "$(CARGO_TOKEN)" = "" ]; then echo "CARGO_TOKEN variable not set"; exit 1; fi
	cargo login $(CARGO_TOKEN)
	cargo publish

.PHONY: build-mac-m1
build-mac-m1:
	rustup target add aarch64-apple-darwin
	cargo build --release --target aarch64-apple-darwin
	mkdir -p ./release/
	cp ./target/aarch64-apple-darwin/release/ctrlg ./release/ctrlg-macos-arm

.PHONY: build-mac-x86
build-mac-x86:
	rustup target add x86_64-apple-darwin
	cargo build --release --target x86_64-apple-darwin
	mkdir -p ./release/
	cp ./target/x86_64-apple-darwin/release/ctrlg ./release/ctrlg-macos-x86

.PHONY: build-linux
build-linux:
	cargo build --release --target x86_64-unknown-linux-gnu
	mkdir -p ./release/
	cp ./target/x86_64-unknown-linux-gnu/release/ctrlg ./release/ctrlg-linux-x86

.PHONY: check
check:
	cargo fmt -- --check
	cargo clippy -- -D warnings
	cargo test
