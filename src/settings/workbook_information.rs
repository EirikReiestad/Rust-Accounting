use crate::accounting;
use crate::excel;
use eframe::egui;
use egui::{Color32, RichText};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]

pub struct WorkbookInformation {
    pub accounts: Vec<accounting::lib::Account>,
    pub active_account: accounting::lib::Account,
    pub categories: excel::reading::Categories,

    // check if the account information is newly updated, else it will fetch it again
    // e.g. if the filename or directory changes
    pub updated: bool,
}

impl super::Window for WorkbookInformation {
    fn name(&self) -> &'static str {
        "Workbook Information"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .collapsible(true)
            .resizable(true)
            .open(open)
            .hscroll(true)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl super::View for WorkbookInformation {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Account details");
        ui.collapsing("accounts", |ui| {
            ui.horizontal(|ui| {
                ui.heading("Accounts: ");
            });
            egui::Grid::new("parent grid").striped(true).show(ui, |ui| {
                for account in &self.accounts {
                    ui.horizontal(|ui| ui.label(account.to_string()));
                    ui.end_row();
                }
            });
        });
        ui.heading("Categories");
        ui.collapsing("from text", |ui| {
            egui::Grid::new("from text").striped(true).show(ui, |ui| {
                // begins on the third element, because the headings are first
                for cat in &self.categories.from_text {
                    ui.vertical(|ui| {
                        // cat should contain at least two elements
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(cat[0].to_string())
                                    .color(Color32::from_rgb(100, 100, 255)),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(cat[1].to_string())
                                    .color(Color32::from_rgb(200, 50, 255)),
                            );
                        });
                        for row in 2..cat.len() {
                            ui.horizontal(|ui| ui.label(cat[row].to_string()));
                            ui.end_row();
                        }
                    });
                    ui.add(egui::Separator::default().vertical());
                }
            });
        });
        ui.collapsing("from type", |ui| {
            egui::Grid::new("from type").striped(true).show(ui, |ui| {
                // begins on the third element, because the headings are first
                for cat in &self.categories.from_type {
                    ui.vertical(|ui| {
                        // cat should contain at least two elements
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(cat[0].to_string())
                                    .color(Color32::from_rgb(100, 100, 255)),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(cat[1].to_string())
                                    .color(Color32::from_rgb(200, 50, 255)),
                            );
                        });
                        for row in 2..cat.len() {
                            ui.horizontal(|ui| ui.label(cat[row].to_string()));
                            ui.end_row();
                        }
                    });
                    ui.add(egui::Separator::default().vertical().spacing(1.0));
                }
            });
        });
    }
}

impl Default for WorkbookInformation {
    fn default() -> Self {
        let accs = vec![];
        let active_account = accounting::lib::Account {
            name: String::new(),
            number: 0,
        };

        let categories = excel::reading::Categories {
            from_text: vec![],
            from_type: vec![],
        };

        Self {
            accounts: accs,
            active_account: active_account,
            categories: categories,
            updated: false,
        }
    }
}

impl WorkbookInformation {
    pub fn init(&mut self, path: &str) {
        self.update_all_workbook_information(path)
    }

    pub fn update_all_workbook_information(&mut self, path: &str) {
        self.update_accounts(path);
        self.update_categories(path);
        self.updated = true;
    }

    pub fn update_categories(&mut self, path: &str) {
        let mut categories = excel::reading::Categories::new(vec![], vec![]);
        match excel::reading::get_categories(path) {
            Ok(cat) => categories = cat,
            Err(_) => (),
        };
        self.categories = categories;
    }

    fn update_accounts(&mut self, path: &str) {
        let mut accounts = vec![];
        match excel::reading::get_accounts(path) {
            Ok(accs) => accounts = accs,
            Err(_) => (),
        };
        if accounts.len() >= 1 {
            self.active_account = accounting::lib::Account::clone(&accounts[0]);
        }
        self.accounts = accounts;
    }

    pub fn reset_accounts(&mut self) {
        self.active_account = accounting::lib::Account {
            name: String::new(),
            number: 0,
        };
        self.accounts = vec![];
    }

    pub fn get_accounts(&self) -> Vec<accounting::lib::Account> {
        let mut res = vec![];
        for account in &self.accounts {
            res.push(accounting::lib::Account {
                name: account.name.clone(),
                number: account.number.clone(),
            })
        }
        res
    }
}
