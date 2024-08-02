use crate::hooks::mouse_move::hook_set::Hook;
use lazy_static::lazy_static;
use std::{ thread::{self, JoinHandle}};
use windows::Win32::{Foundation::{HWND, LPARAM}, UI::WindowsAndMessaging::HHOOK};
mod hooks;
use eframe::egui::{self, mutex::RwLock, RichText};
struct MousePosition {
    x: u16,
    y: u16,
}
impl Default for MousePosition {
    fn default() -> Self {
        MousePosition { x: 0, y: 0 }
    }
}
lazy_static! {
    static ref Points: RwLock<MousePosition> = RwLock::new(MousePosition::default());
}

fn start_hook_thread() -> JoinHandle<()> {
    return thread::spawn(|| unsafe {
        let mut hook: Hook = Hook::default();
        let _hhok = Hook::create_hook(&mut hook, LPARAM::default());
        Hook::get_messages(&mut hook, HWND::default(), 0, 0);
    });
}

fn main() {
    start_hook_thread();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([200.0, 100.0]).with_resizable(false).with_taskbar(false).with_title_shown(false),
        ..Default::default()
    };
    eframe::run_native(
        "TS",
        options,
        Box::new(|_cc| {
            return Ok(Box::<MyApp>::default());
        }),
    )
    .unwrap();
}

struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let point_x = format!("Point x: {}", Points.read().x);
                let point_y = format!("Point y: {}", Points.read().y);
                ui.label(RichText::new(point_x));
                ui.label(RichText::new(point_y));
            });
        });
    }
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        std::process::exit(0); // todo normalize unregister hook
    }
}
