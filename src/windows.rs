use std::path::PathBuf;
use std::ptr::copy_nonoverlapping;
use crate::{ClipboardError};
use windows::Win32::{
    Foundation::{HWND, HANDLE, GetLastError, ERROR_SUCCESS},
    System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE}, 
    System::Ole::CF_HDROP,
    UI::Shell::{DragQueryFileW, DROPFILES, HDROP},
};
use windows::Win32::System::DataExchange::{
    CloseClipboard, 
    GetClipboardData, 
    IsClipboardFormatAvailable, 
    OpenClipboard,
    EmptyClipboard,
    SetClipboardData,
};
use windows::core::{Error as WinError, BOOL};



impl From<WinError> for ClipboardError {
    fn from(e: WinError) -> Self {
        ClipboardError::SystemError(format!("System returned an error: {e}"))
    }
}


struct ClipboardGuard;

impl ClipboardGuard {
    pub fn open() -> Result<Self, ClipboardError> {
        unsafe { OpenClipboard(Some(HWND(std::ptr::null_mut()))) }?;
        Ok(Self)
    }
}

impl Drop for ClipboardGuard {
    fn drop(&mut self) {
        unsafe { let _ = CloseClipboard(); }
    }
}

pub(crate) fn read_clipboard() -> Result<Vec<PathBuf>, ClipboardError> {
    let mut paths = Vec::new();
    
    // when this is dropped, clipboard gets closed closed
    let _guard = ClipboardGuard::open()?;

    unsafe { IsClipboardFormatAvailable(CF_HDROP.0.into()) }.map_err(|_| ClipboardError::NoFiles)?;

    let hdrop_data =  unsafe { GetClipboardData(CF_HDROP.0.into()) }?;
    let hdrop = HDROP(hdrop_data.0);
    let count = unsafe { DragQueryFileW(hdrop, 0xFFFFFFFF, None) };

    for i in 0..count {
        unsafe {
            let len = DragQueryFileW(hdrop, i, None);
            let mut buf = vec![0u16; (len + 1) as usize];
            DragQueryFileW(hdrop, i, Some(&mut buf));
            if let Some(s) = String::from_utf16(&buf[..len as usize]).ok() {
                paths.push(s.into());
            }
        }
    }

    return Ok(paths);
}

pub(crate) fn write_clipboard(paths: Vec<PathBuf>) -> Result<(), ClipboardError>{
    let mut path_buf = String::new();

    paths.iter().for_each(| path| {
        let s = path.display().to_string(); 

        path_buf.push_str(&s);
        path_buf.push('\0');
    });
    path_buf.push('\0');

    let utf16: Vec<u16> = path_buf.encode_utf16().collect();
    let dropfiles_size = std::mem::size_of::<DROPFILES>();
    let total_size = dropfiles_size + utf16.len() * 2;

    let dropfiles = DROPFILES {
        pFiles: dropfiles_size as u32,
        pt: Default::default(),
        fNC: BOOL(0),
        fWide: BOOL(1),
    };
    


    let h_global;
    
    unsafe {
        h_global =  GlobalAlloc(GMEM_MOVEABLE, total_size)?;
        let ptr =  GlobalLock(h_global) as *mut u8;

         *(ptr as *mut DROPFILES) = dropfiles; 
        let dest = ptr.add(dropfiles_size) as *mut u16 ;
        copy_nonoverlapping(utf16.as_ptr(), dest, utf16.len());

        if !GlobalUnlock(h_global).is_ok() {
            let err = GetLastError();
            if err != ERROR_SUCCESS {
                return Err(WinError::from_win32().into());
            }
        }
    }

    let _guard = ClipboardGuard::open()?;
    
    unsafe { 
        EmptyClipboard()?; 
        SetClipboardData(CF_HDROP.0.into(), Some(HANDLE(h_global.0)))?;
    }

    Ok(())

}
