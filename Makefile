##
# Test and build hc-zomes Project
#
# This Makefile is primarily instructional; you can simply enter the Nix environment for
# holochain development (supplied by holonix;) via `nix-shell` and run
# `make test` directly, or build a target directly, eg. `nix-build -A hc-zomes`.
#
SHELL		= bash
DNANAME		= integrity-template
HAPP		= ./dna/workdir/happ/$(DNANAME).happ
DNA		= ./dna/workdir/dna/$(DNANAME).dna
WASM		= ./target/wasm32-unknown-unknown/release/integrity.wasm \
		  ./target/wasm32-unknown-unknown/release/coordinator.wasm

# External targets; Uses a nix-shell environment to obtain Holochain runtimes, run tests, etc.
.PHONY: all FORCE
all: nix-test

# nix-test, nix-install, ...
nix-%:
	nix-shell --pure --run "make $*"

# Internal targets; require a Nix environment in order to be deterministic.
# - Uses the version of `hc` on the system PATH.
# - Normally called from within a Nix environment, eg. run `nix-shell`
.PHONY:		rebuild install build
rebuild:	clean build

install:	build

build:		$(HAPP)

$(HAPP):	$(DNA)
	@echo "Packaging HAPP:"
	@hc app pack dna/workdir/happ
	@ls -l $@

# Package the DNA from the built target release WASM
$(DNA):		$(WASM) FORCE
	@echo "Packaging DNA:"
	@hc dna pack dna/workdir/dna
	@ls -l $@

# Recompile the target release WASM
$(WASM): FORCE
	@echo "Building  DNA WASM:"
	@RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build \
	    --release --target wasm32-unknown-unknown


.PHONY: test test-all test-unit test-e2e test-dna test-dna-debug test-stress test-sim2h test-node
test-all:	test

# test:		test-unit test-e2e # test-stress # re-enable when Stress tests end reliably

# test-unit:
# 	RUST_BACKTRACE=1 cargo test \
# 	    -- --nocapture

# test-dna:	$(DNA) FORCE
# 	@echo "Starting Scenario tests in $$(pwd)..."; \
# 	    ( [ -d  node_modules ] || npm install ) && npm test

# test-dna-debug: $(DNA) FORCE
# 	@echo "Starting Scenario tests in $$(pwd)..."; \
# 	    ( [ -d  node_modules ] || npm install ) && npm run test-debug

# test-e2e:	test-dna

#
# Testing
#
tests/package-lock.json:	tests/package.json
	touch $@
tests/node_modules:		tests/package-lock.json
	cd tests; npm install
	touch $@
test:		$(DNA) tests/node_modules
	cd tests; npx mocha integration/test_api.js
test-debug:	$(DNA) tests/node_modules
	cd tests; LOG_LEVEL=silly npx mocha integration/test_api.js


# Generic targets; does not require a Nix environment
.PHONY: clean
clean:
	rm -rf \
	    dist \
	    tests/node_modules \
	    .cargo \
	    target \
	    Cargo.lock
