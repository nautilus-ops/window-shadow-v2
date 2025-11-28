use tauri::{App, Manager};

pub fn set_shadows(app: &mut App, enable: bool) {
    #[cfg(target_os = "macos")]
    unsafe {
        let window = app.get_window("main").unwrap();
        let ns_window = window.ns_window().unwrap();

        let win = ns_window as *mut objc2_app_kit::NSWindow;
        (*win).setHasShadow(enable);
    }

    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::{
            Graphics::Dwm::DwmExtendFrameIntoClientArea, UI::Controls::MARGINS,
        };

        let window = app.get_window("main").unwrap();

        let hwnd = window.hwnd().unwrap().0;

        let m = if enable { 1 } else { 0 };
        let margins = MARGINS {
            cxLeftWidth: m,
            cxRightWidth: m,
            cyTopHeight: m,
            cyBottomHeight: m,
        };
        unsafe {
            DwmExtendFrameIntoClientArea(hwnd as _, &margins);
        };
    }
}

