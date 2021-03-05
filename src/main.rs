use bindings::{
    windows::win32::system_services::MB_OK,
    windows::win32::windows_and_messaging::{MessageBoxA, HWND},
};

fn main() {
    unsafe {
        let caption = b"Hangman\0";
        let text = b"Press OK\0";

        MessageBoxA(
            HWND(0),
            text.as_ptr() as *const i8,
            caption.as_ptr() as *const i8,
            MB_OK as u32,
        );
    }
}