use passes::{resource, sign, Package, PassBuilder, PassConfig};

use std::fs::File;
use std::path::Path;

fn main() {
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

    // Display pass.json
    let json = pass.make_json().unwrap();
    println!("pass.json: {}", json);

    // Creating package
    let mut package = Package::new(pass);

    // Icon for pass required (!)
    let image_path = Path::new("template_app_icon.png");
    let file = match File::open(&image_path) {
        Err(why) => panic!("couldn't open {}: {}", image_path.display(), why),
        Ok(file) => file,
    };
    package
        .add_resource(resource::Type::Icon(resource::Version::Standard), file)
        .unwrap();

    // Add certificates
    let sign_cert_path = Path::new("certs/signerCert.pem");
    let mut file_sign_cert = match File::open(&sign_cert_path) {
        Err(why) => panic!("couldn't open {}: {}", sign_cert_path.display(), why),
        Ok(file) => file,
    };
    let mut sign_cert_data = Vec::new();
    std::io::Read::read_to_end(&mut file_sign_cert, &mut sign_cert_data).unwrap();

    // Setup certificates
    let sign_cert_key_path = Path::new("certs/signerKey.key");
    let mut file_sign_key_cert = match File::open(&sign_cert_key_path) {
        Err(why) => panic!("couldn't open {}: {}", sign_cert_key_path.display(), why),
        Ok(file) => file,
    };
    let mut sign_cert_key_data = Vec::new();
    std::io::Read::read_to_end(&mut file_sign_key_cert, &mut sign_cert_key_data).unwrap();

    let sign_config =
        sign::SignConfig::new(sign::WWDR::G4, &sign_cert_data, &sign_cert_key_data)
            .unwrap();
    package.add_certificates(sign_config);

    // Save package as .pkpass
    let path = Path::new("test_pass.pkpass");
    let file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };
    package.write(file).unwrap();
}
