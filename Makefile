buildCmd := cargo build
lintCmd := cargo clippy

build:
	eval "$(buildCmd)"

release:
	eval "$(buildCmd) --release"

lint:
	eval "$(lintCmd)"

lintfix:
	eval "$(lintCmd) --fix"
