use crate::exposure_info::{BuildExposureInfo, ExposureError, ExposureInfo};
use snafu::prelude::*;
use std::path::Path;

pub fn read(path: &Path) -> Result<Vec<ExposureInfo>, Error> {
    let mut rdr = csv::Reader::from_path(path).context(InvalidCSVSnafu)?;

    let mut res = Vec::new();
    for exp in rdr.deserialize() {
        let args: BuildExposureInfo = exp.context(FailedToParseSnafu)?;
        let exposure = ExposureInfo::build(args).context(ExposureSnafu)?;

        res.push(exposure);
    }

    Ok(res)
}

type Error = ReadError;

#[derive(Debug, Snafu)]
pub enum ReadError {
    #[snafu(display("Failed to read CSV: {}", source))]
    InvalidCSV { source: csv::Error },

    #[snafu(display("Failed to deserialize the row: {}", source))]
    FailedToParse { source: csv::Error },

    #[snafu(display("Failed to build the exposure value: {}", source))]
    Exposure { source: ExposureError },
}
