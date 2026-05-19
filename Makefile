.PHONY: help fmt check lint test test-minimal build doc examples audit deny sbom publish-dry-run release-readiness verify

PUBLISH_CRATES := use-case use-slug use-token use-word use-text-line use-text
SBOM_MANIFEST := crates/use-text/Cargo.toml

help:
	@printf "%s\n" \
		"help               Show available repository tasks" \
		"fmt                Check formatting with rustfmt" \
		"check              Run cargo check for the workspace" \
		"lint               Run clippy with warnings denied" \
		"test               Run workspace tests with all features" \
		"test-minimal       Run workspace tests with no default features" \
		"build              Build the workspace with all features" \
		"doc                Build workspace docs without dependencies" \
		"examples           Check all examples" \
		"audit              Run cargo-audit" \
		"deny               Run cargo-deny" \
		"sbom               Generate a CycloneDX SBOM" \
		"publish-dry-run    List package contents and dry-run publish workspace crates" \
		"release-readiness  Run the pre-release validation path" \
		"verify             Run the main workspace validation path"

fmt:
	cargo fmt --all -- --check

check:
	cargo check --workspace --all-features

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features

test-minimal:
	cargo test --workspace --no-default-features

build:
	cargo build --workspace --all-features

doc:
	cargo doc --workspace --all-features --no-deps

examples:
	cargo check --workspace --all-features --examples

audit:
	cargo audit

deny:
	cargo deny check

sbom:
	cargo cyclonedx --manifest-path $(SBOM_MANIFEST) --all-features --format json --spec-version 1.5 --override-filename sbom.cyclonedx

publish-dry-run:
	@for crate in $(PUBLISH_CRATES); do \
		cargo package --list -p $$crate; \
		cargo publish --dry-run --allow-dirty -p $$crate; \
	done

release-readiness: verify examples test-minimal publish-dry-run

verify: fmt lint test build
