use crate::pass::Pass;

/// Pass Package, contains information about pass.json, images, manifest.json and signature.
pub struct Package {
    /// Represents pass.json
    pub pass: Pass,
    // TODO: signature
    // TODO: resources
}

impl Package {
    /// Create new package
    pub fn new(pass: Pass) -> Self {
        Self { pass }
    }

    /// Read package from file
    pub fn read_file(_path: String) -> Self {
        todo!(".pkpass reader is not implemented yet!")
    }

    /// Sign package with Apple Developer certificate
    pub fn sign(_cert: String) -> Self {
        todo!("signing is not implemented yet!")
    }

    /// Save package as .pkpass
    pub fn save_file(_path: String) {
        todo!(".pkpass save is not implemented yet!")
    }
}

#[cfg(test)]
mod tests {
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
}
