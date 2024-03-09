build:
	cargo build

release:
	cargo build --release

lint:
	cargo check

lintfix:
	cargo fmt --all -- --check
