use crate::{exif_metadata::ReadExifMetadata, exposure::Exposure};
use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use std::fs;
use std::path::PathBuf;

pub struct FilmLogbookJson {
    path: PathBuf,
}

impl FilmLogbookJson {
    pub fn new(path: &PathBuf) -> Self {
        Self { path: path.clone() }
    }
}

impl ReadExifMetadata for FilmLogbookJson {
    type Error = JSONError;

    fn read_exif_metadata(&self) -> Result<Vec<Exposure>, Self::Error> {
        let raw_payload = fs::read_to_string(&self.path).context(ReadJSONSnafu)?;
        let film_logbook: FilmLogbookJsonPayload =
            serde_json::from_str(&raw_payload).context(ParseJSONSnafu)?;

        let exposures = Vec::try_from(&film_logbook).context(ParseExposureSnafu)?;

        Ok(exposures)
    }
}

impl TryFrom<&FilmLogbookJsonPayload> for Vec<Exposure> {
    type Error = ParseError;

    fn try_from(value: &FilmLogbookJsonPayload) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct FilmLogbookJsonPayload {
    name: String,
    speed: String,
    film: String,
    start: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FilmLogbookPicture {
    frame_number: usize,
    image_reference_uuid: String,
    camera: FilmLogbookCamera,
    lens: FilmLogbookLens,
    shutterspeed: String,
    time: String,
    camera_name: String,
    location: String,
    exposure_compensation: String,
    aperture: String,
    focal_length: String,
    speed: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FilmLogbookCamera {
    notes: String,
    name: String,
    mount: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FilmLogbookLens {
    mount: String,
    max_focal_length: f32,
    min_focal_length: f32,
    prime: String,
    focal_length: f32,
    notes: String,
    name: String,
}

#[derive(Debug, Snafu)]
pub enum ParseError {}

#[derive(Debug, Snafu)]
pub enum JSONError {
    #[snafu(display("Failed to read JSON: {}", source))]
    ReadJSON { source: std::io::Error },

    #[snafu(display("Failed to parse JSON: {}", source))]
    ParseJSON { source: serde_json::Error },

    #[snafu(display("Failed to build exposures: {}", source))]
    ParseExposure { source: ParseError },
}
