mod csv;
mod exif_metadata;
mod exiftool;
mod exposure_info;
mod photos;
mod utils;

use crate::exif_metadata::ExifMetadata;
use crate::{csv::Csv, exif_metadata::ReadExifMetadata};
use clap::{Parser, Subcommand};
use dirs::home_dir;
use dotenv::dotenv;
use log::debug;
use std::path::{Path, PathBuf};

fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let args = Args::parse();
    debug!("Arguments: {:?}", args);

    match args.command {
        Commands::FromCsv(args) => {
            let photos_path = Path::new(&args.source);
            let metadata_path = Path::new(&args.metadata);

            update_metadata_from_csv(&photos_path.to_path_buf(), &metadata_path.to_path_buf())
        }
    }
}

fn update_metadata_from_csv(photos_path: &PathBuf, metadata_path: &PathBuf) {
    let csv = Csv::new(metadata_path);
    let metadata = ExifMetadata::new(photos_path);
    update_metadata(metadata, csv);
}

fn update_metadata<M>(metadata: ExifMetadata, strategy: M)
where
    M: ReadExifMetadata,
    M::Error: std::error::Error + 'static,
{
    let result = metadata.update_photos(strategy);

    match result {
        Ok(_) => debug!("Done"),
        Err(err) => {
            eprintln!("{}", err);

            panic!();
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author = "Victor Quiroz Castro", version, about = "Film Flicker")]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Applies EXIF Metadata to photos based on a CSV File
    FromCsv(ExifMetadataFromCsv),
}

#[derive(Parser, Debug)]
struct ExifMetadataFromCsv {
    /// Path for the photos
    #[clap(
        short, long, default_value_t = home_dir().unwrap().into_os_string().into_string().unwrap()
    )]
    source: String,

    /// Path for the csv file with the metadata.
    #[clap(short, long)]
    metadata: String,

    /// Name of the film
    #[clap(short, long)]
    film: Option<String>,
}
