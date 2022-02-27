//! OS Specific functions

#[cfg(target_os = "macos")]
use objc::{sel, sel_impl};

#[cfg(target_os = "macos")]
unsafe fn center_window(ui: &UI, window: &mut Window) {
    let handle = ui_sys::uiControlHandle(window.ptr() as *mut ui_sys::uiControl) as *mut objc::runtime::Object;
    let _: () = objc::msg_send![handle, center];
}
