#!/usr/bin/env bash
set -o errexit  # exit when a command fails.
set -o nounset  # exit when using an undeclared variables.
set -o pipefail # exit when anything fails.


protected_branch='main'

if read local_ref local_sha remote_ref remote_sha; then
	
	# Formats all code
	# https://github.com/rust-lang/rustfmt
	cargo fmt --

	# Linting
	# https://github.com/rust-lang/rust-clippy#configuration
	cargo clippy -- -D warnings

	# Dependency checks similar to GH Dependabot
	# https://crates.io/crates/cargo-outdated
	cargo outdated

	# Security checks
	# https://lib.rs/crates/cargo-audit
	cargo-audit audit

fi