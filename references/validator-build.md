# Native Validator Materialization

Read this only when the materialized validator is missing or its native source has changed. Prose-only edits do not require rebuilding.

Do not use `cargo run` as the normal route. Build from this skill's [Cargo.toml](../Cargo.toml) by following the rust-tooling skill, then replace `bin/darwin-arm64/skill-md-clarifier-validator` with the built `release/skill-md-clarifier-validator`. The rust-tooling skill decides the build directory, signing, how to replace the binary, and cleanup for the current host.
