use passes::package::Package;
use passes::pass::{PassBuilder, PassConfig};

fn main() {
    let pass = PassBuilder::new(PassConfig {
        organization_name: "Apple inc.".into(),
        description: "Example pass".into(),
        pass_type_identifier: "com.example.pass".into(),
        team_identifier: "AA00AA0A0A".into(),
        serial_number: "ABCDEFG1234567890".into(),
    })
    .logo_text("Test pass".into())
    .build();

    let package = Package::new(pass);
    let json = package.pass.make_json().unwrap();
    println!("JSON: {}", json);
}
