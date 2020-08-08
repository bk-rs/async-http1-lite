# async-http1-lite

* [Cargo package](https://crates.io/crates/async-http1-lite)

## Examples

### smol 

* [client](demos/smol/src/client.rs)
* [client_with_http_proxy](demos/smol/src/client_with_http_proxy.rs)
* [client_with_tls](demos/smol/src/client_with_tls.rs)

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
