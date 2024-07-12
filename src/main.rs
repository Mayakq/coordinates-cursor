use std::{mem, thread};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{CallNextHookEx, DispatchMessageA, GetMessageA, HHOOK, MOUSEHOOKSTRUCT, MSG, SetWindowsHookExA, TranslateMessage, UnhookWindowsHookEx, WH_MOUSE_LL};

mod hooks;


struct Hook {
    hhook: HHOOK,
}

impl Hook {
    unsafe fn message_loop() {
        let mut message: MSG = mem::zeroed();
        while GetMessageA(&mut message, HWND::default(), 0, 0).into() {
            let _ = TranslateMessage(&message);
            DispatchMessageA(&message);
        }
    }
    unsafe fn mouse_logger(_lparam: LPARAM) -> Hook {
        let hhook = SetWindowsHookExA(
            WH_MOUSE_LL,
            Some(Hook::mouse_event),
            None,
            0,
        ).unwrap();
        return Hook {
            hhook,
        };
    }
    unsafe fn get_messages() {
        Hook::message_loop();
    }
    unsafe fn unregister(hhook: HHOOK) -> bool {
        if UnhookWindowsHookEx(hhook).is_ok() {
            return true;
        };
        return false;
    }
    unsafe extern "system" fn mouse_event(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        let _key_info: MOUSEHOOKSTRUCT = *(l_param.0 as *const MOUSEHOOKSTRUCT);
        if w_param.0 == 0x200 {}
        return CallNextHookEx(HHOOK::default(), code, w_param, l_param);
    }
}


fn main() {
    let hhok = unsafe { Hook::mouse_logger(LPARAM::default()) };
    let a = thread::spawn(|| unsafe {
        Hook::message_loop();
    });
    a.join().unwrap();
}



