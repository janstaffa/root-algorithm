# root-fn

A root algorithm implementation in Rust.

note: _this is my own algorithm not meant to be efficent or fast, for production code use the function provided by Rust instead_

## usage:

```rust
use root_fn::root;

// third root of 8
let result = root(8, 3);
asssert_eq!(result, 2);
```

## CLI:

```console
// preview / development
cargo run

// build
cargo build --release

// test
cargo test
```
