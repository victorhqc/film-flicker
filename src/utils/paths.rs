use snafu::prelude::*;
use std::path::PathBuf;

/// Gets the root path for the project. In development is simply the cwd, but it changes once it's
/// a production build, as each OS will have a slightly different configuration. This function is
/// primarily needed because we need to fetch & execute the exiftool script.
pub fn project_root() -> Result<PathBuf, Error> {
    #[cfg(debug_assertions)]
    {
        std::env::current_dir().context(CurrentDirSnafu)
    }

    #[cfg(all(target_os = "windows", not(debug_assertions)))]
    {
        let path = std::env::current_exe()
            .context(CurrentDirSnafu)?
            .parent()
            .context(ParentDirSnafu)?
            .parent()
            .context(ParentDirSnafu)?
            .to_path_buf();

        Ok(path)
    }

    #[cfg(all(not(target_os = "windows"), not(debug_assertions)))]
    unimplemented!()
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to get current dir: {:?}", source))]
    CurrentDir { source: std::io::Error },

    #[cfg(all(target_os = "windows", not(debug_assertions)))]
    #[snafu(display("Failed to get the parent dir"))]
    ParentDir,
}
