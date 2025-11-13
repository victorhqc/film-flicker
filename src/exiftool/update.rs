use super::spawn::spawn_exiftool;
use crate::exposure_info::ExposureInfo;
use crate::utils::paths::{project_root, PathsError};
use console::Emoji;
use indicatif::ProgressBar;
use log::{debug, trace};
use snafu::prelude::*;
use std::io::Error as IOError;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Output;
#[cfg(target_os = "windows")]
use winapi::um::winbase::CREATE_NO_WINDOW;

static FILM: Emoji<'_, '_> = Emoji("🎞️ ", "");

pub fn update_exif_metadata(files: Vec<String>, exposures: Vec<ExposureInfo>) -> Result<(), Error> {
    if files.len() != exposures.len() {
        return Err(Error::BadInformation {
            photos: files.len(),
            exposures: exposures.len(),
        });
    }

    let root = project_root().context(PathSnafu)?;

    #[cfg(target_os = "windows")]
    let exiftool_path = root.join("deps").join("exiftool").join("exiftool(-k).exe");

    #[cfg(not(target_os = "windows"))]
    let exiftool_path = root.join("deps").join("exiftool").join("exiftool");

    debug!("Exiftool Dir {:?}", exiftool_path);

    println!("\n");
    println!("{}Processing {} Photos...", FILM, files.len());
    println!("\n");

    let pb = ProgressBar::new(files.len() as u64);

    for (index, file) in files.iter().enumerate() {
        let exposure = exposures.get(index).unwrap();

        trace!("File: {}", file);
        trace!("Exposure: {:?}", exposure);

        let args = ExifArgs { file, exposure };

        exiftool(&args, &exiftool_path)?;
        trace!("\n");
        pb.inc(1);
    }

    pb.finish();
    println!("\n");

    Ok(())
}

fn exiftool(args: &ExifArgs, exiftool_path: &Path) -> Result<(), Error> {
    let mut cmd = spawn_exiftool(exiftool_path);

    #[cfg(not(target_os = "windows"))]
    let mut cmd = cmd.arg(exiftool_path);

    #[cfg(target_os = "windows")]
    let mut cmd = &mut cmd;

    if let Some(date) = &args.exposure.date {
        cmd = cmd.arg(format!("-AllDates={}", date));
    }

    if let Some(aperture) = args.exposure.aperture {
        cmd = cmd
            .arg(format!("-fnumber={}", aperture))
            .arg(format!("-aperturevalue={}", aperture));
    }

    if let Some(focal_length) = args.exposure.focal_length {
        cmd = cmd
            .arg(format!("-FocalLength={}mm", focal_length))
            .arg(format!("-Lens={}mm", focal_length))
            .arg(format!("-FocalLengthIn35mmFormat={}mm", focal_length));
    }

    if let Some(shutter_speed) = &args.exposure.shutter_speed {
        cmd = cmd
            .arg(format!("-ShutterSpeedValue={}", shutter_speed))
            .arg(format!("-ExposureTime={}", shutter_speed));
    }

    if let Some(iso) = args.exposure.iso {
        cmd = cmd.arg(format!("-iso={}", iso));
    }

    if let Some(lens_name) = &args.exposure.lens_name {
        cmd = cmd.arg(format!("-LensModel={}", lens_name));
    }

    if let Some(lens_maker) = &args.exposure.lens_maker {
        cmd = cmd.arg(format!("-LensMake={}", lens_maker));
    }

    if let Some(maker) = &args.exposure.camera_maker {
        cmd = cmd.arg(format!("-Make={}", maker));
    }

    if let Some(model) = &args.exposure.camera_name {
        cmd = cmd.arg(format!("-Model={}", model));
    }

    if let Some(exp_comp) = args.exposure.exposure_compensation {
        trace!("Applying exposure compensation as {}", exp_comp);
        cmd = cmd.arg(format!("-ExposureCompensation={:.2}", exp_comp));
    }

    let cmd = cmd.arg(args.file);

    #[cfg(not(target_os = "windows"))]
    let output: Output = {
        let child = cmd.spawn().context(ExiftoolSpawnSnafu {
            path: format!("{}", exiftool_path.display()),
        })?;

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
}

type Error = UpdateError;

#[derive(Debug, Snafu)]
pub enum UpdateError {
    #[snafu(display("The amount of images do not match the number of exposures, photos found: {}, exposures in metadata: {}", photos, exposures))]
    BadInformation { photos: usize, exposures: usize },

    #[snafu(display("Failed to run exiftool \"{}\": {:?}", path, source))]
    ExiftoolSpawn { source: IOError, path: String },

    #[cfg(not(target_os = "windows"))]
    #[snafu(display("Failed to get run exiftool: {:?}", source))]
    ExiftoolWait { source: IOError },

    #[snafu(display("Failed to run exiftool: {:?}", stderr))]
    ExiftoolExe { stderr: String },

    #[snafu(display("Failed to get path for exiftool: {:?}", source))]
    Path { source: PathsError },
}
