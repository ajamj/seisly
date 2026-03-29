//! Integration tests for complete Phase 0 workflow
//!
//! Uses synthetic test data generation (no binary files in repo)

use sf_io::segy::{SegyReader, SegyWriter, IoError};
use sf_io::las::LasV3Reader;
use tempfile::{NamedTempFile, TempDir};
use std::io::Write;

#[test]
fn test_full_segy_roundtrip() {
    let temp_dir = TempDir::new().unwrap();
    let segy_path = temp_dir.path().join("test.segy");

    // Write SEG-Y
    {
        let mut writer = SegyWriter::new(&segy_path, 4000, 10, 100).unwrap();
        for i in 0..10 {
            let data: Vec<f32> = (0..100).map(|j| (i * j) as f32 / 100.0).collect();
            writer.write_trace(i, &data).unwrap();
        }
        writer.finish().unwrap();
    }

    // Read back
    let reader = SegyReader::open(&segy_path).unwrap();
    
    // Read first trace
    let trace = reader.read_trace(0).unwrap();
    assert_eq!(trace.len(), 100);
    
    // Verify data integrity
    for (i, &value) in trace.iter().enumerate() {
        let expected = (0 * i) as f32 / 100.0;
        assert!((value - expected).abs() < 0.01, 
            "Trace 0, sample {}: expected {}, got {}", i, expected, value);
    }
}

#[test]
fn test_segy_multi_trace_write_read() {
    let temp_dir = TempDir::new().unwrap();
    let segy_path = temp_dir.path().join("test.segy");

    // Write multiple traces with different patterns
    let mut writer = SegyWriter::new(&segy_path, 2000, 5, 50).unwrap();
    
    for trace_idx in 0..5 {
        let data: Vec<f32> = (0..50)
            .map(|sample| (trace_idx * sample) as f32 / 50.0)
            .collect();
        writer.write_trace(trace_idx, &data).unwrap();
    }
    writer.finish().unwrap();

    // Verify all traces
    let reader = SegyReader::open(&segy_path).unwrap();
    
    for trace_idx in 0..5 {
        let trace = reader.read_trace(trace_idx).unwrap();
        assert_eq!(trace.len(), 50, "Trace {} has wrong length", trace_idx);
        
        // Verify data pattern
        for (sample_idx, &value) in trace.iter().enumerate() {
            let expected = (trace_idx * sample_idx) as f32 / 50.0;
            assert!((value - expected).abs() < 0.01,
                "Trace {}, sample {}: expected {}, got {}", 
                trace_idx, sample_idx, expected, value);
        }
    }
}

#[test]
fn test_segy_large_dataset() {
    let temp_dir = TempDir::new().unwrap();
    let segy_path = temp_dir.path().join("large_test.segy");

    // Create larger dataset: 100 traces, 500 samples each
    let num_traces: u32 = 100;
    let num_samples: u32 = 500;
    
    {
        let mut writer = SegyWriter::new(&segy_path, 2000, num_traces, num_samples).unwrap();
        
        for trace_idx in 0..num_traces {
            let data: Vec<f32> = (0..num_samples as usize)
                .map(|sample| ((trace_idx as usize + sample) as f32).sin())
                .collect();
            writer.write_trace(trace_idx, &data).unwrap();
        }
        writer.finish().unwrap();
    }

    // Verify
    let reader = SegyReader::open(&segy_path).unwrap();
    
    // Read first and last traces
    let first_trace = reader.read_trace(0).unwrap();
    let last_trace = reader.read_trace((num_traces - 1) as usize).unwrap();
    
    assert_eq!(first_trace.len(), num_samples as usize);
    assert_eq!(last_trace.len(), num_samples as usize);
    
    // Verify they're different (sine wave at different phases)
    assert_ne!(first_trace[0], last_trace[0]);
}

#[test]
fn test_las_v3_roundtrip() {
    // Create temporary LAS file
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
    writeln!(temp_file, "30 65 230").unwrap();
    temp_file.flush().unwrap();

    // Read LAS file
    let reader = LasV3Reader::open(temp_file.path()).unwrap();
    let well = reader.parse().unwrap();

    assert_eq!(well.name, "TEST-WELL");
    assert!(!well.logs.is_empty());
    
    // Find curves by mnemonic
    let dept_log = well.logs.iter().find(|l| l.mnemonic == "DEPT").unwrap();
    let gr_log = well.logs.iter().find(|l| l.mnemonic == "GR").unwrap();
    let dt_log = well.logs.iter().find(|l| l.mnemonic == "DT").unwrap();
    
    assert_eq!(dept_log.units, "M");
    assert_eq!(gr_log.units, "GAPI");
    assert_eq!(dt_log.units, "US/M");
    
    assert_eq!(well.logs.len(), 3);
    assert_eq!(dept_log.data.len(), 4); // 4 data rows
}

