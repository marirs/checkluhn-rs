Check Luhn
===========
![Crates.io](https://img.shields.io/crates/v/checkluhn)
[![Documentation](https://docs.rs/checkluhn/badge.svg)](https://docs.rs/checkluhn/0.0.1/checkluhn/fn.validate.html)

[![Build Status](https://travis-ci.com/marirs/checkluhn-rs.svg?branch=main)](https://travis-ci.com/marirs/checkluhn-rs)

Validate the Luhn algorithm for the given string.

### Requirements
- Rust 1.40+

### Usage
```toml
[dependencies]
checkluhn = "0.0.1"
```

and 

```rust
use checkluhn;

fn main() {
    let n = "4111111111111111";
    assert!(checkluhn::validate(n));
}
```

---
LICENSE: MIT
