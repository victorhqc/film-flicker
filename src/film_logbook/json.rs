use crate::exposure::{Camera, Lens, LensKind};
use crate::film_logbook::adapter::{find_adapter, read_adapters};
use crate::utils::parse_mm;
use crate::{exif_metadata::ReadExifMetadata, exposure::Exposure};
use log::debug;
use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use std::fs;
use std::path::PathBuf;

use super::adapter::{Adapter, AdapterError};

pub struct FilmLogbookJson {
    path: PathBuf,
    adapter_path: PathBuf,
}

impl FilmLogbookJson {
    pub fn new(path: &PathBuf, adapter_path: &PathBuf) -> Self {
        Self {
            path: path.clone(),
            adapter_path: adapter_path.clone(),
        }
    }
}

impl ReadExifMetadata for FilmLogbookJson {
    type Error = JSONError;

    fn read_exif_metadata(&self) -> Result<Vec<Exposure>, Self::Error> {
        let raw_payload = fs::read_to_string(&self.path).context(ReadJSONSnafu)?;
        let film_logbook: FilmLogbookJsonPayload =
            serde_json::from_str(&raw_payload).context(ParseJSONSnafu)?;

        let adapters = read_adapters(&self.adapter_path).context(AdapterSnafu)?;
        let data = PayloadAndAdapter(&film_logbook, &adapters);

        let exposures = Vec::try_from(&data).context(ParseExposureSnafu)?;

        Ok(exposures)
    }
}

impl<'a> TryFrom<&'a PayloadAndAdapter<'a>> for Vec<Exposure> {
    type Error = ParseError;

    fn try_from(value: &PayloadAndAdapter) -> Result<Self, Self::Error> {
        let payload = value.0;
        let adapters: &Vec<Adapter> = value.1;

        let mut pictures = payload.pictures.clone();
        pictures.sort_by_key(|p| p.frame_number);

        let mut exposures = Vec::new();

        for picture in pictures {
            debug!("Picture {:?}", picture);

            let picture_and_adapter = PictureAndAdapter(&picture, adapters);

            let camera = Option::<Camera>::from(&picture_and_adapter);
            debug!("Camera {:?}", camera);

            let lens = Option::<Lens>::from(&picture_and_adapter);
            debug!("lens {:?}", lens);

            unimplemented!()
            // let exposure = Exposure { camera };

            // exposures.push(exposure);
        }

        Ok(exposures)
    }
}

impl<'a> From<&'a PictureAndAdapter<'a>> for Option<Camera> {
    fn from(value: &PictureAndAdapter) -> Self {
        let picture = value.0;
        let maybe_adapter = find_adapter(value.1, picture.camera_name.trim());

        if let Some(adapter) = maybe_adapter
            && let Adapter::Camera(camera) = adapter
        {
            Some(Camera::new(&camera.camera_name, &camera.camera_maker, None))
        } else {
            None
        }
    }
}

impl<'a> From<&'a PictureAndAdapter<'a>> for Option<Lens> {
    fn from(value: &PictureAndAdapter) -> Self {
        let picture = value.0;
        let maybe_adapter = find_adapter(value.1, picture.lens.name.trim());

        let maybe_focal_length = parse_mm(&picture.focal_length);
        let kind = if picture.lens.prime.to_lowercase().as_str() == "true" {
            LensKind::Prime
        } else {
            LensKind::Zoom
        };

        if let Some(adapter) = maybe_adapter
            && let Adapter::Lens(lens) = adapter
            && let Some(focal_length) = maybe_focal_length
        {
            let lens_name: Option<&str> = Some(&lens.lens_name).map(|n| n.as_ref());
            let lens_maker: Option<&str> = Some(&lens.lens_maker).map(|m| m.as_ref());

            Some(Lens::new(focal_length, lens_name, lens_maker, Some(kind)))
        } else {
            None
        }
    }
}

struct PayloadAndAdapter<'a>(pub &'a FilmLogbookJsonPayload, pub &'a Vec<Adapter>);
struct PictureAndAdapter<'a>(pub &'a FilmLogbookPicture, pub &'a Vec<Adapter>);

#[derive(Debug, Serialize, Deserialize)]
struct FilmLogbookJsonPayload {
    name: String,
    speed: String,
    film: String,
    start: String,
    end: String,
    pictures: Vec<FilmLogbookPicture>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FilmLogbookCamera {
    notes: String,
    name: String,
    mount: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    ReadJSON {
        source: std::io::Error,
    },

    #[snafu(display("Failed to build adapter: {}", source))]
    Adapter {
        source: AdapterError,
    },

    #[snafu(display("Failed to parse JSON: {}", source))]
    ParseJSON {
        source: serde_json::Error,
    },

    #[snafu(display("Failed to build exposures: {}", source))]
    ParseExposure {
        source: ParseError,
    },

    FocalLength,
}