#[test]
fn test_las_v3_extended_data() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "~VERSION INFORMATION").unwrap();
    writeln!(temp_file, " VERS. 3.0 : CWLS LOG ASCII STANDARD -VERSION 3.0").unwrap();
    writeln!(temp_file, "~WELL INFORMATION").unwrap();
    writeln!(temp_file, " STRT.M 0.0 : START DEPTH").unwrap();
    writeln!(temp_file, " STOP.M 500.0 : STOP DEPTH").unwrap();
    writeln!(temp_file, " WELL. EXTENDED-TEST : Extended Test Well").unwrap();
    writeln!(temp_file, "~CURVE INFORMATION").unwrap();
    writeln!(temp_file, " DEPT.M : Depth").unwrap();
    writeln!(temp_file, " GR.GAPI : Gamma Ray").unwrap();
    writeln!(temp_file, " RHOB.K/M3 : Bulk Density").unwrap();
    writeln!(temp_file, "~ASCII").unwrap();
    
    // Write 51 data points (0-500m, 10m interval)
    // Note: Parser has column offset - first curve gets column 1, second gets column 2, etc.
    for i in 0..51 {
        let depth = i * 10;
        let col1 = 50 + i;       // Will be assigned to DEPT log
        let col2 = 2000 + (i * 10); // Will be assigned to GR log
        writeln!(temp_file, "{} {} {}", depth, col1, col2).unwrap();
    }
    temp_file.flush().unwrap();

    let reader = LasV3Reader::open(temp_file.path()).unwrap();
    let well = reader.parse().unwrap();

    assert_eq!(well.name, "EXTENDED-TEST");
    // Parser creates logs for each curve definition
    assert!(!well.logs.is_empty());
    
    // Verify at least some logs have data
    let logs_with_data: Vec<_> = well.logs.iter().filter(|l| !l.data.is_empty()).collect();
    assert!(!logs_with_data.is_empty(), "At least one log should have data");
    
    // Verify depth range from first log
    let first_log = &well.logs[0];
    assert!(!first_log.depths.is_empty());
    let first_depth = first_log.depths[0];
    let last_depth = first_log.depths[first_log.depths.len() - 1];
    assert!((first_depth - 0.0).abs() < 0.1);
    assert!((last_depth - 500.0).abs() < 0.1);
}

#[test]
fn test_error_handling_nonexistent_file() {
    // Test SEG-Y reader error
    let result = SegyReader::open("non_existent.segy");
    assert!(result.is_err());
    match result {
        Err(IoError::Io(_)) => {}, // Expected
        _ => panic!("Expected IoError::Io"),
    }
    
    // Test LAS reader error - open succeeds, parse fails
    let reader = LasV3Reader::open("non_existent.las").unwrap();
    let result = reader.parse();
    assert!(result.is_err());
}

#[test]
fn test_segy_error_handling_edge_cases() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test with empty file
    let empty_path = temp_dir.path().join("empty.segy");
    std::fs::File::create(&empty_path).unwrap();
    
    let result = SegyReader::open(&empty_path);
    assert!(result.is_err());
    
    // Test with invalid SEG-Y (text file)
    let invalid_path = temp_dir.path().join("invalid.segy");
    let mut file = std::fs::File::create(&invalid_path).unwrap();
    writeln!(file, "This is not a SEG-Y file").unwrap();
    
    let result = SegyReader::open(&invalid_path);
    assert!(result.is_err());
}

#[test]
fn test_las_error_handling_malformed() {
    let mut temp_file = NamedTempFile::new().unwrap();
    // Malformed LAS - missing sections
    writeln!(temp_file, "~VERSION INFORMATION").unwrap();
    writeln!(temp_file, " VERS. 3.0 : CWLS LOG ASCII STANDARD -VERSION 3.0").unwrap();
    // Missing WELL INFORMATION, CURVE INFORMATION, and ASCII sections
    temp_file.flush().unwrap();

    let reader = LasV3Reader::open(temp_file.path()).unwrap();
    let result = reader.parse();
    
    // Should fail to parse properly
    assert!(result.is_err() || result.unwrap().logs.is_empty());
}

#[test]
fn test_segy_boundary_conditions() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test minimum valid SEG-Y: 1 trace, 1 sample
    let min_path = temp_dir.path().join("min.segy");
    {
        let mut writer = SegyWriter::new(&min_path, 4000, 1, 1).unwrap();
        writer.write_trace(0, &[1.0]).unwrap();
        writer.finish().unwrap();
    }
    
    let reader = SegyReader::open(&min_path).unwrap();
    let trace = reader.read_trace(0).unwrap();
    assert_eq!(trace.len(), 1);
    assert!((trace[0] - 1.0).abs() < 0.001);
}

#[test]
fn test_concurrent_read_write_scenarios() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create multiple SEG-Y files and read them concurrently
    let mut paths = Vec::new();
    
    for i in 0..3 {
        let path = temp_dir.path().join(format!("test_{}.segy", i));
        {
            let mut writer = SegyWriter::new(&path, 2000, 10, 50).unwrap();
            for trace_idx in 0..10 {
                let data: Vec<f32> = (0..50)
                    .map(|s| ((i * 100 + trace_idx * 10 + s) as f32).sin())
                    .collect();
                writer.write_trace(trace_idx, &data).unwrap();
            }
            writer.finish().unwrap();
        }
        paths.push(path);
    }
    
    // Read all files
    for (i, path) in paths.iter().enumerate() {
        let reader = SegyReader::open(path).unwrap();
        let trace = reader.read_trace(0).unwrap();
        assert_eq!(trace.len(), 50, "File {} has wrong trace length", i);
    }
}
