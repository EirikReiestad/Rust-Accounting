use eframe::egui;
use egui::{color::*, ComboBox, RichText};
use rfd;

use super::file_information::FileInformation;
use super::lib;
use super::workbook_information::WorkbookInformation;
use super::Window;
use crate::accounting;
use crate::excel;
use crate::file;

#[derive(PartialEq)]
pub enum SettingsError {
    None,
    ValidFilename,
    ValidAccount,
}

pub struct Settings {
    // windows
    file_information_active: bool,
    workbook_information_active: bool,
    file_information: FileInformation,
    workbook_information: WorkbookInformation,

    // insert
    insert_cb: bool,
    // fill
    fill_cb: bool,
    fill_range: u32,
    fill_margin: u32,
    // regroup
    re_group_cb: bool,
    // redate
    re_date_cb: bool,
    // date
    date_delimiter: Vec<String>,
    date_delimiter_active: String,
    date_month_style: Vec<String>,
    date_month_style_active: String,
    date_language: Vec<String>,
    date_language_active: String,
    date_capitalize: bool,
    // bank
    bank: accounting::bank::Bank,
    // error/status
    flash_error: String,
    flash_ok: String,
    error: SettingsError,
}

impl Default for Settings {
    fn default() -> Self {
        let file_information = FileInformation::default();
        let path = String::new();

        let mut workbook_information = WorkbookInformation::default();
        workbook_information.init(&path);

        Settings {
            file_information_active: false,
            workbook_information_active: false,

            file_information: file_information,
            workbook_information: workbook_information,
            insert_cb: false,
            fill_cb: false,
            fill_range: 10,
            fill_margin: 5,
            re_group_cb: false,
            re_date_cb: false,
            date_delimiter: vec![
                (String::from(".")),
                (String::from("-")),
                (String::from("/")),
            ],
            date_delimiter_active: String::from("/"),
            date_month_style: vec![(String::from("short")), (String::from("long"))],
            date_month_style_active: String::from("short"),
            date_language: vec![String::from("norsk"), String::from("english")],
            date_language_active: String::from("norsk"),
            date_capitalize: false,
            bank: accounting::bank::Bank::SBanken,
            flash_error: String::new(),
            flash_ok: String::new(),
            error: SettingsError::None,
        }
    }
}

