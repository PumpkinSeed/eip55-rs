# EIP-55 written in Rust

[![Latest Version](https://img.shields.io/crates/v/eip55.svg)](https://crates.io/crates/eip55)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.31.0+-green.svg)

Rust implementation of EIP-55

### Usage

- [Crates.io/eip55](https://crates.io/crates/eip55)
- [docs.rs](https://docs.rs/eip55)

```rust
fn main () {
    eip55::validate_address("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"); // returns true
    eip55::checksum("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"); // returns "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"
}
```
