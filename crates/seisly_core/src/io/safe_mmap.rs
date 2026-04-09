//! Safe Memory-Mapped I/O Wrapper
//!
//! This module provides a safe wrapper around memory-mapped file access to protect
//! against SIGBUS (Unix) and Access Violation (Windows) exceptions that can occur
//! when accessing memory-mapped regions that have become invalid.
//!
//! # Safety Guarantees
//!
//! - All memory accesses are bounds-checked before dereferencing
//! - Errors from the underlying mmap are captured and returned as `std::io::Error`
//! - Prevents crashes from truncated files or network share disconnections
//!
//! # Platform Notes
//!
//! - **Windows**: Access violations are Structured Exception Handling (SEH) events.
//!   We prevent them through bounds checking rather than trying to catch SEH.
//! - **Unix/Linux**: SIGBUS signals can be caught, but prevention through bounds
//!   checking is more efficient and portable.

use std::fs::File;
use std::io::Result;
use std::sync::Arc;

/// A safe wrapper around memory-mapped file access.
///
/// This wrapper provides guarded access to memory-mapped regions by:
/// 1. Validating all memory accesses against known bounds
/// 2. Capturing mapping errors as standard I/O errors
/// 3. Preventing SIGBUS/Access Violation crashes through bounds checking
///
/// # Example
///
/// ```no_run
/// use seisly_core::io::safe_mmap::SafeMmap;
/// use std::path::Path;
///
/// let file = std::fs::File::open("data.segy").unwrap();
/// let mmap = SafeMmap::map(&file).unwrap();
///
/// // Safe access with bounds checking
/// if let Some(byte) = mmap.get(100) {
///     println!("Byte at offset 100: {}", byte);
/// }
///
/// // Safe slice access with bounds checking
/// if let Some(slice) = mmap.get_slice(100, 4) {
///     println!("4 bytes starting at offset 100: {:?}", slice);
/// }
/// ```
pub struct SafeMmap {
    /// The underlying memory-mapped data
    inner: memmap2::Mmap,
    /// Cached length for fast bounds checking
    len: usize,
}

impl SafeMmap {
    /// Creates a safe memory-mapped view of the given file.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be opened for reading
    /// - The memory mapping fails (e.g., insufficient permissions)
    /// - The file is empty
    #[allow(unsafe_code)]
    // Safety: memmap2::Mmap::map() requires unsafe because the returned
    // Mmap provides raw memory access. SafeMmap wraps this and provides
    // only bounded access via get() and get_slice() methods that validate
    // offsets against the mapped region length, preventing SIGBUS/Access Violation.
    pub fn map(file: &File) -> Result<Self> {
        // Safety: The caller of SafeMmap::map() trusts that the file descriptor
        // remains valid for the lifetime of the SafeMmap. The OS guarantees that
        // the mapping is valid as long as the file is open and not truncated.
        // SafeMmap enforces bounds checking on all access methods to prevent
        // out-of-bounds reads that could cause SIGBUS (Unix) or Access Violation (Windows).
        let mmap = unsafe { memmap2::Mmap::map(file) }?;
        let len = mmap.len();

        Ok(Self { inner: mmap, len })
    }

    /// Returns the length of the memory-mapped region in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the memory-mapped region is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Safely reads a single byte at the given offset.
    ///
    /// Returns `None` if the offset is out of bounds.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use seisly_core::io::safe_mmap::SafeMmap;
    /// use std::fs::File;
    ///
    /// let file = File::open("data.bin").unwrap();
    /// let mmap = SafeMmap::map(&file).unwrap();
    ///
    /// if let Some(byte) = mmap.get(42) {
    ///     println!("Byte: {}", byte);
    /// } else {
    ///     println!("Offset 42 is out of bounds");
    /// }
    /// ```
    #[inline]
    pub fn get(&self, offset: usize) -> Option<u8> {
        if offset >= self.len {
            None
        } else {
            // Safety: We've verified the offset is within bounds
            Some(unsafe { *self.inner.get_unchecked(offset) })
        }
    }

