# Native Validator Materialization

Read this only when the materialized validator is missing or its native source has changed. Run the commands from the `skill-md-clarifier` source root.

Do not use `cargo run` as the normal route. If the materialized binary is absent or its native source changed, materialize it from this skill root using a new task-owned temporary build directory. Confirm the manifest owner and exact target directory in the metadata output before building or cleaning:

```bash
clarifier_manifest="$PWD/Cargo.toml"
clarifier_build_dir="$(mktemp -d "${TMPDIR:-/tmp}/skill-md-clarifier-build.XXXXXX")"
CARGO_TARGET_DIR="$clarifier_build_dir" cargo metadata --locked --no-deps --format-version 1 --manifest-path "$clarifier_manifest"
cargo build --locked --release --manifest-path "$clarifier_manifest" --target-dir "$clarifier_build_dir"
mkdir -p bin/darwin-arm64
install -m 755 "$clarifier_build_dir/release/skill-md-clarifier-validator" bin/darwin-arm64/skill-md-clarifier-validator
cargo clean --manifest-path "$clarifier_manifest" --target-dir "$clarifier_build_dir"
rmdir "$clarifier_build_dir"
```

If construction fails or is interrupted, clean that same verified temporary target with the shown `cargo clean` command and remove its empty task-created directory; preserve the stable binary and pre-existing build directories. The example is a materialization route, not an instruction to build for prose-only edits.

