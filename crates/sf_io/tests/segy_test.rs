//! SEG-Y reader integration tests

use sf_io::segy::{IoError, SegyReader};
use tempfile::TempDir;

#[test]
fn test_segy_reader_open_and_read() {
    // Test with SegyWriter - create a file and read it back
    use sf_io::segy::SegyWriter;
    
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.segy");

    // Create a test file with SegyWriter
    let mut writer = SegyWriter::new(
        &test_file,
        4000, // 4ms sample rate
        5,    // 5 traces
        50,   // 50 samples per trace
    ).unwrap();

    // Write dummy traces
    for i in 0..5 {
        let data: Vec<f32> = (0..50).map(|j| (i as f32) * (j as f32) / 50.0).collect();
        writer.write_trace(i, &data).unwrap();
    }
    writer.finish().unwrap();

    // Now read it back
    let reader = SegyReader::open(&test_file).unwrap();

    // Check binary header is accessible
    let _bin_header = reader.binary_header();

    // Read first trace
    let trace = reader.read_trace(0).unwrap();
    assert_eq!(trace.len(), 50);
}

#[test]
fn test_segy_reader_error_handling() {
    // Test error on non-existent file
    let result = SegyReader::open("non_existent_file.segy");
    assert!(result.is_err());

    // Verify it's an Io error
    match result {
        Err(IoError::Io(_)) => {}, // Expected
        Err(IoError::ParseError(_)) => panic!("Expected IO error, got ParseError"),
        Ok(_) => panic!("Expected error, got Ok"),
    }
}
