.PHONY: install
install:
	cargo install --path .

.PHONY: publish
publish:
	@if [ "$(CARGO_TOKEN)" = "" ]; then echo "CARGO_TOKEN variable not set"; exit 1; fi
	cargo login $(CARGO_TOKEN)
	cargo publish

.PHONY: clean
clean:
	@cargo clean
