NAME=jade-netlist
EXE=target/debug/$(NAME)

profile-perf:
	perf record -g $(EXE)
	perf script | stackcollapse-perf.pl | rust-unmangle | flamegraph.pl > flame.svg

profile-valgrind:
	valgrind --tool=massif $(EXE)

heaptrack: 
	heaptrack $(EXE)

debug: ## debug with gdb
	lldb $(EXE)

wasm-release: ## build the wasm release
	cargo build --target=wasm32-wasi --release

wasm-pack:
	wasm-pack build browser --target web

wasmer-run: wasm-release
	wasmer --dir=. target/wasm32-wasi/release/$(NAME).wasm

wasmtime-run: wasm-release
	wasmtime --dir=. target/wasm32-wasi/release/$(NAME).wasm

build: ## build
	cargo build		

test: ## test
	cargo test

#test-watch: export RUST_BACKTRACE = 1
test-watch: ## test on file change
	cargo watch -x test


run: ## run with backtrace
	RUST_BACKTRACE=1 cargo run

clean: ## clean all the things
	bash clean.bash

work: ## open all files in editor		 
	emacs -nw Makefile `find . -name '*rs'` Cargo* readme.org browser/Cargo*

# http://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk \
	'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

FORCE:

