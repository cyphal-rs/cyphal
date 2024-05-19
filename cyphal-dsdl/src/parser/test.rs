use crate::{DsdlResult, Parser, Statement};
use std::{fs::File, io::BufReader, path::PathBuf};

#[test]
#[ignore = "not implemented"]
fn test_7509_heartbeat_1_0() {
    let dsdl = "test/assets/public_regulated_data_types/uavcan/node/7509.Heartbeat.1.0.dsdl";

    let result = parse_dsdl(dsdl);
    assert!(result.is_ok());

    let statements = result.unwrap();
    assert_eq!(statements.len(), 37);
}

#[test]
fn test_health_1_0() {
    let dsdl = "test/assets/public_regulated_data_types/uavcan/node/Health.1.0.dsdl";

    let result = parse_dsdl(dsdl);
    assert!(result.is_ok());

    let statements = result.unwrap();
    assert_eq!(statements.len(), 22);
}

#[test]
fn test_mode_1_0() {
    let dsdl = "test/assets/public_regulated_data_types/uavcan/node/Mode.1.0.dsdl";
    let result = parse_dsdl(dsdl);

    assert!(result.is_ok());

    let statements = result.unwrap();
    assert_eq!(statements.len(), 18);
}

#[test]
fn test_version_1_0() {
    let dsdl = "test/assets/public_regulated_data_types/uavcan/node/Version.1.0.dsdl";
    let result = parse_dsdl(dsdl);

    assert!(result.is_ok());

    let statements = result.unwrap();
    assert_eq!(statements.len(), 7);
}

fn parse_dsdl(dsdl: &str) -> DsdlResult<Vec<Statement>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(dsdl);

    let file = File::open(path).expect("Could not open file");
    let mut reader = BufReader::new(file);

    let parser = Parser::new().expect("Could not construct parser");
    parser.read_dsdl(&mut reader)
}
