mod utils;

use crate::utils::get_photos::get_photos;
use crate::utils::read_metadata::read_metadata;
use crate::utils::update_exif_metadata::update_exif_metadata;
use clap::Parser;
use dirs::home_dir;
use dotenv::dotenv;
use log::debug;
use std::path::Path;

fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let args = Args::parse();
    debug!("Arguments: {:?}", args);

    let source_path = Path::new(&args.source);
    let metadata_path = Path::new(&args.metadata);

    let photo_paths = get_photos(source_path).unwrap();
    let exposures = read_metadata(metadata_path).unwrap();

    let model = args.camera.as_deref();
    let maker = args.maker.as_deref();

    let result = update_exif_metadata(photo_paths, exposures, model, maker);
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
    /// Path for the photos
    #[clap(
        short, long, default_value_t = home_dir().unwrap().into_os_string().into_string().unwrap()
    )]
    source: String,

    /// Path for the csv file with the metadata.
    #[clap(short, long)]
    metadata: String,

    /// Name of the camera
    #[clap(short, long)]
    camera: Option<String>,

    /// Example: KONICA, NIKON, CANON
    #[clap(short = 'k', long)]
    maker: Option<String>,

    /// Name of the film
    #[clap(short, long)]
    film: Option<String>,
}
