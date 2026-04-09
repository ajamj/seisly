use shared_memory::{Shmem, ShmemConf, ShmemError};
use std::ptr;

/// A high-performance Shared Memory segment for seismic data transfer.
pub struct ShmSegment {
    shmem: Shmem,
}

impl ShmSegment {
    /// Creates a new Shared Memory segment of the specified size.
    pub fn create(size: usize) -> Result<Self, ShmemError> {
        let shmem = ShmemConf::new().size(size).create()?;

        Ok(Self { shmem })
    }

    /// Opens an existing Shared Memory segment by its ID.
    pub fn open(id: &str) -> Result<Self, ShmemError> {
        let shmem = ShmemConf::new().os_id(id).open()?;

        Ok(Self { shmem })
    }

    /// Returns the unique OS identifier for this segment.
    pub fn id(&self) -> &str {
        self.shmem.get_os_id()
    }

    /// Returns the size of the segment.
    pub fn size(&self) -> usize {
        self.shmem.len()
    }

    /// Writes data into the shared memory segment.
    ///
    /// # Panics
    /// Panics if the data slice is larger than the segment size.
    pub fn write_data(&mut self, data: &[u8]) {
        assert!(data.len() <= self.shmem.len(), "Data too large for segment");
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr(), self.shmem.as_ptr(), data.len());
        }
    }

    /// Reads data from the shared memory segment into the provided buffer.
    ///
    /// # Panics
    /// Panics if the buffer is larger than the segment size.
    pub fn read_data(&self, buffer: &mut [u8]) {
        assert!(
            buffer.len() <= self.shmem.len(),
            "Buffer too large for segment"
        );
        unsafe {
            ptr::copy_nonoverlapping(self.shmem.as_ptr(), buffer.as_mut_ptr(), buffer.len());
        }
    }

    /// Returns a pointer to the start of the shared memory.
    pub fn as_ptr(&self) -> *const u8 {
        self.shmem.as_ptr()
    }

    /// Returns a mutable pointer to the start of the shared memory.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.shmem.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_write() {
        let mut seg = ShmSegment::create(1024).unwrap();
        let data = b"Hello, Shared Memory!";
        seg.write_data(data);

        let mut buffer = vec![0u8; data.len()];
        seg.read_data(&mut buffer);
        assert_eq!(data, buffer.as_slice());
    }

    #[test]
    fn test_open_existing() {
        let mut seg1 = ShmSegment::create(1024).unwrap();
        let id = seg1.id().to_string();
        let data = b"Cross-process simulation";
        seg1.write_data(data);

        let seg2 = ShmSegment::open(&id).unwrap();
        let mut buffer = vec![0u8; data.len()];
        seg2.read_data(&mut buffer);
        assert_eq!(data, buffer.as_slice());
    }
}
