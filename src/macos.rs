use crate::ClipboardError;
use objc2::{runtime::ProtocolObject, ClassType};
use objc2_app_kit::{NSPasteboard, NSPasteboardURLReadingFileURLsOnlyKey};
use objc2_foundation::{NSArray, NSDictionary, NSNumber, NSURL};
use std::path::PathBuf;
use std::str::FromStr;

pub(crate) fn read_clipboard() -> Result<Vec<PathBuf>, ClipboardError> {
    let pasteboard = unsafe { NSPasteboard::generalPasteboard() };

    let val = NSNumber::numberWithBool(true);
    let options = NSDictionary::from_slices(
        &[unsafe { NSPasteboardURLReadingFileURLsOnlyKey }],
        &[val.as_ref()],
    );

    let class_arr = NSArray::from_slice(&[NSURL::class()]);

    let nsarray_result =
        unsafe { pasteboard.readObjectsForClasses_options(&class_arr, Some(options.as_ref())) };

    let ns_array = nsarray_result.ok_or(ClipboardError::NoFiles)?;

    if ns_array.count() == 0 {
        Err(ClipboardError::NoFiles)
    } else {
        Ok(ns_array
            .iter()
            .filter_map(|s| {
                if let Ok(url_string) = s.downcast::<NSURL>() {
                    unsafe {
                        url_string
                            .absoluteString()
                            .map(|f| strip_prefix(PathBuf::from(f.to_string())))
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<PathBuf>>())
    }
}

pub(crate) fn write_clipboard(paths: &Vec<PathBuf>) -> Result<(), ClipboardError> {
    let nsurl_array = NSArray::from_retained_slice(
        &paths
            .iter()
            .filter_map(|p| NSURL::from_file_path(p.as_path()).map(ProtocolObject::from_retained))
            .collect::<Vec<_>>(),
    );

    unsafe {
        let pasteboard = NSPasteboard::generalPasteboard();
        pasteboard.clearContents();
        if !pasteboard.writeObjects(&*nsurl_array) {
            return Err(ClipboardError::SystemError(
                "Could not write to system clipboard!".into(),
            ));
        }
    }

    Ok(())
}

fn strip_prefix(p: PathBuf) -> PathBuf {
    match p.to_str() {
        None => p,
        Some(s) => PathBuf::from_str(s.strip_prefix(r"file://").unwrap_or(s)).unwrap_or(p),
    }
}
