use eframe::egui;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

pub struct FileInformation {
    pub workbook_file: String,
    pub transaction_file: String,
    pub sheet_names: Vec<String>,
}

impl super::Window for FileInformation {
    fn name(&self) -> &'static str {
        "File Information"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .collapsible(true)
            .resizable(true)
            .open(open)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl super::View for FileInformation {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Sheet names: ");
        });
        egui::Grid::new("parent grid").striped(true).show(ui, |ui| {
            for sheet_name in &self.sheet_names {
                ui.horizontal(|ui| {
                    ui.label(format!("'{}'", sheet_name));
                });
                ui.end_row();
            }
        });
    }
}

impl Default for FileInformation {
    fn default() -> Self {
        Self {
            workbook_file: String::from(""),
            transaction_file: String::from(""),
            sheet_names: vec![
                String::from("Kontoutkskrift"),
                String::from("Kategorier"),
                String::from("Informasjon"),
            ],
        }
    }
}
