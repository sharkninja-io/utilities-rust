# = Parameters
# Override envars using -e

#
# = Common
#

# Checks two given strings for equality.
eq = $(if $(or $(1),$(2)),$(and $(findstring $(1),$(2)),\
                                $(findstring $(2),$(1))),1)

#
# = Targets
#

.PHONY : \
	doc \
	fmt \
	clippy \
	test \
	nextest \
	build \
	clean \
	install \
	install_and_test


# Compile application for running on local machine
#
# Usage :
#	make build

build :
	cargo build --workspace

# Test cloudcore
#
# Usage :
#	make test

test :
	cargo test

# Generate crates documentation from Rust sources.
#
# Usage :
#	make doc [private=(yes|no)] [clean=(no|yes)]

doc :
ifeq ($(clean),yes)
	@rm -rf target/doc/
endif
	cargo doc --all-features --workspace \
		$(if $(call eq,$(private),no),,--document-private-items)

# Check formatting of the sources.
#
# Usage :
#	make fmt-check

fmt-check :
	cargo +nightly fmt --all --check

# Lint Rust sources with Clippy.
#
# Usage :
#	make clippy

clippy :
	cargo clippy --workspace --all-features --all-targets -- -D warnings

# Next-generation test runner for Rust.
# cargo nextest ignores the doctests at the moment. So if you are using it locally you also have to run `cargo test --doc`.
# Usage:
# 	make nextest

nextest :
	cargo nextest run

# Run project Rust sources with Cargo.
#
# Usage :
#	make clean

clean :
	cargo clean && cargo update

# Run format checks, clippy and tests for cloudcore.
#
# Usage :
#	make precommit

precommit : fmt-check clippy test

# Install required tools.
#
# Usage :
#	make install

install :
	if ! command -v rustup >/dev/null 2>&1; then\
		curl https://sh.rustup.rs -sSf | sh -s -- -y;\
		exec bash -l $ENTRY_SCRIPT;\
	fi

	rustup --version
	cargo --version
	rustc --version

	rustup toolchain install nightly

	rustup component add rustfmt clippy
	rustup component add rustfmt --toolchain nightly

	cargo install cargo-nextest

# Install required tools and run the ci/cd tests.
#
# Usage :
#	make install_and_test

install_and_test : install fmt-check clippy build test
