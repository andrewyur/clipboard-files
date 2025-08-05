#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
use linux::{read_clipboard, write_clipboard};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos::{read_clipboard, write_clipboard};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::{read_clipboard, write_clipboard};

use std::path::PathBuf;
use thiserror::Error;

/// Read the system-wide clipboard. Returns a list of one or more file paths, taken straight from the clipboard. they are not guaranteed to exist.
pub fn read() -> Result<Vec<PathBuf>, ClipboardError> {
    read_clipboard()
}

/// Write file paths straight to the system clipboard. These do not have to be valid file paths, but some systems may clear paths from the clipboard that are invalid.
pub fn write(paths: &Vec<PathBuf>) -> Result<(), ClipboardError> {
    write_clipboard(paths)
}

#[derive(Debug, PartialEq, Error)]
pub enum ClipboardError {
    #[error("No file paths in the clipboard")]
    NoFiles,
    #[error("The system returned an error: {0}")]
    SystemError(String),
}
