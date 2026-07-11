# AGENTS.md

## Code-Qualität (Pflicht vor jedem Abschluss)

Vor dem Beenden einer Aufgabe muss der Code sauber sein. Immer ausführen und grün bekommen:

```
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo build
```

- `cargo fmt --all -- --check` muss ohne Diff durchlaufen (die CI bricht sonst ab).
- `cargo clippy` muss ohne Warnings durchlaufen (`-D warnings`).
- Änderungen erst als fertig melden, wenn fmt, clippy und build grün sind.

## Toolchain

- Rust-Version ist in `rust-toolchain.toml` gepinnt (aktuell 1.97.0, stable).
- Dockerfile und CI müssen dieselbe Version verwenden.
