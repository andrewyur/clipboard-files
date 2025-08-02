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

use std::fs::canonicalize;
use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;

/// Read the system-wide clipboard. Returns a list of one or more absolute file paths, guaranteed to exist, or an error. if the clipboard contains file paths that do not exist, they are filtered out from the response.
pub fn read() -> Result<Vec<PathBuf>, ClipboardError> {
    let paths = read_clipboard()?;

    Ok(paths.into_iter()
        .filter_map(|f| canonicalize(f).map(strip_prefix).ok())
        .collect::<Vec<_>>())
}

/// Write file paths to the system clipboard. file paths may be relative, but an error will be returned if they do not exist.
pub fn write(paths: Vec<PathBuf>) -> Result<(), ClipboardError> {
    let absolute_paths = paths
        .into_iter()
        .map(|path_buf| canonicalize(path_buf))
        .collect::<Result<Vec<PathBuf>, _>>();

    if absolute_paths.is_err() {
        Err(ClipboardError::NoExist)
    } else {
        write_clipboard(absolute_paths
            .unwrap()
            .into_iter()
            .map(|p| strip_prefix(p))
            .collect::<Vec<_>>()
        )
    }
}

fn strip_prefix(p: PathBuf) -> PathBuf {
    match p.to_str() {
        None => p,
        Some(s) => {
            PathBuf::from_str(s.strip_prefix(r"\\?\").unwrap_or(s)).unwrap_or(p)
        }
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
