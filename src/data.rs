#[derive(Debug)]
pub struct Monitor {
    pub name: String,
    pub rect: Rect,
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub topleft: (i32, i32),
    pub size: (i32, i32),
}

impl Monitor {
    pub fn from_info(name: String, info: winapi::um::winuser::MONITORINFOEXW) -> Self {
        Monitor {
            name: name.replace(['\\', '.'], ""), //.replace(['\\', '.'], "").replace("DISPLAY", "")
            rect: Rect::new(
                (info.rcMonitor.left, info.rcMonitor.top),
                (
                    info.rcMonitor.right - info.rcMonitor.left,
                    info.rcMonitor.bottom - info.rcMonitor.top,
                ),
            ),
        }
    }
}

impl Rect {
    pub fn new(topleft: (i32, i32), size: (i32, i32)) -> Self {
        Self { topleft, size }
    }

    pub fn center(&self) -> (i32, i32) {
        (
            self.topleft.0 + self.size.0 / 2,
            self.topleft.1 + self.size.1 / 2,
        )
    }

    pub fn contains(&self, point: (i32, i32)) -> bool {
        self.topleft.0 < point.0
            && point.0 < self.topleft.0 + self.size.0
            && self.topleft.1 < point.1
            && point.1 < self.topleft.1 + self.size.1
    }
}
