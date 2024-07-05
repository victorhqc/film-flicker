use clap::Parser;
use dirs::home_dir;
use dotenv::dotenv;
use log::debug;

fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let args = Args::parse();

    debug!("Arguments: {:?}", args);
}

#[derive(Parser, Debug)]
#[clap(author = "Victor Quiroz Castro", version, about = "Film Exif Fixer")]
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
    camera: String,

    /// Example: KONICA, NIKON, CANON
    #[clap(short = 'k', long)]
    maker: String,

    /// Name of the film
    #[clap(short, long)]
    film: String,
}
