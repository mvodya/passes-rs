use passes::package::resource;
use passes::package::sign::SignConfig;
use passes::package::Package;
use passes::pass::barcode::{Barcode, BarcodeFormat};
use passes::pass::fields::{self, Content, ContentOptions};
use passes::pass::semantic_tags::{SemanticTagLocation, SemanticTagSeat, SemanticTags};
use passes::pass::{PassBuilder, PassConfig};

use chrono::prelude::*;

use std::fs::File;
use std::path::Path;

fn main() {
    // Creating pass
    let pass = PassBuilder::new(PassConfig {
        organization_name: "Dodo Airlines".into(),
        description: "DAL Boarding Pass".into(),
        pass_type_identifier: "com.example.pass".into(),
        team_identifier: "AA00AA0A0A".into(),
        serial_number: "ABCDEFG1234567890".into(),
    })
    .appearance(passes::pass::visual_appearance::VisualAppearance {
        label_color: passes::pass::visual_appearance::Color::white(),
        foreground_color: passes::pass::visual_appearance::Color::white(),
        background_color: passes::pass::visual_appearance::Color::new(0, 143, 212),
    })
    .fields(
        fields::Type::BoardingPass {
            pass_fields: fields::Fields {
                ..Default::default()
            },
            transit_type: fields::TransitType::Air,
        }
        .add_primary_field(Content::new(
            "from",
            "OAK",
            ContentOptions {
                label: String::from("Oak island").into(),
                ..Default::default()
            },
        ))
        .add_primary_field(Content::new(
            "to",
            "MVK",
            ContentOptions {
                label: String::from("Маврикий").into(),
                ..Default::default()
            },
        ))
        .add_auxiliary_field(Content::new(
            "seq",
            "457",
            ContentOptions {
                label: String::from("seq").into(),
                ..Default::default()
            },
        ))
        .add_auxiliary_field(Content::new(
            "boards",
            "18:46",
            ContentOptions {
                label: String::from("scheduled").into(),
                ..Default::default()
            },
        ))
        .add_auxiliary_field(Content::new(
            "seat",
            "20A",
            ContentOptions {
                label: String::from("seat").into(),
                ..Default::default()
            },
        ))
        .add_auxiliary_field(Content::new(
            "group",
            "A",
            ContentOptions {
                label: String::from("group").into(),
                ..Default::default()
            },
        ))
        .add_secondary_field(Content::new(
            "passenger",
            "John Cena",
            ContentOptions {
                label: String::from("passenger").into(),
                ..Default::default()
            },
        ))
        .add_header_field(fields::Content::new(
            "gate",
            "21",
            fields::ContentOptions {
                label: String::from("gate").into(),
                ..Default::default()
            },
        ))
        .add_header_field(fields::Content::new(
            "flight",
            "DL 1132",
            fields::ContentOptions {
                label: String::from("flight").into(),
                ..Default::default()
            },
        ))
        .add_back_field(fields::Content::new(
            "about",
            "This is test boarding pass for Dodo Airlines",
            fields::ContentOptions {
                label: String::from("About").into(),
                ..Default::default()
            },
        ))
        .add_back_field(fields::Content::new(
            "githubURL",
            "https://github.com/mvodya/passes-rs",
            fields::ContentOptions {
                label: String::from("Github").into(),
                ..Default::default()
            },
        )),
    )
    .relevant_date(Utc.with_ymd_and_hms(2024, 02, 28, 0, 0, 0).unwrap())
    .expiration_date(Utc.with_ymd_and_hms(2024, 02, 29, 0, 0, 0).unwrap())
    .semantics(SemanticTags {
        airline_code: String::from("DL 1132").into(),
        departure_gate: String::from("21").into(),
        departure_location: SemanticTagLocation {
            latitude: 43.3948533,
            longitude: 132.1451673,
        }
        .into(),
        original_boarding_date: Utc
            .with_ymd_and_hms(2024, 02, 29, 18, 46, 0)
            .unwrap()
            .into(),
        original_departure_date: Utc
            .with_ymd_and_hms(2024, 02, 29, 18, 46, 0)
            .unwrap()
            .into(),
        original_arrival_date: Utc
            .with_ymd_and_hms(2024, 02, 29, 23, 20, 0)
            .unwrap()
            .into(),
        seats: vec![SemanticTagSeat {
            seat_identifier: String::from("20A").into(),
            seat_number: String::from("A").into(),
            seat_row: String::from("20").into(),
            seat_type: String::from("econom").into(),
            ..Default::default()
        }],
        ..Default::default()
    })
    .add_barcode(Barcode {
        message: String::from("01230011223344//Dodo//Airlines//econom//20A"),
        format: BarcodeFormat::PDF417,
        ..Default::default()
    })
    .build();

    // Display pass.json
    let json = pass.make_json().unwrap();
    println!("pass.json: {}", json);

    // Creating package
    let mut package = Package::new(pass);

    // Adding icon
    let image_path = Path::new("DAL_logo.png");
    let file = match File::open(&image_path) {
        Err(why) => panic!("couldn't open {}: {}", image_path.display(), why),
        Ok(file) => file,
    };
    package
        .add_resource(resource::Type::Icon(resource::Version::Size2X), file)
        .unwrap();

    // Adding logo
    let image_path = Path::new("DAL_logo_text.png");
    let file = match File::open(&image_path) {
        Err(why) => panic!("couldn't open {}: {}", image_path.display(), why),
        Ok(file) => file,
    };
    package
        .add_resource(resource::Type::Logo(resource::Version::Size2X), file)
        .unwrap();

    // Add certificates
    let sign_cert_path = Path::new("certs/signerCert.pem");
    let mut file_sign_cert = match File::open(&sign_cert_path) {
        Err(why) => panic!("couldn't open {}: {}", sign_cert_path.display(), why),
        Ok(file) => file,
    };
    let mut sign_cert_data = Vec::new();
    std::io::Read::read_to_end(&mut file_sign_cert, &mut sign_cert_data).unwrap();

    let sign_cert_key_path = Path::new("certs/signerKey.key");
    let mut file_sign_key_cert = match File::open(&sign_cert_key_path) {
        Err(why) => panic!("couldn't open {}: {}", sign_cert_key_path.display(), why),
        Ok(file) => file,
    };
    let mut sign_cert_key_data = Vec::new();
    std::io::Read::read_to_end(&mut file_sign_key_cert, &mut sign_cert_key_data).unwrap();

    let sign_config = SignConfig::new(
        passes::package::sign::WWDR::G4,
        &sign_cert_data,
        &sign_cert_key_data,
    )
    .unwrap();
    package.add_certificates(sign_config);

    // Save package as .pkpass
    let path = Path::new("DAL-boardingpass.pkpass");
    let file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };
    package.write(file).unwrap();
}
