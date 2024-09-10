use snafu::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use zip::write::SimpleFileOptions;

fn main() {
    let current = std::env::current_dir().unwrap();

    let exiftool_path_src = current.join("deps").join("exiftool");
    let exiftool_path_dist = current.join("deps").join("exiftool.zip");

    let dist = File::create(&exiftool_path_dist).unwrap();
    let walk_dir = WalkDir::new(&exiftool_path_src);
    let it = walk_dir.into_iter();

    zip_exiftool(&mut it.filter_map(|e| e.ok()), &exiftool_path_src, dist).unwrap();
}

fn zip_exiftool<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    src: &Path,
    writer: T,
) -> Result<(), Error>
where
    T: Write + Seek,
{
    if !Path::new(&src).is_dir() {
        return Err(Error::SrcNotFound);
    };

    println!("cargo:rerun-if-changed=deps/exiftool");

    let mut zip = zip::ZipWriter::new(writer);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let src = Path::new(src);
    let mut buffer = Vec::new();

    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(&src).unwrap();
        let path_string = name
            .to_str()
            .map(str::to_owned).context(BadPathSnafu { path: format!("{:?}", name) })?;

        if path.is_file() {
            println!("adding file {path:?} as {name:?} ...");
            zip.start_file(&path_string, options).unwrap();
            let mut f = File::open(path).context(OpenFileSnafu { name: &path_string })?;

            f.read_to_end(&mut buffer).context(ReadFileSnafu)?;
            zip.write_all(&buffer).context(WriteFileZipSnafu)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            println!("Adding dir {path_string:?} as {name:?} ...");
            zip.add_directory(path_string, options)
                .context(WriteDirZipSnafu)?;
        }
    }

    zip.finish().unwrap();
    println!("Zip finished");
    Ok(())
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("deps/exiftool path does not exist, please run the install command"))]
    SrcNotFound,

    #[snafu(display("Failed to open file {}: {:?}", name, source))]
    OpenFile {
        source: std::io::Error,
        name: String,
    },

    #[snafu(display("Is a Non UTF-8 Path: {}", path))]
    BadPath { path: String },

    #[snafu(display("Failed to read file {:?}", source))]
    ReadFile { source: std::io::Error },

    #[snafu(display("Failed to add file to zip {:?}", source))]
    WriteFileZip { source: std::io::Error },

    #[snafu(display("Failed to add dir to zip {:?}", source))]
    WriteDirZip { source: zip::result::ZipError },
}
