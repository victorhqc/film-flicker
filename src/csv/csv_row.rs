use crate::exposure::{
    Camera, Exposure, ExposureCompensation, Lens, ShutterSpeed, ShutterSpeedError,
};
use fraction::{Fraction, ToPrimitive, error::ParseError};
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
        let exposure_compensation = Option::<ExposureCompensation>::try_from(&value)?;
        let camera = Option::<Camera>::from(&value);
        let lens = Option::<Lens>::from(&value);
        let shutter_speed = Option::<ShutterSpeed>::try_from(&value)?;

        let result = Exposure {
            camera,
            lens,
            shutter_speed,
            exposure_compensation,
            iso: value.iso,
            aperture: value.aperture,
            date: value.date,
            geo_location: None,
        };

        Ok(result)
    }
}

impl From<&CsvRow> for Option<Camera> {
    fn from(value: &CsvRow) -> Self {
        if let Some((name, maker)) = value.camera_name.as_ref().zip(value.camera_maker.as_ref()) {
            Some(Camera::new(name, maker, None))
        } else {
            None
        }
    }
}

impl From<&CsvRow> for Option<Lens> {
    fn from(value: &CsvRow) -> Self {
        if let Some(focal_length) = value.focal_length {
            let lens_name = value.lens_name.as_ref().map(|x| x.as_str());
            let lens_maker = value.lens_maker.as_ref().map(|x| x.as_str());

            Some(Lens::new(focal_length, lens_name, lens_maker, None))
        } else {
            None
        }
    }
}

impl TryFrom<&CsvRow> for Option<ShutterSpeed> {
    type Error = Error;

    fn try_from(value: &CsvRow) -> Result<Self, Self::Error> {
        if let Some(shutter_speed) = &value.shutter_speed {
            let shutter_speed =
                ShutterSpeed::try_new(shutter_speed.as_str()).context(ShutterSpeedSnafu)?;

            Ok(Some(shutter_speed))
        } else {
            Ok(None)
        }
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

                if acc >= 0.0 { acc + float } else { acc - float }
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
    #[snafu(display("{}", source))]
    ShutterSpeed { source: ShutterSpeedError },

    #[snafu(display("Wrong format for exposure compensation \"{}\": {:?}", value, source))]
    ExposureCompensation { source: ParseError, value: String },

    #[snafu(display("The format of the exposure compensation \"{}\" is wrong", value))]
    InvalidExposureCompensation { value: String },
}
