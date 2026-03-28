//! Content-addressed blob store using BLAKE3 hashing

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlobError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Blob not found: {0}")]
    NotFound(String),
    #[error("Hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },
}

/// Content-addressed blob store
pub struct BlobStore {
    base_path: PathBuf,
}

impl BlobStore {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    /// Store content and return BLAKE3 hash
    pub fn store(&self, content: &[u8]) -> Result<String, BlobError> {
        let hash = blake3::hash(content);
        let hash_str = hash.to_string();

        // Create subdirectory structure (ab/cd/<hash>)
        let subdir = self.base_path.join(&hash_str[0..2]).join(&hash_str[2..4]);
        fs::create_dir_all(&subdir)?;

        let file_path = subdir.join(&hash_str);
        let mut file = File::create(&file_path)?;
        file.write_all(content)?;

        Ok(hash_str)
    }

    /// Retrieve content by hash
    pub fn retrieve(&self, hash: &str) -> Result<Vec<u8>, BlobError> {
        let file_path = self.get_blob_path(hash);

        if !file_path.exists() {
            return Err(BlobError::NotFound(hash.to_string()));
        }

        let mut file = File::open(&file_path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        // Verify hash
        let computed_hash = blake3::hash(&content).to_string();
        if computed_hash != hash {
            return Err(BlobError::HashMismatch {
                expected: hash.to_string(),
                actual: computed_hash,
            });
        }

        Ok(content)
    }

    /// Check if blob exists
    pub fn exists(&self, hash: &str) -> bool {
        self.get_blob_path(hash).exists()
    }

    /// Get the file path for a blob
    fn get_blob_path(&self, hash: &str) -> PathBuf {
        if hash.len() < 4 {
            return self.base_path.join(hash);
        }
        self.base_path
            .join(&hash[0..2])
            .join(&hash[2..4])
            .join(hash)
    }

    /// Get the size of a blob
    pub fn size(&self, hash: &str) -> Result<u64, BlobError> {
        let file_path = self.get_blob_path(hash);
        if !file_path.exists() {
            return Err(BlobError::NotFound(hash.to_string()));
        }
        let metadata = fs::metadata(&file_path)?;
        Ok(metadata.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_blob_store_roundtrip() {
        let temp_dir = TempDir::new().unwrap();
        let store = BlobStore::new(temp_dir.path().to_path_buf());

        let content = b"Hello, StrataForge!";
        let hash = store.store(content).unwrap();

        let retrieved = store.retrieve(&hash).unwrap();
        assert_eq!(content, retrieved.as_slice());
        assert_eq!(hash.len(), 64); // BLAKE3 produces 64-char hex string
    }

    #[test]
    fn test_blob_exists() {
        let temp_dir = TempDir::new().unwrap();
        let store = BlobStore::new(temp_dir.path().to_path_buf());

        let content = b"Test content";
        let hash = store.store(content).unwrap();

        assert!(store.exists(&hash));
        assert!(!store.exists("nonexistent"));
    }

    #[test]
    fn test_blob_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let store = BlobStore::new(temp_dir.path().to_path_buf());

        let result = store.retrieve("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_blob_size() {
        let temp_dir = TempDir::new().unwrap();
        let store = BlobStore::new(temp_dir.path().to_path_buf());

        let content = b"Test content for size";
        let hash = store.store(content).unwrap();

        let size = store.size(&hash).unwrap();
        assert_eq!(size, content.len() as u64);
    }
}
