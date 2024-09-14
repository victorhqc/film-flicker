use super::paths::{project_root, Error as PathError};
use crate::utils::read_metadata::ExposureInfo;
use log::debug;
use snafu::prelude::*;
use std::io::Error as IOError;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Command, Output};
#[cfg(target_os = "windows")]
use winapi::um::winbase::CREATE_NO_WINDOW;

pub fn update_exif_metadata(
    files: Vec<String>,
    exposures: Vec<ExposureInfo>,
    model: &str,
    maker: &str,
) -> Result<(), Error> {
    if files.len() != exposures.len() {
        return Err(Error::BadInformation);
    }

    let root = project_root().context(PathSnafu)?;

    #[cfg(target_os = "windows")]
    let exiftool_path = root.join("deps").join("exiftool").join("exiftool(-k).exe");

    #[cfg(not(target_os = "windows"))]
    let exiftool_path = root.join("deps").join("exiftool").join("exiftool");

    debug!("Exiftool Dir {:?}", exiftool_path);

    for (index, file) in files.iter().enumerate() {
        let exposure = exposures.get(index).unwrap();

        debug!("File: {}", file);
        debug!("Exposure: {:?}", exposure);

        let args = ExifArgs {
            file,
            model,
            exposure,
            maker,
        };

        exiftool(&args, &exiftool_path)?;
        debug!("\n");
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn spawn_exiftool(_exiftool_path: &Path) -> Result<Command, Error> {
    let cmd = Command::new("perl");

    Ok(cmd)
}

#[cfg(target_os = "windows")]
pub fn spawn_exiftool(exiftool_path: &Path) -> Result<Command, Error> {
    let mut cmd = Command::new(exiftool_path);

    cmd.creation_flags(CREATE_NO_WINDOW);

    Ok(cmd)
}

pub fn exiftool(args: &ExifArgs, exiftool_path: &Path) -> Result<(), Error> {
    let mut cmd = spawn_exiftool(exiftool_path)?;

    #[cfg(not(target_os = "windows"))]
    let cmd = cmd.arg(exiftool_path);

    let cmd = cmd
        .arg(format!("-AllDates={}", args.exposure.date))
        .arg(format!("-fnumber={}", args.exposure.aperture))
        .arg(format!("-aperturevalue={}", args.exposure.aperture))
        .arg(format!("-FocalLength={}mm", args.exposure.focal_length))
        .arg(format!("-Lens={}mm", args.exposure.focal_length))
        .arg(format!(
            "-FocalLengthIn35mmFormat={}mm",
            args.exposure.focal_length
        ))
        .arg(format!(
            "-ShutterSpeedValue={}",
            args.exposure.shutter_speed
        ))
        .arg(format!("-ExposureTime={}", args.exposure.shutter_speed))
        .arg(format!("-iso={}", args.exposure.iso))
        .arg(format!("-LensModel={}", args.exposure.lens_name))
        .arg(format!("-Make={}", args.maker))
        .arg(format!("-Model={}", args.model));

    let cmd = if let Some(exp_comp) = &args.exposure.exposure_compensation {
        debug!("Applying exposure compensation as {}", exp_comp);
        cmd.arg(format!("-ExposureCompensation={:.2}", exp_comp))
    } else {
        cmd
    };

    let cmd = cmd.arg(args.file);

    #[cfg(not(target_os = "windows"))]
    let output: Output = {
        let child = cmd.spawn().context(ExiftoolSpawnSnafu)?;

        child.wait_with_output().context(ExiftoolWaitSnafu)?
    };

    #[cfg(target_os = "windows")]
    let output: Output = {
        cmd.output().context(ExiftoolSpawnSnafu {
            path: format!("{}", exiftool_path.display()),
        })?
    };

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);

        Err(Error::ExiftoolExe {
            stderr: stderr.to_string(),
        })
    }
}

pub struct ExifArgs<'a> {
    file: &'a str,
    exposure: &'a ExposureInfo,
    model: &'a str,
    maker: &'a str,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("The amount of images do not match the number of exposures"))]
    BadInformation,

    #[snafu(display("Failed to run exiftool \"{}\": {:?}", path, source))]
    ExiftoolSpawn { source: IOError, path: String },

    #[cfg(not(target_os = "windows"))]
    #[snafu(display("Failed to get run exiftool: {:?}", source))]
    ExiftoolWait { source: IOError },

    #[snafu(display("Failed to run exiftool: {:?}", stderr))]
    ExiftoolExe { stderr: String },

    #[snafu(display("Failed to get path for exiftool: {:?}", source))]
    Path { source: PathError },
}
