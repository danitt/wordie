build:
	cargo build

install:
	cargo install --path .
	@echo "Binary globally installed"

test:
	cargo test -- --show-output