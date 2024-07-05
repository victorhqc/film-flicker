use regex::Regex;
use serde::Deserialize;
use snafu::prelude::*;
use std::path::Path;

pub fn read_metadata(path: &Path) -> Result<Vec<ExposureInfo>, Error> {
    let mut rdr = csv::Reader::from_path(path).context(InvalidCSVSnafu)?;

    let mut res = Vec::new();
    for exp in rdr.deserialize() {
        let args: BuildExposureInfo = exp.context(FailedToParseSnafu)?;
        let exposure = ExposureInfo::build(args)?;

        res.push(exposure);
    }

    Ok(res)
}

#[derive(Debug)]
pub struct ExposureInfo {
    pub lens_name: String,
    pub focal_length: f32,
    pub date: String,
    pub iso: i32,
    pub aperture: f32,
    pub shutter_speed: String,
}

#[derive(Debug, Deserialize)]
pub struct BuildExposureInfo {

    lens_name: String,
    focal_length: f32,
    date: String,
    iso: i32,
    aperture: f32,
    shutter_speed: String,
}

impl ExposureInfo {
    pub fn build(args: BuildExposureInfo) -> Result<ExposureInfo, Error> {
        if !Self::is_shutter_speed_valid(&args.shutter_speed) {
            return Err(Error::InvalidShutterSpeed {
                text: args.shutter_speed.to_string(),
            });
        }

        let result = ExposureInfo {
            lens_name: args.lens_name,
            date: args.date,
            iso: args.iso,
            focal_length: args.focal_length,
            aperture: args.aperture,
            shutter_speed: args.shutter_speed,
        };

        Ok(result)
    }

    fn is_shutter_speed_valid(txt: &str) -> bool {
        let expr = Regex::new(r#"1/\d+|\d+""#).unwrap();

        expr.is_match(txt)
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("The Shutter speed is incorrect: {} does not follow the pattern", text))]
    InvalidShutterSpeed { text: String },

    #[snafu(display("Failed to read CSV: {:?}", source))]
    InvalidCSV { source: csv::Error },

    #[snafu(display("Failed to deserialize the row: {:?}", source))]
    FailedToParse { source: csv::Error },
}
