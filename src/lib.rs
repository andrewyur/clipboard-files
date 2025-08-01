#[macro_use]
#[cfg(target_os = "macos")]
extern crate objc;

#[cfg(target_os = "linux")]
mod linux;
use std::path::PathBuf;

#[cfg(target_os = "linux")]
use linux::read_clipboard;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos::{read_clipboard, write_clipboard};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::{read_clipboard, write_clipboard};

use thiserror::Error;

/// Read the system-wide clipboard. Returns a list of one or more absolute file paths or an error.
pub fn read() -> Result<Vec<PathBuf>, ClipboardError> {
    read_clipboard()
}

/// Write file paths to the system clipboard. file paths may be relative, but an error will be returned if they do not exist. Operation parameter is only used on linux systems.
pub fn write(paths: Vec<PathBuf>, operation: FileOperation) -> Result<(), ClipboardError> {
    let absolute_paths = paths.into_iter().map(|path_buf| {
        std::fs::canonicalize(path_buf)
    }).collect::<Result<Vec<PathBuf>, _> >();

    if absolute_paths.is_err() {
        Err(ClipboardError::NoExist)
    } else {
        write_clipboard(absolute_paths.unwrap(), operation)
    }
}  

#[derive(Debug, PartialEq, Error)]
pub enum ClipboardError {
    #[error("No file paths in the clipboard")]
    NoFiles,
    #[error("One or more of the given file paths do not exist")]
    NoExist,
    #[error("The system returned an error: {0}")]
    SystemError(String),
}

#[derive(Debug, PartialEq)]
pub enum FileOperation {
    Copy,
    Move,
}