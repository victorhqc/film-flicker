mod csv;
mod exif_metadata;
mod exiftool;
mod exposure_info;
mod photos;
mod utils;

use crate::csv::Csv;
use crate::exif_metadata::ExifMetadata;
use clap::{Parser, Subcommand};
use dirs::home_dir;
use dotenv::dotenv;
use log::debug;
use std::path::Path;

fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let args = Args::parse();
    debug!("Arguments: {:?}", args);

    match args.command {
        Commands::ExifApply(args) => {
            handle_exif_apply(args);
        }
    }
}

fn handle_exif_apply(args: ExifApplyArgs) {
    let photos_path = Path::new(&args.source);
    let metadata_path = Path::new(&args.metadata);

    let csv = Csv::new(&metadata_path.to_path_buf());
    let metadata = ExifMetadata::new(&photos_path.to_path_buf());

    let result = metadata.update_photos(csv);

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
    ExifApply(ExifApplyArgs),
}

#[derive(Parser, Debug)]
struct ExifApplyArgs {
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
