#![windows_subsystem = "windows"]

use winsafe::co::{self, LVS, WS};
use winsafe::prelude::*;
use winsafe::{gui, POINT, SIZE};

fn main() {
    let my = MyWindow::new(); // instantiate our main window
    if let Err(e) = my.wnd.run_main(None) {
        // ... and run it
        eprintln!("{}", e);
    }
}

#[derive(Clone)]
pub struct MyWindow {
    wnd: gui::WindowMain, // responsible for managing the window
    log: gui::ListView,   // a button
}

impl MyWindow {
    pub fn new() -> Self {
        let wnd = gui::WindowMain::new(
            // instantiate the window manager
            gui::WindowMainOpts {
                title: "My window title".to_owned(),
                class_style: co::CS::DBLCLKS | co::CS::VREDRAW | co::CS::HREDRAW,
                // class_bg_brush: gui::Brush::Color(co::COLOR::HIGHLIGHT),
                class_icon: gui::Icon::Id(1),
                style: WS::CAPTION
                    | WS::SYSMENU
                    | WS::CLIPCHILDREN
                    | WS::BORDER
                    | WS::VISIBLE
                    | co::WS::THICKFRAME
                    | co::WS::SIZEBOX,
                size: (400, 400),
                ..Default::default() // leave all other options as default
            },
        );

        let log = gui::ListView::new(
            &wnd,
            gui::ListViewOpts {
                list_view_style: LVS::REPORT,
                position: (0, 200),
                size: (400, 200),
                columns: vec![
                    ("date".into(), 100),
                    ("kind".into(), 100),
                    ("event".into(), 200),
                ],
                ..Default::default()
            },
        );

        let new_self = Self { wnd, log };
        new_self.events(); // attach our events
        new_self
    }

    fn events(&self) {}
}
