# [WIP] safe-builder
![Continuous Integration](https://github.com/raiden-rs/safe-builder/workflows/Continuous%20Integration/badge.svg)

## Examples

``` Rust
use safe_builder::*;

#[derive(Builder)]
pub struct User {
    id: String,
    name: String,
    addr: Option<String>,
}

fn main() {
    let me = User::builder()
        .id("1234".to_owned())
        .name("bokuweb".to_owned())
        .build();
}
```
