use objc::runtime::{Class, Object};
use objc_foundation::{INSArray, INSString};
use objc_foundation::{NSArray, NSDictionary, NSObject, NSString};
use objc_id::{Id, Owned};
use std::mem::transmute;
use std::path::PathBuf;

use crate::Error;
use crate::FileOperation;

pub(crate) fn read_clipboard() -> Result<Vec<PathBuf>, Error> {
    let clipboard = Clipboard::new()?;
    clipboard.read()
}

pub(crate) fn write_clipboard(paths: Vec<PathBuf>, _operation: FileOperation) -> Result<(), Error> {
    let clipboard = Clipboard::new()?;
    clipboard.write(paths)
}

pub struct Clipboard {
    pasteboard: Id<Object>,
}

// required to bring NSPasteboard into the path of the class-resolver
#[link(name = "AppKit", kind = "framework")]
extern "C" {
    // NSString
    static NSPasteboardURLReadingFileURLsOnlyKey: &'static Object;
}

impl Clipboard {
    pub fn new() -> Result<Clipboard, Error> {
        let ns_pasteboard = class!(NSPasteboard);
        let pasteboard: *mut Object = unsafe { msg_send![ns_pasteboard, generalPasteboard] };
        if pasteboard.is_null() {
            return Err(Error::SystemError(
                "NSPasteboard#generalPasteboard returned null".into(),
            ));
        }
        let pasteboard: Id<Object> = unsafe { Id::from_ptr(pasteboard) };
        Ok(Clipboard { pasteboard })
    }

    pub fn read(&self) -> Result<Vec<PathBuf>, Error> {
        let ns_dict = class!(NSDictionary);
        let ns_number = class!(NSNumber);
        let options: Id<NSDictionary<NSObject, NSObject>> = unsafe {
            let obj: Id<NSObject> =
                Id::from_ptr(msg_send![ns_number, numberWithBool: objc::runtime::YES]);
            Id::from_ptr(
                msg_send![ns_dict, dictionaryWithObject: &*obj forKey: NSPasteboardURLReadingFileURLsOnlyKey],
            )
        };

        let nsurl_class: Id<NSObject> = {
            let cls: Id<Class> = unsafe { Id::from_ptr(class("NSURL")) };
            unsafe { transmute(cls) }
        };

        let classes: Id<NSArray<NSObject, Owned>> = NSArray::from_vec(vec![nsurl_class]);
        let nsurl_array: Id<NSArray<NSObject>> = unsafe {
            let obj: *mut NSArray<NSObject> =
                msg_send![self.pasteboard, readObjectsForClasses:&*classes options:&*options];
            if obj.is_null() {
                return Err(Error::NoFiles);
            }
            Id::from_ptr(obj)
        };

        let results: Vec<_> = nsurl_array
            .to_vec()
            .into_iter()
            .filter_map(|obj| {
                let s: &NSString = unsafe {
                    let is_file: bool = msg_send![obj, isFileURL];
                    if !is_file {
                        return None;
                    }
                    let ret = msg_send![obj, path];
                    ret
                };
                Some(PathBuf::from(s.as_str()))
            })
            .collect();
        if results.is_empty() {
            Err(Error::NoFiles)
        } else {
            Ok(results)
        }
    }

    pub fn write(&self, paths: Vec<PathBuf>) -> Result<(), Error> {
        unsafe{ msg_send![ self.pasteboard, clearContents ]}

        let nsurl_class = class!(NSURL);
        
        let nsurl_array = {
            let nsurl_vec = paths.iter().map(|path| {
                let ns_str = NSString::from_str(path.to_str().unwrap());
                let ns_url: Id<NSObject, Owned> = unsafe { Id::from_ptr(msg_send![nsurl_class, fileURLWithPath:ns_str ]) };
                ns_url
            }).collect();
            NSArray::from_vec(nsurl_vec)
        };


        let success: bool = unsafe { msg_send![ self.pasteboard, writeObjects: nsurl_array] }; 

        if success {
            Ok(())
        } else {
            Err(Error::SystemError("Failed to write file URLs to pasteboard".into()))
        }
    }
    
}

// this is a convenience function that both cocoa-rs and
// glutin define, which seems to depend on the fact that
// Option::None has the same representation as a null pointer
#[inline]
fn class(name: &str) -> *mut Class {
    unsafe { transmute(Class::get(name)) }
}
