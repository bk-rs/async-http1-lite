# async-http1-lite

* [Cargo package](https://crates.io/crates/async-http1-lite)

## Examples

### smol 

* [client](demos/smol/src/client.rs)

## Dev

```
cargo test --all-features --all -- --nocapture && \
cargo clippy --all -- -D clippy::all && \
cargo fmt --all -- --check
```

```
cargo build-all-features
cargo test-all-features --all
```

```
cargo tarpaulin --all-features
```
