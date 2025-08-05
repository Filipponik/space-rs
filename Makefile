fix:
	cargo fmt
	cargo fix --allow-dirty --allow-staged
	cargo clippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::panic -W clippy::missing_panics_doc -W clippy::panic_in_result_fn -W clippy::cargo_common_metadata

build-static:
	docker run -v ./:/volume --rm -t clux/muslrust:stable cargo build --release
	upx --best --lzma target/*-linux-musl/release/space-rs-cli

# Run tests
test:
	cargo test
