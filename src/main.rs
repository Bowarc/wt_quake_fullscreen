#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod data;
mod utils;

fn main() {
    /*

        Get the handle of the Wt window
            If you have multiple WindowsTerminal instances open, this may get the wrong window

        Detect window switching from hidden to visible

        Wait for drop down animation

        Set the window to full screen

    */

    let Some(handle) = utils::get_quake_id() else {
        panic!("Failled to get quake window id");
    };
    println!("Quake handle: {handle:?}");

    let screens = utils::get_screens();

    println!("screen list: {screens:?}");

    let mut hidden = false;

    loop {
        let temp = unsafe {
            // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowvisible
            winapi::um::winuser::IsWindowVisible(handle) == 0
        };

        if temp == hidden {
            // Limit cpu usage
            std::thread::sleep(std::time::Duration::from_millis(100)); 
            continue;
        }

        hidden = temp;

        if hidden {
            continue;
        }

        /*
            Window terminal has a drop down animation which we need to wait to set full screen

            We could wait for the quake window's rect.topleft.1 (y) to be positive but it would not work on some setup
                For example:
                    2 screen, vertical setup, the main one is the one below
                    The 2nd screen(the one above) will have a negative position
                    therefore the quake window will never have a positive y position on that screen
            A fix delay could work but idk how reliable it would be
        */
        loop {
            let win_rect = utils::get_window_pos_size(handle);

            if !screens.iter().any(|s| s.rect.contains(win_rect.center())) {
                continue;
            }

            break;
        }


        unsafe {
            // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
            // I added the first one because for some reason when having 0 drop down animation (https://github.com/microsoft/terminal/issues/16175#issuecomment-1764312749)
            // Even waiting for the window to be on a screen isn't enough
            // So we set the window to normal mode, then fullscreen (this should do nothing in most cases)
            winapi::um::winuser::ShowWindow(handle, 1);
            winapi::um::winuser::ShowWindow(handle, 3);
        }
    }
}
