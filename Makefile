fix:
	cargo fmt
	cargo fix --allow-dirty --allow-staged
	cargo clippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::panic -W clippy::missing_panics_doc -W clippy::panic_in_result_fn -W clippy::cargo_common_metadata

test:
	cargo test
