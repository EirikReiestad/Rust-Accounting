use super::workbook;
use super::reading;
use std::fs;
use std::sync::Once;

#[allow(dead_code)]
static INIT: Once = Once::new();

#[allow(dead_code)]
pub fn initialize() {
    INIT.call_once(|| {
        // initialization code here
        initialize_tests().unwrap();
    });
}

#[allow(dead_code)]
pub fn initialize_tests() -> Result<(), Box<dyn std::error::Error>> {
    // removes file if exists
    match fs::remove_file("test.xslx") {
        Ok(_) => (),
        Err(_) => (),
    };
    // copies the template and make it a test file
    fs::copy("rustAccounting/src/templates/template.xlsx", "test.xlsx")?;
    Ok(())
}

struct mock_transaction {
    path: String,
    info: workbook::WorkbookInfo,
    categories: reading::Categories,
    date_delimiter: String,
    date_month_style: String,
    date_language: String,
    date_capitalize: bool,
}

pub fn create_mock_transactions() -> mock_transaction {
    let path = "test.xlsx";
    let transaction_path = ""
}
