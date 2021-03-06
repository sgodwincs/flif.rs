# flif.rs
<p align="center">
  <img src="https://cdn.rawgit.com/dgriffen/flif.rs/e9cb5c4a/flif.rs.svg" alt="logo" height=150 />
</p>

 [![Build Status](https://travis-ci.org/dgriffen/flif.rs.svg?branch=master)](https://travis-ci.org/dgriffen/flif.rs) [![version][version-badge]][CHANGELOG] [![license][license-badge]][LICENSE]

flif.rs is a Rust implementation of the [flif16](http://flif.info/spec.html) image format. This project was inspired by the work on [flif-rs](https://github.com/panicbit/flif-rs).
## Current Status

Currently this project is pre-alpha. As of right now it is only capable of decoding header data, pixel decoding is not yet implemented. The project will enter alpha status once enough pixel decoding is implemented to successfully decode the reference image in the examples folder.

## Development
### Prerequisites
- rustc (either via rustup or your distributions package manager)
- cargo (via the same method as above)

### Building
- `git clone https://github.com/dgriffen/flif.rs.git`
- `cd flif.rs`
- `cargo build`

## Usage
1. add this crate to your crates `Cargo.toml` like so:
```toml
[package]
name = "some_package"
version = "0.0.1"
authors = ["John Doe <you@example.com>"]

[dependencies]
flif = { git = "https://github.com/dgriffen/flif.rs" }
```
2. in the root of your project reference the crate:
```rust
extern crate flif;
```
3. the crate can now be used to decode flif headers :D
```rust
extern crate flif;

use flif::Decoder;

fn main() {
    let file = std::fs::File::open("some_image.flif").unwrap();
    let mut decoder = Decoder::new(file);
    let flif = decoder.decode().unwrap();
    println!("{:?}", flif.header);
    println!("{:?}", flif.metadata);
    println!("{:?}", flif.second_header);
}
```

### Trademarks
The flif.rs logo is a combination of the official flif logo and Rust logo.

[CHANGELOG]: ./CHANGELOG.md
[LICENSE]: ./LICENSE
[version-badge]: https://img.shields.io/badge/version-0.0.2-blue.svg
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[logo]: ./flif.rs.png
