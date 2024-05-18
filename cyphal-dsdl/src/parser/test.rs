use crate::{Directive, DsdlResult, Parser, Primitive, Statement, UintPrimitive};
use std::{fs::File, io::BufReader, path::PathBuf};

#[test]
#[ignore = "not implemented"]
fn test_7509_heartbeat_1_0() {
    let dsdl = "assets/7509.Heartbeat.1.0.dsdl";

    let result = parse_dsdl(dsdl);
    assert!(result.is_ok());

    let statements = result.unwrap();
    assert_eq!(statements.len(), 37);
}

#[test]
fn test_health_1_0() {
    let dsdl = "assets/Health.1.0.dsdl";

    let result = parse_dsdl(dsdl);
    assert!(result.is_ok());

    let statements = result.unwrap();
    assert_eq!(statements.len(), 22);
    assert_eq!(statements[0], Statement::Comment(" Abstract component health information. If the node performs multiple activities (provides multiple network services),".to_string()));
    assert_eq!(statements[1], Statement::Comment(" its health status should reflect the status of the worst-performing activity (network service).".to_string()));
    assert_eq!(statements[2], Statement::Comment(" Follows:".to_string()));
    assert_eq!(
        statements[3],
        Statement::Comment("   https://www.law.cornell.edu/cfr/text/14/23.1322".to_string())
    );
    assert_eq!(statements[4], Statement::Comment("   https://www.faa.gov/documentLibrary/media/Advisory_Circular/AC_25.1322-1.pdf section 6".to_string()));
    assert_eq!(statements[5], Statement::Empty);
    assert_eq!(
        statements[6],
        Statement::Primitive(Primitive::Uint(
            UintPrimitive::new(2, "value".to_string(), None, None).unwrap()
        )),
    );
    assert_eq!(statements[7], Statement::Empty);
    assert_eq!(
        statements[8],
        Statement::Primitive(Primitive::Uint(
            UintPrimitive::new(2, "NOMINAL".to_string(), Some(0), None).unwrap()
        )),
    );
    assert_eq!(
        statements[9],
        Statement::Comment(" The component is functioning properly (nominal).".to_string())
    );
    assert_eq!(statements[10], Statement::Empty);
    assert_eq!(
        statements[11],
        Statement::Primitive(Primitive::Uint(
            UintPrimitive::new(2, "ADVISORY".to_string(), Some(1), None).unwrap()
        )),
    );
    assert_eq!(statements[12], Statement::Comment(" A critical parameter went out of range or the component encountered a minor failure that does not prevent".to_string()));
    assert_eq!(
        statements[13],
        Statement::Comment(
            " the subsystem from performing any of its real-time functions.".to_string()
        )
    );
    assert_eq!(statements[14], Statement::Empty);
    assert_eq!(
        statements[15],
        Statement::Primitive(Primitive::Uint(
            UintPrimitive::new(2, "CAUTION".to_string(), Some(2), None).unwrap()
        )),
    );
    assert_eq!(statements[16], Statement::Comment(" The component encountered a major failure and is performing in a degraded mode or outside of its designed limitations.".to_string()));
    assert_eq!(statements[17], Statement::Empty);
    assert_eq!(
        statements[18],
        Statement::Primitive(Primitive::Uint(
            UintPrimitive::new(2, "WARNING".to_string(), Some(3), None).unwrap()
        )),
    );
    assert_eq!(statements[19], Statement::Comment(" The component suffered a fatal malfunction and is unable to perform its intended function.".to_string()));
    assert_eq!(statements[20], Statement::Empty);
    assert_eq!(
        statements[21],
        Statement::Directive(Directive::Sealed(None))
    );
}

#[test]
fn test_mode_1_0() {
    let dsdl = "assets/Mode.1.0.dsdl";
    let result = parse_dsdl(dsdl);

    assert!(result.is_ok());

    let statements = result.unwrap();
    assert_eq!(statements.len(), 18);

    assert_eq!(
        statements[0],
        Statement::Comment(" The operating mode of a node.".to_string())
    );
    assert_eq!(
        statements[1],
        Statement::Comment(
            " Reserved values can be used in future revisions of the specification.".to_string()
        )
    );
    assert_eq!(statements[2], Statement::Empty);
    assert_eq!(
        statements[3],
        Statement::Primitive(Primitive::Uint(
            UintPrimitive::new(3, "value".to_string(), None, None).unwrap()
        )),
    );
    assert_eq!(statements[4], Statement::Empty);
    assert_eq!(
        statements[5],
        Statement::Primitive(Primitive::Uint(
            UintPrimitive::new(3, "OPERATIONAL".to_string(), Some(0), None).unwrap()
        )),
    );
    assert_eq!(
        statements[6],
        Statement::Comment(" Normal operating mode.".to_string())
    );
    assert_eq!(statements[7], Statement::Empty);
    assert_eq!(
        statements[8],
        Statement::Primitive(Primitive::Uint(
            UintPrimitive::new(3, "INITIALIZATION".to_string(), Some(1), None).unwrap()
        )),
    );
    assert_eq!(
        statements[9],
        Statement::Comment(
            " Initialization is in progress; this mode is entered immediately after startup."
                .to_string()
        )
    );
    assert_eq!(statements[10], Statement::Empty);
    assert_eq!(
        statements[11],
        Statement::Primitive(Primitive::Uint(
            UintPrimitive::new(3, "MAINTENANCE".to_string(), Some(2), None).unwrap()
        )),
    );
    assert_eq!(
        statements[12],
        Statement::Comment(" E.g., calibration, self-test, etc.".to_string())
    );
    assert_eq!(statements[13], Statement::Empty);
    assert_eq!(
        statements[14],
        Statement::Primitive(Primitive::Uint(
            UintPrimitive::new(3, "SOFTWARE_UPDATE".to_string(), Some(3), None).unwrap()
        )),
    );
    assert_eq!(
        statements[15],
        Statement::Comment(
            " New software/firmware is being loaded or the bootloader is running.".to_string()
        )
    );
}

#[test]
fn test_version_1_0() {
    let dsdl = "assets/Version.1.0.dsdl";
    let result = parse_dsdl(dsdl);

    assert!(result.is_ok());

    let statements = result.unwrap();
    assert_eq!(statements.len(), 7);

    assert_eq!(
        statements[0],
        Statement::Comment(
            " A shortened semantic version representation: only major and minor.".to_string()
        )
    );
    assert_eq!(
        statements[1],
        Statement::Comment(
            " The protocol generally does not concern itself with the patch version.".to_string()
        )
    );
    assert_eq!(statements[2], Statement::Empty);
    assert_eq!(
        statements[3],
        Statement::Primitive(Primitive::Uint(
            UintPrimitive::new(8, "major".to_string(), None, None).unwrap()
        )),
    );
    assert_eq!(
        statements[4],
        Statement::Primitive(Primitive::Uint(
            UintPrimitive::new(8, "minor".to_string(), None, None).unwrap()
        )),
    );
    assert_eq!(statements[5], Statement::Empty);
    assert_eq!(statements[6], Statement::Directive(Directive::Sealed(None)));
}

fn parse_dsdl(dsdl: &str) -> DsdlResult<Vec<Statement>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(dsdl);

    let file = File::open(path).expect("Could not open file");
    let mut reader = BufReader::new(file);

    let parser = Parser::new().expect("Could not construct parser");
    parser.read_dsdl(&mut reader)
}
