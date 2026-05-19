# Releasing

This workspace supports manual, safety-first publish flows. It does not publish automatically on
normal pushes.

## crates.io token setup

1. Create or reuse a crates.io API token with publish access for the intended crates.
2. Add the token to the GitHub repository secrets as `CARGO_REGISTRY_TOKEN`.
3. Do not print the token in logs or local shell history.

## GitHub Actions secret

- Secret name: `CARGO_REGISTRY_TOKEN`

## Dry-run publish

Use the `Publish` workflow with:

- `crate = all` or one specific workspace crate
- `dry_run = true`

This is the default mode and is the safest way to validate publish packaging before a real
release.

## Manual publish

Use the `Publish` workflow with:

- `crate = all` to publish the full workspace in dependency order, or one specific crate
- `dry_run = false`

The workflow will run formatting, linting, tests, and `cargo check` before it attempts any publish
step.

## Line package migration

The text line package publishes as `use-text-line` starting with version `0.1.0`. Its Rust crate
name is `use_text_line`. The `use-text` facade release `0.2.0` reexports that crate and leaves the
public `use-line` package name available for geometry line primitives.

## Post-initial-release automation

After the first manual crates.io release wave for the `use-text` workspace,
the repository can use the `release-plz` workflows for follow-up releases.

### Release PR automation

- Workflow: `Release PR Automation`
- Trigger: pushes to `main` or manual dispatch
- Purpose: opens or updates a release pull request based on
  `release-plz.toml` and the current changelog rules

### Release publish automation

- Workflow: `Release Publish Automation`
- Trigger: manual dispatch only
- Required input: `post-initial-release = true`
- Purpose: confirms all published `use-text` workspace crates already exist on
  crates.io, then runs `release-plz release`

Real release-plz publishes still require `CARGO_REGISTRY_TOKEN` unless the
repository later moves to trusted publishing.

## Local dry-run examples

```sh
cargo publish -p use-case --dry-run
cargo publish -p use-text --dry-run
```

## Semver notes

- Patch bumps are for compatible fixes and small additive maintenance changes.
- Minor bumps are for additive API changes during `0.x` development.
- Major bumps are for breaking changes once the crates stabilize at `1.0.0`.
- Pre-release identifiers should remain intentional and explicit.

## Permanent version warning

Published crates.io versions are permanent. You cannot replace an already published version with
new contents, so verify the crate list, metadata, and changelog inputs before any real publish.
