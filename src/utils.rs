use std::{ffi::OsString, mem, os::windows::ffi::OsStringExt};
use winapi::{
    shared::{minwindef, windef},
    um::winuser,
};

// This is mostly taken from https://github.com/Bowarc/Lumin (workerw_fetcher and client::window_manager::windows::utils)

pub fn get_screens() -> Vec<crate::data::Monitor> {
    let mut output = Vec::new();

    for monitor in enumerate_monitors() {
        let name = match &monitor.szDevice[..].iter().position(|c| *c == 0) {
            Some(len) => OsString::from_wide(&monitor.szDevice[0..*len]),
            None => OsString::from_wide(&monitor.szDevice[0..monitor.szDevice.len()]),
        };

        output.push(crate::data::Monitor::from_info(
            name.to_str().unwrap_or("????").to_string(),
            monitor,
        ));
    }
    output
}

pub fn get_window_pos_size(window_id: *mut windef::HWND__) -> crate::data::Rect {
    let mut rect = windef::RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    unsafe { winuser::GetWindowRect(window_id, &mut rect) };

    let position = (rect.left, rect.top);
    let size = (rect.right - rect.left, rect.bottom - rect.top);
    crate::data::Rect::new(position, size)
}


fn enumerate_monitors() -> Vec<winuser::MONITORINFOEXW> {
    let mut monitors = Vec::<winuser::MONITORINFOEXW>::new();
    let userdata = &mut monitors as *mut _;

    let result = unsafe {
        winuser::EnumDisplayMonitors(
            std::ptr::null_mut(),
            std::ptr::null(),
            Some(enumerate_monitors_callback),
            userdata as minwindef::LPARAM,
        )
    };

    if result != minwindef::TRUE {
        // Get the last error for the current thread.
        // This is analogous to calling the Win32 API GetLastio::Error.
        panic!(
            "Could not enumerate monitors: {}",
            std::io::Error::last_os_error()
        );
    }

    monitors
}

unsafe extern "system" fn enumerate_monitors_callback(
    monitor: windef::HMONITOR,
    _: windef::HDC,
    _: windef::LPRECT,
    userdata: minwindef::LPARAM,
) -> minwindef::BOOL {
    let monitors: &mut Vec<winuser::MONITORINFOEXW> = mem::transmute(userdata);

    // Initialize the MONITORINFOEXW structure and get a pointer to it
    let mut monitor_info: winuser::MONITORINFOEXW = mem::zeroed();
    monitor_info.cbSize = mem::size_of::<winuser::MONITORINFOEXW>() as u32;
    let monitor_info_ptr = <*mut _>::cast(&mut monitor_info);

    if winuser::GetMonitorInfoW(monitor, monitor_info_ptr) == minwindef::TRUE {
        monitors.push(monitor_info);
    }

    minwindef::TRUE
}

pub fn get_quake_id() -> Option<windef::HWND> {
    let mut handle = std::ptr::null_mut::<windef::HWND__>();

    unsafe { winuser::EnumWindows(Some(find_quake), &mut handle as *mut _ as minwindef::LPARAM) };

    if handle.is_null() {
        return None;
    }

    Some(handle)
}

fn str_ptr(base: &'static str) -> *const i8 {
    format!("{base}\0").as_ptr() as *const i8
}

extern "system" fn find_quake(wnd: *mut windef::HWND__, lp: minwindef::LPARAM) -> minwindef::BOOL {
    // Keep in mind that `windef::HWND == *mut windef::HWND__`

    let pworker = unsafe { &mut *(lp as *mut windef::HWND) };

    unsafe {
        *pworker = winuser::FindWindowExA(
            std::ptr::null_mut(),
            wnd,
            str_ptr("CASCADIA_HOSTING_WINDOW_CLASS"),
            std::ptr::null(),
        )
    };

    if !(*pworker).is_null() {
        return minwindef::FALSE;
    }

    minwindef::TRUE
}
