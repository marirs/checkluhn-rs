Check Luhn
===========

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
