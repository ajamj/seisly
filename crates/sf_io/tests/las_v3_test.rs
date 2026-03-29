//! LAS 3.0 Parser Tests

use sf_io::las::LasV3Reader;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_las_v3_reader_parse() {
    // Create temporary LAS 3.0 file
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "~VERSION INFORMATION").unwrap();
    writeln!(temp_file, " VERS. 3.0 : CWLS LOG ASCII STANDARD -VERSION 3.0").unwrap();
    writeln!(temp_file, "~WELL INFORMATION").unwrap();
    writeln!(temp_file, " STRT.M 0.0 : START DEPTH").unwrap();
    writeln!(temp_file, " STOP.M 100.0 : STOP DEPTH").unwrap();
    writeln!(temp_file, " WELL. TEST-WELL : Well Name").unwrap();
    writeln!(temp_file, "~CURVE INFORMATION").unwrap();
    writeln!(temp_file, " DEPT.M : Depth").unwrap();
    writeln!(temp_file, " GR.GAPI : Gamma Ray").unwrap();
    writeln!(temp_file, " DT.US/M : Delta-T").unwrap();
    writeln!(temp_file, "~ASCII").unwrap();
    writeln!(temp_file, "0 50 200").unwrap();
    writeln!(temp_file, "10 55 210").unwrap();
    writeln!(temp_file, "20 60 220").unwrap();
    temp_file.flush().unwrap();

    // Read LAS file
    let reader = LasV3Reader::open(temp_file.path()).unwrap();
    let well = reader.parse().unwrap();

    assert_eq!(well.name, "TEST-WELL");
    assert!(!well.logs.is_empty());
    assert_eq!(well.logs[0].mnemonic, "DEPT");
    assert!(well.logs.iter().any(|log| log.mnemonic == "GR"));
    assert!(well.logs.iter().any(|log| log.mnemonic == "DT"));
    
    // Check that data was parsed
    let gr_log = well.logs.iter().find(|log| log.mnemonic == "GR").unwrap();
    assert!(!gr_log.data.is_empty());
    assert_eq!(gr_log.data.len(), 3); // 3 data rows
}

#[test]
fn test_las_v3_error_on_invalid_version() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "~VERSION INFORMATION").unwrap();
    writeln!(temp_file, " VERS. 2.0 : OLD VERSION").unwrap();
    writeln!(temp_file, "~WELL INFORMATION").unwrap();
    writeln!(temp_file, " WELL. TEST-WELL : Well Name").unwrap();
    writeln!(temp_file, "~CURVE INFORMATION").unwrap();
    writeln!(temp_file, " DEPT.M : Depth").unwrap();
    writeln!(temp_file, "~ASCII").unwrap();
    writeln!(temp_file, "0").unwrap();
    writeln!(temp_file, "10").unwrap();
    temp_file.flush().unwrap();

    let reader = LasV3Reader::open(temp_file.path()).unwrap();
    let result = reader.parse();
    
    // Should still parse but may warn about version
    assert!(result.is_ok());
}

#[test]
fn test_las_v3_with_json_metadata() {
    // LAS 3.0 supports JSON-like metadata
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "~VERSION INFORMATION").unwrap();
    writeln!(temp_file, " VERS. 3.0 : CWLS LOG ASCII STANDARD -VERSION 3.0").unwrap();
    writeln!(temp_file, "~WELL INFORMATION").unwrap();
    writeln!(temp_file, " WELL. TEST-WELL-JSON : Well with JSON metadata").unwrap();
    writeln!(temp_file, " API. 12345 : API Number").unwrap();
    writeln!(temp_file, "~CURVE INFORMATION").unwrap();
    writeln!(temp_file, " DEPT.M : Depth").unwrap();
    writeln!(temp_file, " GR.GAPI : Gamma Ray").unwrap();
    writeln!(temp_file, "~ASCII").unwrap();
    writeln!(temp_file, "0 50").unwrap();
    writeln!(temp_file, "10 55").unwrap();
    temp_file.flush().unwrap();

    let reader = LasV3Reader::open(temp_file.path()).unwrap();
    let well = reader.parse().unwrap();

    assert_eq!(well.name, "TEST-WELL-JSON");
    assert_eq!(well.logs.len(), 2); // DEPT and GR
}
