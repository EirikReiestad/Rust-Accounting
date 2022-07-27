use super::reading;
use super::workbook;
use chrono::NaiveDate;
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
    // copies the template and make it a test files
    fs::copy("src/templates/template.xlsx", "test.xlsx").map_err(|e| format!("could not copy file: {:?}", e))?;
    Ok(())
}

pub struct MockTransaction {
    pub path: String,
    pub info: workbook::WorkbookInfo,
    pub categories: reading::Categories,
    pub date_delimiter: String,
    pub date_month_style: String,
    pub date_language: String,
    pub date_capitalize: bool,
}

pub fn create_mock_transactions() -> Result<MockTransaction, Box<dyn std::error::Error>> {
    let path = String::from("test.xlsx");
    let size = 1000;
    let mut accounting_date = vec![];
    let mut interest_date = vec![];
    let mut archive_reference = vec![];
    let mut counter_account = vec![];
    let mut types = vec![];
    let mut text = vec![];
    let mut out_of_account = vec![];
    let mut into_account = vec![];
    let mut account = vec![];
    for i in 0..size {
        accounting_date.push(NaiveDate::from_num_days_from_ce(i));
        interest_date.push(NaiveDate::from_num_days_from_ce(i));
        archive_reference.push(String::from("0"));
        counter_account.push(String::from("1"));
        types.push(String::from("types"));
        text.push(String::from("text"));
        out_of_account.push(0.0);
        into_account.push(0.0);
        account.push(String::from("0"))
    }
    let info = workbook::WorkbookInfo::new(
        accounting_date,
        interest_date,
        archive_reference,
        counter_account,
        types,
        text,
        out_of_account,
        into_account,
        account,
    )?;

    let mock_transaction = MockTransaction {
        path: path,
        info: info,
        categories: reading::Categories::new(vec![], vec![]),
        date_delimiter: String::from("/"),
        date_month_style: String::from("short"),
        date_language: String::from("english"),
        date_capitalize: false,
    };
    Ok(mock_transaction)
}
