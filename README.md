# Passes

A Rust library for generating PassKit passes, featuring:

- Read & parse `.pkpass` files
- Build & make passes by using library API
- Sign passes with certificate and compress to `.pkpass`
- Change field values is pass by key name
- Supported semantic tags for pass & fields
- All features of [Wallet Passes standard](https://developer.apple.com/documentation/walletpasses) represented in library

Documentation:

- [API reference (doc.rs)](https://docs.rs/passes)
- [Examples](https://github.com/mvodya/passes-rs/tree/main/examples)
- [Apple Wallet Documentation](https://developer.apple.com/documentation/walletpasses)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
passes = "0.1.0"
```

## Example

For building simple pass:

```rust
// Creating pass
let pass = PassBuilder::new(PassConfig {
    organization_name: "Test organization".into(),
    description: "Super gentlememe pass".into(),
    pass_type_identifier: "com.example.pass".into(),
    team_identifier: "AA00AA0A0A".into(),
    serial_number: "ABCDEFG1234567890".into(),
})
.grouping_identifier(String::from("com.example.pass.app"))
.logo_text("Test pass".into())
.build();
```

Creating package and generate `.pkpass` file:

```rust
let mut package = Package::new(pass);

// Save package as .pkpass
let path = Path::new("test_pass.pkpass");
let file = match File::create(&path) {
    Err(why) => panic!("couldn't create {}: {}", path.display(), why),
    Ok(file) => file,
};
package.write(file).unwrap();
```

For more examples, see [examples](https://github.com/mvodya/passes-rs/tree/main/examples) directory.

## License

Passes is distributed under the terms of the MIT license. See [LICENSE](LICENSE).
