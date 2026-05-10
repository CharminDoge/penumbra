use std::fs::{self, File};
use std::io::Read;

pub fn file_size(path: &str) -> std::io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

pub fn file_size_usize(path: &str) -> std::io::Result<usize> {
    let size = file_size(path)?;
    usize::try_from(size).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "file too large for this platform",
        )
    })
}

pub fn read_into_ptr(ptr: *mut u8, len: usize, path: &str) -> std::io::Result<usize> {
    let mut file = File::open(path)?;

    let buf = unsafe {
        std::slice::from_raw_parts_mut(ptr, len)
    };

    let bytes_read = file.read(buf)?;

    Ok(bytes_read)
}