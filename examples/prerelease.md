# Prerelease

Steps to run before publishing a release. Execute all applicable target sections.

## Common (all targets)

1. `make check` (or equivalent test suite)
2. Verify version bumped in manifest (Cargo.toml / package.json / pyproject.toml)
3. Update VERSIONS.md / CHANGELOG.md with new version entry
4. Audit docs: `module-harness audit` (if available)
5. No secrets in the diff or release notes

## Target: cargo (crates.io)

- `cargo publish --dry-run`
- Verify `Cargo.toml` metadata (description, license, repository)

## Target: pypi

- Sync pyproject.toml version with Cargo.toml
- `maturin build` (or `maturin publish --dry-run` if available)
- Use `--no-sdist` if path dev-dependencies exist

## Target: npm

- `npm pack --dry-run`
- Verify `package.json` fields (name, version, main/exports)

## Target: github-release

- Tag exists and matches manifest version
- Binary built: `cargo build --release` (or equivalent)
- Release notes drafted
- `gh release create v<version> --generate-notes` with prebuilt binary

## Target: binary-install

- `cargo install --path .` (update ~/.cargo/bin)
- Verify symlinks (e.g., .bin/agent-doc)
