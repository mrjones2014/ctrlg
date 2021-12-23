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
	cargo build --release --target aarch64-apple-darwin
	strip target/aarch64-apple-darwin/release/ctrlg
	otool -L target/aarch64-apple-darwin/release/ctrlg
	mkdir -p release
	tar -C ./target/aarch64-apple-darwin/release/ -czvf ./release/ctrlg-mac-m1.tar.gz ./ctrlg

.PHONY: build-mac-x86
build-mac-x86:
	cargo build --release --target x86_64-apple-darwin
	strip target/x86_64-apple-darwin/release/ctrlg
	otool -L target/x86_64-apple-darwin/release/ctrlg
	mkdir -p release
	tar -C ./target/x86_64-apple-darwin/release/ -czvf ./release/ctrlg-mac-x86.tar.gz ./ctrlg

.PHONY: build-linux
build-linux:
	cargo build --release --target x86_64-unknown-linux-gnu
	mkdir -p release
	tar -C ./target/x86_64-unknown-linux-gnu/release/ -czvf ./release/ctrlg-linux.tar.gz ./ctrlg

.PHONY: check
check:
	cargo fmt -- --check
	cargo clippy -- -D warnings
	cargo test
