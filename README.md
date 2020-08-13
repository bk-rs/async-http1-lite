# async-http1-lite

* [Cargo package](https://crates.io/crates/async-http1-lite)

## Examples

### async-net

* [client](demos/async-net/src/client.rs)
* [client_with_http_proxy](demos/async-net/src/client_with_http_proxy.rs)
* [client_with_tls](demos/async-net/src/client_with_tls.rs)

### async-std

* [client](demos/async-std/src/client.rs)

### tokio

* [client](demos/tokio/src/client.rs)

## Dev

```
cargo clippy --all -- -D clippy::all && \
cargo fmt --all -- --check
```

```
cargo build-all-features
cargo test-all-features
```

```
cargo tarpaulin --all-features
```
