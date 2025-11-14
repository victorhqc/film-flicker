use snafu::prelude::*;
use std::path::{Path, PathBuf};

use crate::exiftool::{update_exif_metadata, UpdateError};
use crate::exposure_info::ExposureInfo;
use crate::photos::{get_paths, PathsError};

pub trait ReadExifMetadata {
    type Error;

    fn read_exif_metadata(&self) -> Result<Vec<ExposureInfo>, Self::Error>;
}

pub struct ExifMetadata {
    photos_path: PathBuf,
}

impl ExifMetadata {
    pub fn new(photos_path: &PathBuf) -> Self {
        Self {
            photos_path: photos_path.clone(),
        }
    }

    pub fn update_photos<M>(&self, metadata: M) -> Result<(), ExifMetadataError<M::Error>>
    where
        M: ReadExifMetadata,
        M::Error: std::error::Error + 'static,
    {
        let photos_path = Path::new(&self.photos_path);

        let photo_paths = get_paths(photos_path).context(PhotosPathSnafu)?;
        let exposures = metadata.read_exif_metadata().context(ReadMetadataSnafu)?;

        update_exif_metadata(photo_paths, exposures).context(UpdateSnafu)
    }
}

#[derive(Snafu, Debug)]
pub enum ExifMetadataError<E>
where
    E: std::error::Error + 'static,
{
    #[snafu(display("Failed to update EXIF metadata: {}", source))]
    Update { source: UpdateError },

    #[snafu(display("Failed to get photos paths: {}", source))]
    PhotosPath { source: PathsError },

    #[snafu(display("Failed to read metadata: {}", source))]
    ReadMetadata { source: E },
}
