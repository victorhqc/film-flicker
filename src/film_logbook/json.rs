use crate::exposure::{
    Camera, ExposureCompensation, GeoLocation, GeoLocationError, Lens, LensKind, ShutterSpeed,
    ShutterSpeedError,
};
use crate::film_logbook::adapter::{find_adapter, read_adapters};
use crate::utils::{
    ExiftoolDateError, ParseExposureCompensationError, parse_aperture, parse_exposure_compensation,
    parse_mm, parse_to_exiftool_format,
};
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
            debug!("-------");

            let picture_and_adapter = PictureAndAdapter(&picture, adapters);

            let camera = Option::<Camera>::from(&picture_and_adapter);
            debug!("{:?}", camera);

            let lens = Option::<Lens>::from(&picture_and_adapter);
            debug!("{:?}", lens);

            let iso: Option<i32> = picture.speed.parse().ok();
            debug!("ISO {:?}", iso);

            let aperture: Option<f32> = parse_aperture(&picture.aperture);
            debug!("APerture {:?}", aperture);

            let shutter_speed =
                ShutterSpeed::try_new(&picture.shutterspeed).context(ShutterSpeedSnafu {
                    frame_number: picture.frame_number,
                })?;
            let shutter_speed: Option<ShutterSpeed> = Some(shutter_speed);
            debug!("{:?}", shutter_speed);

            let exposure_compensation = parse_exposure_compensation(&picture.exposure_compensation)
                .context(EsposureCompensationSnafu {
                    frame_number: picture.frame_number,
                })?;
            let exposure_compensation: Option<ExposureCompensation> = Some(exposure_compensation);
            debug!("{:?}", exposure_compensation);

            let date = parse_to_exiftool_format(&picture.time).context(TimeSnafu {
                frame_number: picture.frame_number,
            })?;
            let date = Some(date);
            debug!("Date {:?}", date);

            let geo_location =
                GeoLocation::try_from(picture.location.as_str()).context(GeoLocationSnafu {
                    frame_number: picture.frame_number,
                })?;
            let geo_location = Some(geo_location);
            debug!("{:?}", geo_location);

            debug!("-------");

            if camera.is_none() {
                return Err(ParseError::MissingCamera {
                    frame_number: picture.frame_number,
                });
            }

            if lens.is_none() {
                return Err(ParseError::MissingLens {
                    frame_number: picture.frame_number,
                });
            }

            if aperture.is_none() {
                return Err(ParseError::MissingAperture {
                    frame_number: picture.frame_number,
                });
            }

            let exposure = Exposure {
                camera,
                lens,
                iso,
                aperture,
                shutter_speed,
                exposure_compensation,
                date,
                geo_location,
            };

            exposures.push(exposure);
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
pub struct FilmLogbookPicture {
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
pub struct FilmLogbookCamera {
    notes: String,
    name: String,
    mount: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilmLogbookLens {
    mount: String,
    max_focal_length: f32,
    min_focal_length: f32,
    prime: String,
    focal_length: f32,
    notes: String,
    name: String,
}

#[derive(Debug, Snafu)]
pub enum ParseError {
    #[snafu(display(
        "Failed to find a camera for a picture (frame #{}). Verify the adapters",
        frame_number
    ))]
    MissingCamera { frame_number: usize },

    #[snafu(display(
        "Failed to find a lens for a picture (frame #{}). Verify the adapters",
        frame_number
    ))]
    MissingLens { frame_number: usize },

    #[snafu(display(
        "Failed to read the aperture for a picture (frame #{}). Verify the that the aperture starts with f/",
        frame_number
    ))]
    MissingAperture { frame_number: usize },

    #[snafu(display(
        "Failed to read the shutter speed for a picture (frame #{}): {}",
        frame_number,
        source
    ))]
    ShutterSpeed {
        source: ShutterSpeedError,
        frame_number: usize,
    },

    #[snafu(display(
        "Failed to parse the exposure compensation (frame #{}): {}",
        frame_number,
        source
    ))]
    EsposureCompensation {
        source: ParseExposureCompensationError,
        frame_number: usize,
    },

    #[snafu(display("Failed to parse the time (frame #{}): {}", frame_number, source))]
    Time {
        source: ExiftoolDateError,
        frame_number: usize,
    },

    #[snafu(display(
        "Failed to parse the geo location (frame #{}): {}",
        frame_number,
        source
    ))]
    GeoLocation {
        source: GeoLocationError,
        frame_number: usize,
    },
}

#[derive(Debug, Snafu)]
pub enum JSONError {
    #[snafu(display("Failed to read JSON: {}", source))]
    ReadJSON { source: std::io::Error },

    #[snafu(display("Failed to build adapter: {}", source))]
    Adapter { source: AdapterError },

    #[snafu(display("Failed to parse JSON: {}", source))]
    ParseJSON { source: serde_json::Error },

    #[snafu(display("Failed to build exposures: {}", source))]
    ParseExposure { source: ParseError },
}
