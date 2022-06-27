#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod accounting;
mod excel;
mod file;
mod settings;

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Accounting",
        options,
        Box::new(|_cc| Box::new(App::default())),
    );
}

struct App {
    settings: settings::settings::Settings,
}

impl Default for App {
    fn default() -> Self {
        Self {
            settings: settings::settings::Settings::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.settings.update(ctx);
    }
}
