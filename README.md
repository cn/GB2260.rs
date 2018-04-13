# GB2260.rs

[![Build Status](https://travis-ci.org/messense/GB2260.rs.svg)](https://travis-ci.org/messense/GB2260.rs)
[![Crates.io](https://img.shields.io/crates/v/gb2260.svg)](https://crates.io/crates/gb2260)
[![codecov](https://codecov.io/gh/messense/GB2260.rs/branch/master/graph/badge.svg)](https://codecov.io/gh/messense/GB2260.rs)
[![docs.rs](https://docs.rs/gb2260/badge.svg)](https://docs.rs/gb2260/)

The Rust implementation for looking up Chinese administrative divisions.

## Installation

Add it to your ``Cargo.toml``:

```toml
[dependencies]
gb2260 = "0.1"
```

Add ``extern crate gb2260`` to your crate root and your're good to go!

## Example

```rust
extern crate gb2260;

use gb2260::Division;

fn main() {
    let division = Division::get("110000").unwrap();
    assert_eq!(division.code, "110000");
    assert_eq!(division.name, "北京市");
    assert!(division.is_province());
    assert!(!division.is_prefecture());
    assert!(!division.is_county());
}
```

## License

This work is released under the MIT license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
