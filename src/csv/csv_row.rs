use crate::exposure::{Exposure, ExposureCompensation};
use fraction::{error::ParseError, Fraction, ToPrimitive};
use serde::Deserialize;
use snafu::prelude::*;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct CsvRow {
    camera_name: Option<String>,
    camera_maker: Option<String>,
    lens_name: Option<String>,
    lens_maker: Option<String>,
    focal_length: Option<f32>,
    date: Option<String>,
    iso: Option<i32>,
    aperture: Option<f32>,
    shutter_speed: Option<String>,
    exposure_compensation: Option<String>,
}

impl TryFrom<CsvRow> for Exposure {
    type Error = Error;

    fn try_from(value: CsvRow) -> Result<Self, Self::Error> {
        if let Some(shutter_speed) = &value.shutter_speed {
            if !Self::is_shutter_speed_valid(shutter_speed) {
                return Err(Error::InvalidShutterSpeed {
                    text: shutter_speed.to_string(),
                });
            }
        }

        let exp_comp = Option::<ExposureCompensation>::try_from(&value)?;

        let result = Exposure {
            camera_name: value.camera_name,
            camera_maker: value.camera_maker,
            lens_name: value.lens_name,
            lens_maker: value.lens_maker,
            date: value.date,
            iso: value.iso,
            focal_length: value.focal_length,
            aperture: value.aperture,
            shutter_speed: value.shutter_speed,
            exposure_compensation: exp_comp,
        };

        Ok(result)
    }
}

impl TryFrom<&CsvRow> for Option<ExposureCompensation> {
    type Error = Error;

    fn try_from(value: &CsvRow) -> Result<Self, Self::Error> {
        if let Some(exp_comp) = &value.exposure_compensation {
            // This to allow the format of "1 1/3" or "2 2/3"
            let parts = exp_comp.split(" ").collect::<Vec<&str>>();
            if parts.len() > 2 {
                return Err(Error::InvalidExposureCompensation {
                    value: exp_comp.clone(),
                });
            }

            let float: f32 = parts.iter().fold(0.0, |acc, part| {
                let float = Fraction::from_str(part)
                    .context(ExposureCompensationSnafu {
                        value: exp_comp.clone(),
                    })
                    .unwrap();
                let float: f32 = float.to_f32().unwrap();

                if acc >= 0.0 {
                    acc + float
                } else {
                    acc - float
                }
            });

            let exp_comp = ExposureCompensation::new(float);

            Ok(Some(exp_comp))
        } else {
            Ok(None)
        }
    }
}

type Error = CsvRowError;

#[derive(Debug, Snafu)]
pub enum CsvRowError {
    #[snafu(display("The Shutter speed is incorrect: {} does not follow the pattern", text))]
    InvalidShutterSpeed { text: String },

    #[snafu(display("Wrong format for exposure compensation \"{}\": {:?}", value, source))]
    ExposureCompensation { source: ParseError, value: String },

    #[snafu(display("The format of the exposure compensation \"{}\" is wrong", value))]
    InvalidExposureCompensation { value: String },
}
