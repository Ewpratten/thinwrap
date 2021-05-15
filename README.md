# ThinWrap

[![Crates.io](https://img.shields.io/crates/v/thinwrap)](https://crates.io/crates/thinwrap) ![Build](https://github.com/Ewpratten/thinwrap/workflows/Build/badge.svg)

ThinWrap is a very small Rust library that provides a macro (`thin_wrap!`) to wrap any struct in an outer struct, and generate its `Deref` and `DerefMut` traits automatically.

## Example

**With ThinWrap:**

```rust
pub struct Inner;

thin_wrap!(pub, Outer, Inner);
```

**Without ThinWrap:**

```rust
pub struct Inner;

pub struct Outer(Inner);

impl Deref for Outer {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Outer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
```