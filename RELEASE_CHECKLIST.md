# Release Checklist

Refer to this document before publishing a new release of the `velvet` umbrella crate.

- [ ] Update `CHANGELOG.md`
- [ ] Update version number in `Cargo.toml`
- [ ] Upgrade dependencies to latest versions
- [ ] Run the test suite
```bash
$ cargo test --workspace
```
- [ ] Generate benchmark report
```bash
$ cargo criterion --workspace
$ mkdir docs/benches/<version>
$ cp -r target/criterion/reports/* docs/benches/<version>
```
- [ ] Commit the changes
```bash
$ git add .
$ git commit -m "Finalize release <version>"
```
- [ ] Do a dry-run publish
```bash
$ cargo publish --dry-run
```
- [ ] Publish to crates.io
```bash
$ cargo publish
```
- [ ] Wait for documentation to build on docs.rs
- [ ] Write Github release