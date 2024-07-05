use crate::utils::read_metadata::ExposureInfo;
use log::debug;
use snafu::prelude::*;
use std::io::Error as IOError;
use std::process::Command;
use std::str::{from_utf8, Utf8Error};

pub fn update_exif_metadata(files: Vec<String>, exposures: Vec<ExposureInfo>, maker: &str) -> Result<(), Error> {
    if files.len() != exposures.len() {
        return Err(Error::BadInformation);
    }

    for (index, file) in files.iter().enumerate() {
        let exposure = exposures.get(index).unwrap();

        debug!("File: {}", file);
        debug!("Exposure: {:?}", exposure);

        exif(file, exposure, maker)?;
    }

    Ok(())
}

pub fn exif(file: &str, exposure: &ExposureInfo, maker: &str) -> Result<(), Error> {
    let mut cmd = Command::new("perl");

    cmd.arg("./deps/exiftool/exiftool")
        .arg(format!("-AllDates={}", exposure.date))
        .arg(format!("-fnumber={}", exposure.aperture))
        .arg(format!("-aperturevalue={}", exposure.aperture))
        .arg(format!("-FocalLength={}mm", exposure.focal_length))
        .arg(format!("-Lens={}mm", exposure.focal_length))
        .arg(format!("-FocalLengthIn35mmFormat={}mm", exposure.focal_length))
        .arg(format!("-ShutterSpeedValue={}", exposure.shutter_speed))
        .arg(format!("-ExposureTime={}", exposure.shutter_speed))
        .arg(format!("-iso={}", exposure.iso))
        .arg(format!("-LensModel={}", exposure.lens_name))
        .arg(format!("-Make={}", maker))
        .arg(file);

    let child = cmd.spawn().context(ExiftoolSpawnSnafu)?;

    let output = child.wait_with_output().context(ExiftoolStdoutSnafu)?;
    let _result = from_utf8(&output.stdout).context(StdoutParseSnafu)?;

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("The amount of images do not match the number of exposures"))]
    BadInformation,

    #[snafu(display("Failed to spawn exiftool: {:?}", source))]
    ExiftoolSpawn { source: IOError },

    #[snafu(display("Failed to get stdout: {:?}", source))]
    ExiftoolStdout { source: IOError },

    #[snafu(display("Failed to parse stdout: {:?}", source))]
    StdoutParse { source: Utf8Error },
}
