pub mod lib;
pub mod settings;
pub mod file_information;
pub mod workbook_information;

pub use eframe::egui;

// Something to view in the windows
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

// Something to view
pub trait Window {
    // `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    // Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
