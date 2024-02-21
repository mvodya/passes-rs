use passes::package::Package;
use passes::pass::{PassBuilder, PassConfig};

use std::fs::File;
use std::path::Path;

fn main() {
    // Creating pass
    let pass = PassBuilder::new(PassConfig {
        organization_name: "Apple inc.".into(),
        description: "Example pass".into(),
        pass_type_identifier: "com.example.pass".into(),
        team_identifier: "AA00AA0A0A".into(),
        serial_number: "ABCDEFG1234567890".into(),
    })
    .logo_text("Test pass".into())
    .build();

    // Display pass.json
    let json = pass.make_json().unwrap();
    println!("pass.json: {}", json);

    // Creating package
    let package = Package::new(pass);

    // Save package as .pkpass
    let path = Path::new("test_pass.pkpass");
    let file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };
    package.write(file).unwrap();
}
