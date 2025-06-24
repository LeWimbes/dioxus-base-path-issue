# Dioxus Base Path Issue

## `dx run`
The `dx run` command works correctly in all scenarios (with/without SSG, debug/release mode):

```bash
# Works (debug mode)
cargo run --manifest-path ../dioxus/Cargo.toml --package dioxus-cli --release -- \
    run --verbose --bin base-path --platform web --base-path base-path
```

```bash
# Works (release mode)
cargo run --manifest-path ../dioxus/Cargo.toml --package dioxus-cli --release -- \
    run --verbose --release --bin base-path --platform web --base-path base-path
```

```bash
# Kinda works (debug mode with SSG)
# Visiting a page that does not exist (e.g. http://127.0.0.1:8080/base-path/abc) results in some error that is not just 404
# Vising a page that is not home (e.g. http://127.0.0.1:8080/base-path/page) often strips the base_path (e.g. http://127.0.0.1:8080/page)
cargo run --manifest-path ../dioxus/Cargo.toml --package dioxus-cli --release -- \
    run --verbose --bin base-path-ssg --platform web --base-path base-path --ssg
```

```bash
# Kinda works (release mode with SSG)
# Visiting a page that does not exist (e.g. http://127.0.0.1:8080/base-path/abc) results in some error that is not just 404
# Vising a page that is not home (e.g. http://127.0.0.1:8080/base-path/page) often strips the base_path (e.g. http://127.0.0.1:8080/page)
cargo run --manifest-path ../dioxus/Cargo.toml --package dioxus-cli --release -- \
    run --verbose --release --bin base-path-ssg --platform web --base-path base-path --ssg
```

## `dx bundle`

The `dx bundle` command only works correctly in the release mode:

```bash
# Does not work (debug mode)
# Does not respect the base path
cargo run --manifest-path ../dioxus/Cargo.toml --package dioxus-cli --release -- \
    bundle --bin base-path --platform web --base-path base-path && \
    PORT=8080 ./target/dx/base-path/debug/web/base-path
```

```bash
# Works (release mode)
cargo run --manifest-path ../dioxus/Cargo.toml --package dioxus-cli --release -- \
    bundle --release --bin base-path --platform web --base-path base-path && \
    PORT=8080 ./target/dx/base-path/release/web/base-path
```

```bash
# Does not work (debug mode with SSG)
# Does not respect the base path
cargo run --manifest-path ../dioxus/Cargo.toml --package dioxus-cli --release -- \
    bundle --bin base-path-ssg --platform web --base-path base-path --ssg && \
    PORT=8080 ./target/dx/base-path-ssg/debug/web/base-path-ssg
```

```bash
# Works (release mode with SSG)
cargo run --manifest-path ../dioxus/Cargo.toml --package dioxus-cli --release -- \
    bundle --release --bin base-path-ssg --platform web --base-path base-path --ssg && \
    PORT=8080 ./target/dx/base-path-ssg/release/web/base-path-ssg
```
