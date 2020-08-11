# async-http1-lite

* [Cargo package](https://crates.io/crates/async-http1-lite)

## Examples

### async-net

* [client](demos/async-net/src/client.rs)
* [client_with_http_proxy](demos/async-net/src/client_with_http_proxy.rs)
* [client_with_tls](demos/async-net/src/client_with_tls.rs)

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
