use super::csv_row::{CsvRow, CsvRowError};
use crate::{exif_metadata::ReadExifMetadata, exposure::Exposure};
use snafu::prelude::*;
use std::path::PathBuf;

pub struct Csv {
    path: PathBuf,
}

impl Csv {
    pub fn new(path: &PathBuf) -> Self {
        Self { path: path.clone() }
    }
}

impl ReadExifMetadata for Csv {
    type Error = ReadError;

    fn read_exif_metadata(&self) -> Result<Vec<Exposure>, Self::Error> {
        let mut rdr = csv::Reader::from_path(&self.path).context(InvalidCSVSnafu)?;

        let mut res = Vec::new();
        for exp in rdr.deserialize() {
            let args: CsvRow = exp.context(FailedToParseSnafu)?;
            let exposure = Exposure::try_from(args).context(ExposureSnafu)?;

            res.push(exposure);
        }

        Ok(res)
    }
}

#[derive(Debug, Snafu)]
pub enum ReadError {
    #[snafu(display("Failed to read CSV: {}", source))]
    InvalidCSV { source: csv::Error },

    #[snafu(display("Failed to deserialize the row: {}", source))]
    FailedToParse { source: csv::Error },

    #[snafu(display("Failed to build the exposure value: {}", source))]
    Exposure { source: CsvRowError },
}
