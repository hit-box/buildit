# Buildit
Library that helps you create builders for your structures with compile-time guarantees.

```rust
use buildit::Builder;

#[derive(Default)]
enum Protocol {
    #[default]
    Http,
    Grpc,
}

#[derive(Builder)]
struct Config {
    port: u16,
    host: &'static str,
    #[builder(default)]
    protocol: Protocol,
}
```
You may use it as ordinary builder:

```rust
let config = Config::builder()
    .port(8080)
    .host("example.rs")
    .build();
```
Or by create `ConfigBuilder` explicit:
```rust
let config = ConfigBuilder::new()
    .host("example.rs")
    .port("443")
    .build();
```

## Features
- Generic and lifetime field support
- Default trait support of custom default value for fields
- Clean and understandable errors
- Multiple time setter calls is allowed
- Simplification of working with partially initialized builders

## Limitations
- Compilation time for huge structures. It may be fixed in one day when specialization will be ready.

## Why not {crate}?

* [typed-builder] - I create [buildit] to fix the complication of using partially initialized builders in the first place and try to extend a type-state pattern-based builder by traits. But if you have an issue with compile time, a typed-builder is a good choice for you.
* [derive-builder] - If you don't need compile-time guarantees is a good choice too, probably best.


[typed-builder]: https://crates.io/crates/typed-builder
[derive-builder]: https://crates.io/crates/derive_builder
[buildit]: https://crates.io/crates/buildit
