use gtk::gdk::Atom;
use gtk::glib::error::BoolError;
use gtk::{gdk::SELECTION_CLIPBOARD, Clipboard, TargetEntry, TargetFlags};
use std::path::PathBuf;

use crate::ClipboardError;

impl From<BoolError> for ClipboardError {
    fn from(e: BoolError) -> Self {
        ClipboardError::SystemError(format!("System returned an error: {e}"))
    }
}

pub(crate) fn read_clipboard() -> Result<Vec<PathBuf>, ClipboardError> {
    gtk::init()?;
    let cb = Clipboard::get(&SELECTION_CLIPBOARD);
    let paths = cb.wait_for_uris();
    if paths.is_empty() {
        return Err(ClipboardError::NoFiles);
    }
    Ok(paths
        .into_iter()
        .filter_map(|path| path.strip_prefix("file://").map(PathBuf::from))
        .collect())
}

pub(crate) fn write_clipboard(paths: Vec<PathBuf>) -> Result<(), ClipboardError> {
    gtk::init()?;
    let uri_list = paths
        .iter()
        .map(|p| format!("file://{}", p.to_string_lossy()))
        .collect::<Vec<_>>()
        .join("\r\n");

    let clipboard = Clipboard::get(&SELECTION_CLIPBOARD);

    let target = TargetEntry::new("text/uri-list", TargetFlags::empty(), 0);

    match clipboard.set_with_data(&[target], move |_, selection, _info| {
        selection.set(&Atom::intern("text/uri-list"), 8, uri_list.as_bytes());
    }) {
        true => Ok(()),
        false => Err(ClipboardError::SystemError(
            "Could not set clipboard contents!".into(),
        )),
    }
}
