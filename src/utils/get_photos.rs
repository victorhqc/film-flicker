use log::warn;
use snafu::prelude::*;
use std::path::{Path, PathBuf};
use std::{fs, io};
use walkdir::WalkDir;

pub fn get_photos(dir: &Path) -> Result<Vec<String>, Error> {
    if dir.is_file() {
        let info = get_photo_info(&dir.to_path_buf())?;
        return Ok(vec![info]);
    }

    let mut photos: Vec<String> = WalkDir::new(dir)
        .into_iter()
        .map(|e| {
            let entry = e.unwrap();
            let path = entry.path();

            if path.is_dir() {
                return Ok(None);
            }

            let info = match get_photo_info(&path.to_path_buf()) {
                Ok(i) => i,
                Err(err) => {
                    warn!("Failed to get file information: {:?}", err);

                    return Ok(None);
                }
            };

            Ok(Some(info))
        })
        // Ignore errors for now.
        .filter_map(|p: Result<Option<String>, ()>| match p {
            Ok(p) => Some(p),
            Err(err) => {
                warn!("{:?}", err);
                None
            }
        })
        // Filter out none values.
        .flatten()
        .collect();

    photos.sort();

    Ok(photos)
}

fn get_photo_info(path: &PathBuf) -> Result<String, Error> {
    let metadata = fs::metadata(path).context(MetadataSnafu)?;

    let extension = path
        .extension()
        .context(NoExtensionSnafu { entry: path })?
        .to_str()
        .unwrap();

    let file_type = metadata.file_type();

    if !file_type.is_file() || !is_photo(extension) {
        return Err(Error::InvalidExtension);
    }

    if let Some(p) = path.to_str() {
        return Ok(p.to_string());
    }

    Err(Error::InvalidFile)
}

fn is_photo(extension: &str) -> bool {
    matches!(extension.to_lowercase().as_str(), |"tiff"| "jpeg"
        | "jpg"
        | "png")
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to read metadata: {}", source))]
    Metadata { source: io::Error },

    #[snafu(display("File has no extension: {}", entry.display()))]
    NoExtension { entry: PathBuf },

    #[snafu(display("File extension not valid"))]
    InvalidExtension,

    #[snafu(display("File is not valid"))]
    InvalidFile,
}
