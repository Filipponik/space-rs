coverage_flags = "-C instrument-coverage"
coverage_path = "./target/debug/coverage"

fix:
	cargo fmt
	cargo fix --allow-dirty --allow-staged
	cargo clippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::panic -W clippy::missing_panics_doc -W clippy::panic_in_result_fn -W clippy::cargo_common_metadata

test:
	cargo test

coverage:
	rm -rf ./target/debug/coverage
	RUSTFLAGS=${coverage_flags} cargo build
	RUSTFLAGS=${coverage_flags} cargo test
	grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ${coverage_path}
	cat ${coverage_path}/coverage.json | jq
	rm -rf ./default_*.profraw
