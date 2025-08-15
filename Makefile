coverage_rust_flags = -C instrument-coverage
coverage_path = ./target/debug/coverage
coverage_llvm_file = ${coverage_path}/profile/app-%p-%m.profraw

fix:
	cargo fmt
	cargo fix --allow-dirty --allow-staged
	cargo clippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::panic -W clippy::missing_panics_doc -W clippy::panic_in_result_fn -W clippy::cargo_common_metadata

test:
	cargo test

coverage:
	rm -rf ${coverage_path}
	mkdir -p ${coverage_path}
	RUSTFLAGS="${coverage_rust_flags}" LLVM_PROFILE_FILE="${coverage_llvm_file}" cargo build
	RUSTFLAGS="${coverage_rust_flags}" LLVM_PROFILE_FILE="${coverage_llvm_file}" cargo test
	LLVM_PROFILE_FILE="${coverage_llvm_file}" grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ${coverage_path} --excl-line "#\[derive"
	cat ${coverage_path}/coverage.json | jq
	rm -rf ${coverage_path}/profile/
