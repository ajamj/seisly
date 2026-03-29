//! SEG-Y writer tests

use sf_io::segy::SegyWriter;
use tempfile::TempDir;

#[test]
fn test_segy_writer_create_and_write() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().join("test.segy");
    
    // Create writer with standard parameters
    let mut writer = SegyWriter::new(
        &temp_path,
        4000, // 4ms sample rate
        10,   // 10 traces
        100,  // 100 samples per trace
    ).unwrap();

    // Write dummy traces
    for i in 0..10 {
        let data: Vec<f32> = (0..100).map(|j| (i as f32) * (j as f32) / 100.0).collect();
        writer.write_trace(i, &data).unwrap();
    }

    writer.finish().unwrap();

    // Verify file can be read back
    use sf_io::segy::SegyReader;
    let reader = SegyReader::open(&temp_path).unwrap();
    assert_eq!(reader.trace_count(), 10);
    
    let trace = reader.read_trace(0).unwrap();
    assert_eq!(trace.len(), 100);
}

#[test]
fn test_segy_writer_error_on_invalid_index() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().join("test.segy");
    
    let mut writer = SegyWriter::new(
        &temp_path,
        4000,
        10,
        100,
    ).unwrap();

    // Try to write trace out of bounds
    let data: Vec<f32> = vec![0.0; 100];
    let result = writer.write_trace(15, &data); // Index 15 > 10 traces
    assert!(result.is_err());
}
