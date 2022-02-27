//! OS Specific functions

#[cfg(target_os = "macos")]
pub mod macos {
    use crate::controls::Window;
    use crate::UI;
    
    extern crate objc;
    use self::objc::{sel, sel_impl, msg_send};
    
    pub unsafe fn center_window(ui: &UI, window: &mut Window) {
        let handle = ui_sys::uiControlHandle(window.ptr() as *mut ui_sys::uiControl) as *mut objc::runtime::Object;
        let _: () = msg_send![handle, center];
    }
}

