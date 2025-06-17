use std::ptr;

use winapi::um::{
    fileapi::{CreateFileA, OPEN_EXISTING},
    handleapi::INVALID_HANDLE_VALUE,
    wincon::{GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO},
    winnt::{FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE},
};

use self::super::Size;

/// Gets the current terminal size
pub fn get() -> Option<Size> {
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Windows
    let handle = unsafe {
        CreateFileA(
            b"CONOUT$\0".as_ptr() as *const i8,
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_WRITE,
            ptr::null_mut(),
            OPEN_EXISTING,
            0,
            ptr::null_mut(),
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        return None;
    }
    let info = unsafe {
        // https://msdn.microsoft.com/en-us/library/windows/desktop/ms683171(v=vs.85).aspx
        let mut info = ::std::mem::MaybeUninit::<CONSOLE_SCREEN_BUFFER_INFO>::uninit();
        if GetConsoleScreenBufferInfo(handle, info.as_mut_ptr()) == 0 {
            None
        } else {
            Some(info.assume_init())
        }
    };
    info.map(|inf| Size {
        rows: (inf.srWindow.Bottom - inf.srWindow.Top + 1) as u16,
        cols: (inf.srWindow.Right - inf.srWindow.Left + 1) as u16,
    })
}
