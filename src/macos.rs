use crate::{ClipboardError, FileOperation};
use objc2::{runtime::ProtocolObject, ClassType};
use objc2_app_kit::{NSPasteboard, NSPasteboardURLReadingFileURLsOnlyKey};
use objc2_foundation::{NSArray, NSDictionary, NSNumber, NSURL};
use std::path::PathBuf;

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

    Ok(nsarray_result
        .ok_or(ClipboardError::NoFiles)?
        .iter()
        .filter_map(|s| {
            if let Ok(url_string) = s.downcast::<NSURL>() {
                unsafe {
                    url_string
                        .absoluteString()
                        .map(|f| PathBuf::from(f.to_string()))
                }
            } else {
                None
            }
        })
        .collect::<Vec<PathBuf>>())
}

pub(crate) fn write_clipboard(
    paths: Vec<PathBuf>
) -> Result<(), ClipboardError> {
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
