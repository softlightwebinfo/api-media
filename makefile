CARGO = cargo

build:
	$(CARGO) build

check:
	$(CARGO) check

fmt:
	$(CARGO) fmt

open:
	open http://localhost:8088

release:
	$(CARGO) build --release

run:
	$(CARGO) run

start:
	$(CARGO) run --release

dev:
	$(CARGO)  watch -x run
