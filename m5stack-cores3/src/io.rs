// no_std implementation of File concept using memory buffer
use rustzx_core::{
    error::IoError,
    host::{SeekableAsset, LoadableAsset, SeekFrom}
    };

#[derive(Debug)]
pub enum FileAssetError {
    ReadError,
    SeekError,
}

pub struct FileAsset {
    data: &'static [u8],
    position: usize,
}

impl FileAsset {
    pub fn new(data: &'static [u8]) -> Self {
        FileAsset { data, position: 0 }
    }

    // Helper methods to convert FileAssetError to IoError if necessary
    fn convert_error(error: FileAssetError) -> IoError {
        match error {
            FileAssetError::ReadError => IoError::HostAssetImplFailed,
            FileAssetError::SeekError => IoError::SeekBeforeStart,
        }
    }
}

impl SeekableAsset for FileAsset {
    fn seek(&mut self, pos: SeekFrom) -> Result<usize, IoError> {
        todo!()
    }
}

impl LoadableAsset for FileAsset {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        let available = self.data.len() - self.position; // Bytes remaining
        let to_read = available.min(buf.len()); // Number of bytes to read

        if to_read == 0 {
            return Err(FileAsset::convert_error(FileAssetError::ReadError));
        }

        buf[..to_read].copy_from_slice(&self.data[self.position..self.position + to_read]);
        self.position += to_read; // Update the position

        Ok(to_read) // Return the number of bytes read
    }
}