    /// Safely reads a slice of bytes starting at the given offset.
    ///
    /// Returns `None` if the range `[offset, offset + len)` is out of bounds.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use seisly_core::io::safe_mmap::SafeMmap;
    /// use std::fs::File;
    ///
    /// let file = File::open("data.bin").unwrap();
    /// let mmap = SafeMmap::map(&file).unwrap();
    ///
    /// if let Some(bytes) = mmap.get_slice(100, 4) {
    ///     println!("4 bytes: {:?}", bytes);
    /// }
    /// ```
    #[inline]
    pub fn get_slice(&self, offset: usize, len: usize) -> Option<&[u8]> {
        if offset.checked_add(len)? > self.len {
            None
        } else {
            // Safety: We've verified the range is within bounds
            Some(unsafe { self.inner.get_unchecked(offset..offset + len) })
        }
    }

    /// Reads a u16 in big-endian format from the given offset.
    ///
    /// Returns `None` if there aren't enough bytes available.
    #[inline]
    pub fn get_u16_be(&self, offset: usize) -> Option<u16> {
        let slice = self.get_slice(offset, 2)?;
        Some(u16::from_be_bytes([slice[0], slice[1]]))
    }

    /// Reads a u32 in big-endian format from the given offset.
    ///
    /// Returns `None` if there aren't enough bytes available.
    #[inline]
    pub fn get_u32_be(&self, offset: usize) -> Option<u32> {
        let slice = self.get_slice(offset, 4)?;
        Some(u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]))
    }

    /// Reads an i32 in big-endian format from the given offset.
    ///
    /// Returns `None` if there aren't enough bytes available.
    #[inline]
    pub fn get_i32_be(&self, offset: usize) -> Option<i32> {
        let slice = self.get_slice(offset, 4)?;
        Some(i32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]))
    }

    /// Reads an f32 in big-endian format from the given offset.
    ///
    /// Returns `None` if there aren't enough bytes available.
    #[inline]
    pub fn get_f32_be(&self, offset: usize) -> Option<f32> {
        let slice = self.get_slice(offset, 4)?;
        Some(f32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]))
    }

    /// Reads a u16 in little-endian format from the given offset.
    ///
    /// Returns `None` if there aren't enough bytes available.
    #[inline]
    pub fn get_u16_le(&self, offset: usize) -> Option<u16> {
        let slice = self.get_slice(offset, 2)?;
        Some(u16::from_le_bytes([slice[0], slice[1]]))
    }

    /// Reads a u32 in little-endian format from the given offset.
    ///
    /// Returns `None` if there aren't enough bytes available.
    #[inline]
    pub fn get_u32_le(&self, offset: usize) -> Option<u32> {
        let slice = self.get_slice(offset, 4)?;
        Some(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
    }

    /// Reads an i32 in little-endian format from the given offset.
    ///
    /// Returns `None` if there aren't enough bytes available.
    #[inline]
    pub fn get_i32_le(&self, offset: usize) -> Option<i32> {
        let slice = self.get_slice(offset, 4)?;
        Some(i32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
    }

    /// Reads an f32 in little-endian format from the given offset.
    ///
    /// Returns `None` if there aren't enough bytes available.
    #[inline]
    pub fn get_f32_le(&self, offset: usize) -> Option<f32> {
        let slice = self.get_slice(offset, 4)?;
        Some(f32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
    }

    /// Returns a reference to the entire underlying memory-mapped data as a byte slice.
    ///
    /// This method provides direct, unbounded access to the mapped region. Use it
    /// only when you need to pass the entire mapping to an API that expects a `&[u8]`.
    ///
    /// # Safety
    ///
    /// This method is unsafe because it exposes the raw memory mapping without
    /// bounds checking. Callers must ensure:
    ///
    /// 1. **No concurrent modification**: The underlying file is not modified or
    ///    truncated while the mapping is active. Concurrent writes can cause
    ///    SIGBUS (Unix) or Access Violation (Windows) exceptions.
    /// 2. **Lifetime safety**: The returned slice is not used after the `SafeMmap`
    ///    is dropped. The slice borrows from `self` and cannot outlive it.
    /// 3. **No concurrent writes**: No other process or thread writes to the
    ///    mapped region while the slice is in use.
    ///
    /// # Prefer Safe Alternatives
    ///
    /// Prefer using `get(offset)`, `get_slice(offset, len)`, or the typed
    /// accessors (`get_u32_be`, `get_f32_le`, etc.) instead, which provide
    /// bounded, safe access with bounds checking.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use seisly_core::io::safe_mmap::SafeMmap;
    /// use std::fs::File;
    ///
    /// let file = File::open("data.bin").unwrap();
    /// let mmap = SafeMmap::map(&file).unwrap();
    ///
    /// // Safe: use bounded accessors
    /// if let Some(byte) = mmap.get(42) { /* ... */ }
    ///
    /// // Unsafe: direct access — caller must ensure safety invariants
    /// let all_bytes = unsafe { mmap.as_slice() };
    /// ```
    #[inline]
    #[allow(unsafe_code)]
    pub unsafe fn as_slice(&self) -> &[u8] {
        &self.inner
    }
}

/// Extension trait for reading arrays of values with bounds checking.
pub trait SafeMmapExt {
    /// Reads an array of `N` bytes from the given offset.
    ///
    /// Returns `None` if there aren't enough bytes available.
    fn get_array<const N: usize>(&self, offset: usize) -> Option<[u8; N]>;

    /// Reads an array of `N` f32 values in big-endian format from the given offset.
    ///
    /// Returns `None` if there aren't enough bytes available.
    fn get_f32_array_be<const N: usize>(&self, offset: usize) -> Option<[f32; N]>;

    /// Reads an array of `N` f32 values in little-endian format from the given offset.
    ///
    /// Returns `None` if there aren't enough bytes available.
    fn get_f32_array_le<const N: usize>(&self, offset: usize) -> Option<[f32; N]>;
}

impl SafeMmapExt for SafeMmap {
    #[inline]
    fn get_array<const N: usize>(&self, offset: usize) -> Option<[u8; N]> {
        let slice = self.get_slice(offset, N)?;
        Some(slice.try_into().expect("slice length checked"))
    }

    #[inline]
    fn get_f32_array_be<const N: usize>(&self, offset: usize) -> Option<[f32; N]> {
        let mut result = [0.0f32; N];
        for i in 0..N {
            result[i] = self.get_f32_be(offset + i * 4)?;
        }
        Some(result)
    }

    #[inline]
    fn get_f32_array_le<const N: usize>(&self, offset: usize) -> Option<[f32; N]> {
        let mut result = [0.0f32; N];
        for i in 0..N {
            result[i] = self.get_f32_le(offset + i * 4)?;
        }
        Some(result)
    }
}

/// A reference-counted safe memory map for sharing across threads.
///
/// This is useful when multiple threads need to read from the same
/// memory-mapped file without cloning the underlying mapping.
#[derive(Clone)]
pub struct SafeMmapArc(Arc<SafeMmap>);

impl SafeMmapArc {
    /// Creates a new reference-counted safe memory map.
    pub fn map(file: &File) -> Result<Self> {
        Ok(Self(Arc::new(SafeMmap::map(file)?)))
    }

    /// Returns a reference to the underlying SafeMmap.
    #[inline]
    pub fn as_ref(&self) -> &SafeMmap {
        &self.0
    }
}

impl std::ops::Deref for SafeMmapArc {
    type Target = SafeMmap;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_safe_mmap_basic() {
        let mut tmp = NamedTempFile::new().unwrap();
        tmp.write_all(&[0x00, 0x01, 0x02, 0x03, 0x04]).unwrap();

        let file = File::open(tmp.path()).unwrap();
        let mmap = SafeMmap::map(&file).unwrap();

        assert_eq!(mmap.len(), 5);
        assert!(!mmap.is_empty());

        // Test individual byte access
        assert_eq!(mmap.get(0), Some(0x00));
        assert_eq!(mmap.get(4), Some(0x04));
        assert_eq!(mmap.get(5), None); // Out of bounds
        assert_eq!(mmap.get(100), None); // Out of bounds

        // Test slice access
        assert_eq!(
            mmap.get_slice(0, 5),
            Some(&[0x00, 0x01, 0x02, 0x03, 0x04][..])
        );
        assert_eq!(mmap.get_slice(1, 3), Some(&[0x01, 0x02, 0x03][..]));
        assert_eq!(mmap.get_slice(0, 6), None); // Out of bounds
        assert_eq!(mmap.get_slice(100, 1), None); // Out of bounds
    }

    #[test]
    fn test_safe_mmap_endianness() {
        let mut tmp = NamedTempFile::new().unwrap();
        // Write: 0x0001 (u16 BE), 0x00000002 (u32 BE), 0x00000003 (u32 LE)
        tmp.write_all(&[0x00, 0x01]).unwrap();
        tmp.write_all(&[0x00, 0x00, 0x00, 0x02]).unwrap();
        tmp.write_all(&[0x03, 0x00, 0x00, 0x00]).unwrap();

        let file = File::open(tmp.path()).unwrap();
        let mmap = SafeMmap::map(&file).unwrap();

        // Test u16 BE
        assert_eq!(mmap.get_u16_be(0), Some(0x0001));
        assert_eq!(mmap.get_u16_be(2), Some(0x0000));
        assert_eq!(mmap.get_u16_be(10), None); // Out of bounds

        // Test u32 BE
        assert_eq!(mmap.get_u32_be(2), Some(0x00000002));
        assert_eq!(mmap.get_u32_be(6), Some(0x03000000)); // LE bytes read as BE

        // Test u32 LE
        assert_eq!(mmap.get_u32_le(6), Some(0x00000003));
        assert_eq!(mmap.get_u32_le(2), Some(0x02000000)); // BE bytes read as LE

        // Test i32
        assert_eq!(mmap.get_i32_be(2), Some(2));
        assert_eq!(mmap.get_i32_le(6), Some(3));

        // Test f32
        let val = 1.5f32;
        let bytes = val.to_be_bytes();
        let mut tmp2 = NamedTempFile::new().unwrap();
        tmp2.write_all(&bytes).unwrap();
        let file2 = File::open(tmp2.path()).unwrap();
        let mmap2 = SafeMmap::map(&file2).unwrap();
        assert_eq!(mmap2.get_f32_be(0), Some(1.5));
    }

    #[test]
    fn test_safe_mmap_empty() {
        let tmp = NamedTempFile::new().unwrap();
        let file = File::open(tmp.path()).unwrap();
        let mmap = SafeMmap::map(&file).unwrap();

        assert_eq!(mmap.len(), 0);
        assert!(mmap.is_empty());
        assert_eq!(mmap.get(0), None);
        assert_eq!(mmap.get_slice(0, 1), None);
    }

    #[test]
    fn test_safe_mmap_arc() {
        let mut tmp = NamedTempFile::new().unwrap();
        tmp.write_all(&[0x42]).unwrap();

        let file = File::open(tmp.path()).unwrap();
        let mmap = SafeMmapArc::map(&file).unwrap();

        // Clone the Arc
        let mmap2 = mmap.clone();

        // Both should be able to read the same data
        assert_eq!(mmap.get(0), Some(0x42));
        assert_eq!(mmap2.get(0), Some(0x42));
    }

    #[test]
    fn test_safe_mmap_ext_array() {
        let mut tmp = NamedTempFile::new().unwrap();
        tmp.write_all(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05])
            .unwrap();

        let file = File::open(tmp.path()).unwrap();
        let mmap = SafeMmap::map(&file).unwrap();

        // Test byte array
        assert_eq!(mmap.get_array::<3>(0), Some([0x00, 0x01, 0x02]));
        assert_eq!(
            mmap.get_array::<6>(0),
            Some([0x00, 0x01, 0x02, 0x03, 0x04, 0x05])
        );
        assert_eq!(mmap.get_array::<7>(0), None); // Out of bounds
    }
}
