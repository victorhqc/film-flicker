#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;
#[cfg(target_os = "windows")]
use winapi::um::winbase::CREATE_NO_WINDOW;

#[cfg(not(target_os = "windows"))]
pub fn spawn_exiftool(_exiftool_path: &Path) -> Command {
    Command::new("perl")
}

#[cfg(target_os = "windows")]
pub fn spawn_exiftool(exiftool_path: &Path) -> Command {
    let mut cmd = Command::new(exiftool_path);

    cmd.creation_flags(CREATE_NO_WINDOW);

    cmd
}
