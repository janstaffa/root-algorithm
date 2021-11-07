# root-fn
A square root function implementation in Rust.

note: *this is my own algorithm not meant to be efficent or fast, for production code use the function provided by Rust instead*


## usage:
```rust
use root_fn::sqrt;

let result = sqrt(4);
asssert_eq!(result, 2);
```
## CLI:
```rust
// preview / development
cargo run

// build
cargo build --release

// test
cargo test
```
