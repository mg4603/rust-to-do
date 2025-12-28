.PHONY: all build test fmt run clean check help commit
BINARY := rust-to-do

help:
	@echo "Usage:"
	@echo "  make build									# build binary"
	@echo "  make test									# run all tests"
	@echo "	 make fmt									# format code"
	@echo "	 make run									# build and run (use ARGS env var to pass args)"
	@echo "  make clean									# remove binaries"
	@echo "  make check									# check for errors without building"
	@echo "  make commit -F <file_name>	[-M <message>]	# format code and commit file <file_name> with message <message> if -M flag is used"

fmt:
	@echo "==> formatting"
	@cargo fmt

check:
	@echo "==> checking"
	@cargo check

build: fmt
	@echo "==> building"
	@cargo build --release

all: build

test:
	@echo "==> running tests"
	@cargo test

run: build
	@echo "==> running"
	@if [ -z "$(ARGS)" ]; then \
		./target/release/$(BINARY); \
	else \
		./target/release/$(BINARY) $(ARGS); \
	fi

clean:
	@echo "==> cleaning"
	@cargo clean

commit: fmt
	@git add $(F)
	@if [ -z "$(M)" ]; then \
		git commit; \
	else \
		git commit -m "$(M)"; \
	fi
