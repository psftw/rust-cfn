extern crate flate2;
extern crate heck;
extern crate itertools;
extern crate json_patch;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use std::io::Read;

mod model;
mod codegen;

fn main() {
    let bytes = include_bytes!("../CloudFormationResourceSpecification.json.gz");
    let mut data = String::new();
    let mut gz = flate2::read::GzDecoder::new(&bytes[..]);
    gz.read_to_string(&mut data)
        .expect("failed to decompress specification file");
    let mut specification = serde_json::from_str::<serde_json::Value>(&data)
        .expect("failed to parse specification data as json");
    let patch = json!({
        "PropertyTypes": {
            "AWS::EC2::LaunchTemplate.CapacityReservationPreference": {
                "Documentation": "https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-capacityreservationpreference.html",
            },
            "AWS::Transfer::User.SshPublicKey": {
                "Documentation": "https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-sshpublickey.html"
            }
        },
    });
    json_patch::merge(&mut specification, &patch);
    let specification = serde_json::from_value::<model::Specification>(specification)
        .expect("failed to parse specification data");
    codegen::generate(specification, "../src/aws")
        .expect("failed to generate output files");
}
