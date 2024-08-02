pub mod hook_set {
    use std::mem;
    use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{
        CallNextHookEx, DispatchMessageA, GetMessageA, SetWindowsHookExA, TranslateMessage,
        UnhookWindowsHookEx, HHOOK, MOUSEHOOKSTRUCT, MSG, WH_MOUSE_LL,
    };

    use crate::{Points};

    pub struct Hook {
        pub hhook: Option<HHOOK>,
    }

    impl Default for Hook {
        fn default() -> Self {
            Hook { hhook: None }
        }
    }

    unsafe extern "system" fn mouse_event(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        if w_param.0 == 0x200 {
            // 0x200 == MouseMove
            let mut mouse_points = *(l_param.0 as *const MOUSEHOOKSTRUCT);
            if mouse_points.pt.x < 0 {
                mouse_points.pt.x = 0;
            }
            if mouse_points.pt.y < 0 {
                mouse_points.pt.y = 0;
            }
            Points.write().x = mouse_points.pt.x as u16;
            Points.write().y = mouse_points.pt.y as u16;
        }
        return CallNextHookEx(HHOOK::default(), code, w_param, l_param);
    }

    impl Hook {
        unsafe fn message_loop(&mut self, hwnd: HWND, msg_filter_min: u32, msg_filter_max: u32) {
            let mut message: MSG = mem::zeroed();
            while GetMessageA(&mut message, hwnd, msg_filter_min, msg_filter_max).into() {
                let _ = TranslateMessage(&message);
                DispatchMessageA(&message);
            }
            Self::unregister(self.hhook.unwrap());
        }
        pub unsafe fn create_hook(&mut self, _lparam: LPARAM) -> HHOOK {
            let hhook: HHOOK = SetWindowsHookExA(WH_MOUSE_LL, Some(mouse_event), None, 0).unwrap();
            self.hhook = Some(hhook);
            return hhook;
        }
        pub unsafe fn get_messages(
            hook: &mut Hook,
            hwnd: HWND,
            msg_filter_min: u32,
            msg_filter_max: u32,
        ) {
            Hook::message_loop(hook, hwnd, msg_filter_min, msg_filter_max);
        }
        unsafe fn unregister(hhook: HHOOK) {
            let _ = UnhookWindowsHookEx(hhook);
        }
    }
}
