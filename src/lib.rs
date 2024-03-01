//! Rust library for generate Apple Wallet Passes for iOS, WatchOS, MacOS.
//!
//! # Quick start
//! [Pass](Pass) represent the displayable fields (pass.json), [Package](Package)
//! includes [Pass](Pass) and [Resource's](Resource) (images, such as logo, background, etc.) and
//! represent pass package (.pkpass file).
//!
//! Before making [Package](Package), you need to create [Pass](Pass) structure, then include pass (and required resources) to
//! package.
//!
//! ```
//! use passes::{resource, sign, Package, PassBuilder, PassConfig};
//! use std::{fs::File, path::Path};
//!
//! // Creating a pass
//! let pass = PassBuilder::new(PassConfig {
//!     organization_name: "Test organization".into(),
//!     description: "Test description for pass".into(),
//!     pass_type_identifier: "com.example.pass".into(),
//!     team_identifier: "AA00AA0A0A".into(),
//!     serial_number: "ABCDEFG1234567890".into(),
//! })
//! .grouping_identifier(String::from("com.example.pass.app"))
//! .logo_text("Test pass".into())
//! .build();
//!
//! // Creating package
//! let mut package = Package::new(pass);
//!
//! // Add required resource
//! let file = File::open(&Path::new("examples/pass-generator/template_app_icon.png")).unwrap();
//! package
//!     .add_resource(resource::Type::Icon(resource::Version::Standard), file)
//!     .unwrap();
//!  ```
//!
//! Then you can add certificates for sign pass package. Signing is required for iOS, MacOS and other Apple stuff.
//!
//! ```rust,ignore
//! let mut file_sign_cert = File::open(&Path::new("certs/signerCert.pem"));
//! let mut sign_cert_data = Vec::new();
//! std::io::Read::read_to_end(&mut file_sign_cert, &mut sign_cert_data).unwrap();
//!
//! let mut file_sign_key_cert = File::open(&Path::new("certs/signerKey.key")).unwrap();
//! let mut sign_cert_key_data = Vec::new();
//! std::io::Read::read_to_end(&mut file_sign_key_cert, &mut sign_cert_key_data).unwrap();
//!
//! let sign_config =
//!     sign::SignConfig::new(passes::sign::WWDR::G4, &sign_cert_data, &sign_cert_key_data)
//!         .unwrap();
//! package.add_certificates(sign_config);
//! ```
//!
//! You can save package as .pkpass file:
//!
//! ```rust,ignore
//! // Save package as .pkpass
//! let file = File::create(&Path::new("test_pass.pkpass")).unwrap();
//! package.write(file).unwrap();
//! ```
//!
//! For more examples, see [example directory](https://github.com/mvodya/passes-rs/tree/main/examples) on GitHub.
// Primary modules
mod package;
mod pass;

// Re-exports
pub use self::package::*;
pub use self::pass::*;
