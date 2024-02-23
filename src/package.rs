use std::io::{Seek, Write};

use crate::pass::Pass;

use self::resource::Resource;

pub mod resource;

/// Pass Package, contains information about pass.json, images, manifest.json and signature.
pub struct Package {
    /// Represents pass.json
    pub pass: Pass,

    /// Resources (image files)
    pub resources: Vec<Resource>,
    // TODO: signature
}

impl Package {
    /// Create new package
    pub fn new(pass: Pass) -> Self {
        Self {
            pass,
            resources: vec![],
        }
    }

    /// Read package from file
    pub fn read_file(_path: String) -> Self {
        todo!(".pkpass reader is not implemented yet!")
    }

    /// Sign package with Apple Developer certificate
    pub fn sign(_cert: String) -> Self {
        todo!("signing is not implemented yet!")
    }

    /// Write compressed package.
    /// Use for creating .pkpass file
    pub fn write<W: Write + Seek>(&self, writer: W) -> Result<(), &'static str> {
        let mut zip = zip::ZipWriter::new(writer);
        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        // Adding pass.json to zip
        zip.start_file("pass.json", options)
            .expect("Error while creating pass.json in zip");
        let pass_json = self
            .pass
            .make_json()
            .expect("Error while building pass.json");
        zip.write(pass_json.as_bytes())
            .expect("Error while writing pass.json in zip");

        zip.finish().expect("Error while saving zip");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use crate::pass::{PassBuilder, PassConfig};

    use super::*;

    #[test]
    fn make_package() {
        let pass = PassBuilder::new(PassConfig {
            organization_name: "Apple inc.".into(),
            description: "Example pass".into(),
            pass_type_identifier: "com.example.pass".into(),
            team_identifier: "AA00AA0A0A".into(),
            serial_number: "ABCDEFG1234567890".into(),
        })
        .logo_text("Test pass".into())
        .build();

        let _package = Package::new(pass);
    }

    #[test]
    fn write_package() {
        let pass = PassBuilder::new(PassConfig {
            organization_name: "Apple inc.".into(),
            description: "Example pass".into(),
            pass_type_identifier: "com.example.pass".into(),
            team_identifier: "AA00AA0A0A".into(),
            serial_number: "ABCDEFG1234567890".into(),
        })
        .logo_text("Test pass".into())
        .build();

        let expected_pass_json = pass.make_json().unwrap();

        let package = Package::new(pass);

        // Save package as .pkpass
        let mut buf = [0; 65536];
        let writer = std::io::Cursor::new(&mut buf[..]);
        package.write(writer).unwrap();

        // Read .pkpass as zip
        let reader = std::io::Cursor::new(&mut buf[..]);
        let mut zip = zip::ZipArchive::new(reader).unwrap();

        for i in 0..zip.len() {
            let file = zip.by_index(i).unwrap();
            println!("file[{}]: {}", i, file.name());
        }

        // Get pass.json and compare
        let mut packaged_pass_json = String::new();
        let _ = zip
            .by_name("pass.json")
            .unwrap()
            .read_to_string(&mut packaged_pass_json);

        assert_eq!(expected_pass_json, packaged_pass_json);
    }
}