impl Settings {
    pub fn update(&mut self, ctx: &egui::Context) {
        self.error = SettingsError::None;
        // saves the current path to compare if the new has changed
        let prev_workbook_path = String::from(&self.file_information.workbook_file);
        let prev_transaction_path = String::from(&self.file_information.transaction_file);

        egui::CentralPanel::default().show(ctx, |ui| {
            // <----- INPUT ----->
            ui.heading("Automatic Accounting");
            ui.horizontal(|ui| {
                if ui.button("workbook file").clicked() {
                    match rfd::FileDialog::new().add_filter("workbook_file", &["xlsx"]).pick_file() {
                        Some(path) => match path.to_str() {
                            Some(p) => self.file_information.workbook_file = String::from(p),
                            _ => (),
                        }
                        _ => (),
                    };
                }
                ui.label(&self.file_information.workbook_file);
                if file::lib::valid_file(&self.file_information.workbook_file).is_err() {
                    self.error = SettingsError::ValidFilename;
                };
            });
            if self.insert_cb {
            ui.horizontal(|ui| {
                if ui.button("transaction file").clicked() {
                    match rfd::FileDialog::new().add_filter("transaction_file", &["xlsx"]).pick_file() {
                        Some(path) => match path.to_str() {
                            Some(p) => self.file_information.transaction_file= String::from(p),
                            _ => (),
                        }
                        _ => (),
                    }
                }
                ui.label(&self.file_information.transaction_file);
                if file::lib::valid_file(&self.file_information.transaction_file).is_err() {
                    self.error = SettingsError::ValidFilename;
                };
            });
            }
            ui.add_space(8.0);

            // <----- BUTTONS TO WINDOW ----->
            ui.horizontal(|ui| {
                if ui.button("Account information").clicked() {
                    self.workbook_information_active = true;

                    // only updates if it needs to
                    if !self.workbook_information.updated {
                        let path = file::lib::valid_file(
                            &self.file_information.workbook_file,
                        );
                        // update the account information
                        match path {
                            Ok(path) => self.workbook_information.update_all_workbook_information(&path),
                            Err(_) => {
                                self.error = SettingsError::ValidFilename;
                                self.workbook_information.reset_accounts()
                            }
                        }
                    }
                };
            });

            ui.horizontal(|ui| {
                if ui.button("File information").clicked() {
                    self.file_information_active = true;
                };
            });
            // adding space between different sections
            ui.add_space(8.0);

            // <----- COMBOBOX ----->
            // only show if insert is enabled
            if self.insert_cb {
                // combobox, choose bank
                ui.vertical(|ui| {
                    ComboBox::from_label("Bank")
                        .selected_text(accounting::bank::Bank::to_string(&self.bank))
                        .show_ui(ui, |ui| {
                            for bank in accounting::bank::Bank::get_banks() {
                                ui.selectable_value(
                                    &mut self.bank,
                                    bank,
                                    accounting::bank::Bank::to_string(&bank),
                                );
                            }
                        })
                });
                // combobox, choose account
                ui.vertical(|ui| {
                    ComboBox::from_label("Account")
                        .selected_text(&self.workbook_information.active_account.name)
                        .show_ui(ui, |ui| {
                            for account in self.workbook_information.get_accounts() {
                                let account_string =
                                    accounting::lib::Account::clone(&account).to_string();
                                ui.selectable_value(
                                    &mut self.workbook_information.active_account,
                                    account,
                                    account_string,
                                );
                            }
                        })
                });
                if !self.workbook_information.updated {
                    let path = file::lib::valid_file(
                        &self.file_information.workbook_file,
                    );
                    // update the account information
                    match path {
                        Ok(path) => self.workbook_information.update_all_workbook_information(&path),
                        Err(_) => {
                            self.error = SettingsError::ValidFilename;
                            self.workbook_information.reset_accounts()
                        }
                    }
                }
                // if no account is selected
                // if self.workbook_information.active_account == ""
                if !self.workbook_information.active_account.is_valid() {
                    self.error = SettingsError::ValidAccount;
                }

                // adding space between different sections
                ui.add_space(8.0);
            }

            if self.insert_cb || self.re_date_cb {
                // combobox, choose date format
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ComboBox::from_label("Date delimiter")
                        .selected_text(&self.date_delimiter_active)
                        .show_ui(ui, |ui| {
                            for delimiter in self.date_delimiter.clone() {
                                let delimiter_string = String::from(&delimiter);
                                ui.selectable_value(
                                    &mut self.date_delimiter_active,
                                    delimiter,
                                    delimiter_string,
                                );
                            }
                        })
                });
                ui.horizontal(|ui| {
                    ComboBox::from_label("Month style")
                        .selected_text(&self.date_month_style_active)
                        .show_ui(ui, |ui| {
                            for style in self.date_month_style.clone() {
                                let style_string = String::from(&style);
                                ui.selectable_value(
                                    &mut self.date_month_style_active,
                                    style,
                                    style_string,
                                );
                            }
                        });
                    ComboBox::from_label("Month language")
                        .selected_text(&self.date_language_active)
                        .show_ui(ui, |ui| {
                            for language in self.date_language.clone() {
                                let language_string = String::from(&language);
                                ui.selectable_value(
                                    &mut self.date_language_active,
                                    language,
                                    language_string,
                                );
                            }
                        });

                    ui.checkbox(&mut self.date_capitalize, "Capitalize");
                });
                ui.label(format!("{} | {}", 
                    "01.01.2021".replace(".", &self.date_delimiter_active), 
                    excel::lib::get_month(1, &self.date_month_style_active, &self.date_language_active, &self.date_capitalize)));
            };

            // <----- CHECKBOX ----->
            ui.checkbox(&mut self.insert_cb, "insert");
            ui.label("When enabled, the program will insert new data from the file location");
            ui.add_space(8.0);

            ui.checkbox(&mut self.fill_cb, "fill");
            ui.label("When enabled, the program will try to fill the unfilled categories");
            if self.fill_cb {
                ui.add(egui::Slider::new(&mut self.fill_range, 1..=20).text("range"));
                ui.label("Specify how many of the surrounding transaction that should be compared");
                ui.add(egui::Slider::new(&mut self.fill_margin, 0..=20).text("margin"));
                ui.label("Specify the amount it can differ");
            };
            ui.add_space(8.0);

            ui.checkbox(&mut self.re_group_cb, "re group");
            ui.label("When enabled, the program will re-group the transactions.");
            if self.re_group_cb {
                ui.label("To store some customized group changes to a transaction,");
                ui.label("make the style for cell in column K bold or make a note in the cell in column O.");
            }
            ui.add_space(8.0);

            ui.checkbox(&mut self.re_date_cb, "re date");
            ui.label("When enabled, the program will change the date format");
            ui.add_space(8.0);

            // <----- FLASH MESSAGE ----->
            ui.vertical_centered(|ui| {
                let mut error = &String::from(self.get_error());
                if error == "" {
                    error = &self.flash_error;
                }
                if error == "" {
                    ui.label(
                        RichText::new(format!("{}", self.flash_ok))
                            .color(Color32::from_rgb(50, 255, 50)),
                    );
                } else {
                    // reset flash ok if it displays an error
                    self.flash_ok = String::new();
                    ui.label(
                        RichText::new(format!("{}", error)).color(Color32::from_rgb(255, 50, 50)),
                    );
                }
            });

            // <----- RUN BUTTON ----->
            ui.vertical_centered(|ui| {
                if ui.button("update").clicked() {
                    // reset flash ok if update button clicked
                    self.flash_ok = String::new();
                    if self.error == SettingsError::None {
                        // update the account information
                        let path = file::lib::valid_file(
                            &self.file_information.workbook_file,
                        );
                        match path {
                            Ok(path) => self.workbook_information.update_categories(&path),
                            Err(_) => {
                                self.error = SettingsError::ValidFilename;
                                self.workbook_information.reset_accounts()
                            }
                        };
                        // if insert checkbox is set
                        if self.insert_cb {
                            let workbook_path = file::lib::valid_file(
                                &self.file_information.workbook_file,
                            );
                            let transaction_path = file::lib::valid_file(
                                &self.file_information.transaction_file,
                            );
                            match workbook_path {
                                Ok(wp) => match transaction_path {
                                    Ok(tp) => {
                                        match excel::writing::write_to_workbook(
                                            &wp,
                                            &tp,
                                            self.bank,
                                            &self.workbook_information.active_account.name,
                                            &self.workbook_information.categories,
                                            &self.date_delimiter_active,
                                            &self.date_month_style_active,
                                            &self.date_language_active,
                                            &self.date_capitalize,
                                        ) {
                                            Ok(_) => {
                                                self.flash_ok =
                                                    String::from("Successfully written to workbook")
                                            }
                                            Err(e) => self.flash_error = lib::get_flash_error(e),
                                        }
                                    }
                                    Err(e) => self.flash_error = lib::get_flash_error(e),
                                },
                                Err(e) => self.flash_error = lib::get_flash_error(e),
                            };
                        };

                        // if fille checkbox is set
                        if self.fill_cb {
                            let workbook_path = file::lib::valid_file(
                                &self.file_information.workbook_file,
                            );
                            match workbook_path {
                                Ok(wp) => match excel::writing::fill_empty_rows(
                                    &wp,
                                    self.fill_range,
                                    self.fill_margin,
                                ) {
                                    Ok(_) => {
                                        self.flash_ok =
                                            String::from("Successfully filled empty groups")
                                    }
                                    Err(e) => self.flash_error = lib::get_flash_error(e),
                                },
                                Err(e) => self.flash_error = lib::get_flash_error(e),
                            }
                        }

                        if self.re_group_cb {
                            let workbook_path = file::lib::valid_file(
                                &self.file_information.workbook_file,
                            );
                            match workbook_path {
                                Ok(wp) => match excel::writing::re_group(
                                    &wp,
                                    &self.workbook_information.categories,
                                ) {
                                    Ok(_) => {
                                        self.flash_ok = String::from("Successfully re-grouped")
                                    }
                                    Err(e) => self.flash_error = lib::get_flash_error(e),
                                },
                                Err(e) => self.flash_error = lib::get_flash_error(e),
                            }
                        }

                        if self.re_date_cb {
                            let workbook_path = file::lib::valid_file(
                                &self.file_information.workbook_file,
                            );
                            match workbook_path {
                                Ok(wp) => match excel::writing::re_date(
                                    &wp,
                                    &self.date_delimiter_active,
                                    &self.date_month_style_active,
                                    &self.date_language_active,
                                    &self.date_capitalize,
                                ) {
                                    Ok(_) => {
                                        self.flash_ok = String::from("Successfully re-dated")
                                    }
                                    Err(e) => self.flash_error = lib::get_flash_error(e),
                                },
                                Err(e) => self.flash_error = lib::get_flash_error(e),
                            }
                        }
                    };
                };
            });
        });

        // <----- SHOW WINDOWS ----->
        // show account information window
        self.workbook_information
            .show(ctx, &mut self.workbook_information_active);

        // show file information window
        self.file_information
            .show(ctx, &mut self.file_information_active);

        // <----- CHECKS ----->
        // if the path has been updated
        match lib::same_path(&prev_workbook_path, &self.file_information.workbook_file) {
            Ok(value) => {
                if !value {
                    self.workbook_information.updated = false;
                    // resets flash message
                    self.flash_error = String::new();
                }
            }
            Err(_) => {
                // if an error occurs, set updated to false
                // check if accounts vector is empty
                if self.workbook_information.accounts.len() > 0 {
                    self.workbook_information.updated = false;
                }
            }
        };

        match lib::same_path(
            &prev_transaction_path,
            &self.file_information.transaction_file,
        ) {
            Ok(value) => {
                if !value {
                    // resets flash message
                    self.flash_error = String::new();
                }
            }
            Err(_) => (),
        };
    }

    pub fn get_error(&self) -> &str {
        match self.error {
            SettingsError::ValidFilename => "That file do not exist in that directory",
            SettingsError::ValidAccount => "Need to choose an account",
            SettingsError::None => "",
        }
    }
}

#[cfg(test)]
mod test_workbook_information {
    use super::*;

    #[test]
    fn test_reset_accounts() {
        let mut info = WorkbookInformation::default();
        info.init("");
        info.accounts = vec![accounting::lib::Account {
            name: String::from("name"),
            number: 0,
        }];
        info.reset_accounts();
        assert_eq!(info.accounts.len(), 0);
    }
}
