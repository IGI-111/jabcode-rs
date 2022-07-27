# jabcode-rs
<a href="https://crates.io/crates/jabcode"><img src="https://img.shields.io/crates/v/jabcode.svg" alt="Crate status"/></a>
<a href="https://docs.rs/jabcode"><img src="https://img.shields.io/docsrs/jabcode" alt="Crate docs"/></a>

Idiomatic Rust bindings to the [JAB Code standard implementation](https://github.com/jabcode/jabcode).

## Usage

```rust
let image = jabcode::write_jabcode(
	"Hello world".as_bytes(),
	&jabcode::WriteOptions::default()).unwrap();

let buf = jabcode::read_jabcode(&image).unwrap();
let output = String::from_utf8(buf).unwrap();
assert_eq!("Hello world", output);
```

