use chrono::NaiveDate;
use std::error;

use crate::excel;
use crate::file;

#[derive(PartialEq, Copy, Clone)]
pub enum Bank {
    SBanken,
}

impl Bank {
    pub fn get_banks() -> Vec<Bank> {
        vec![Bank::SBanken]
    }

    pub fn to_string(bank: &Bank) -> String {
        match bank {
            Bank::SBanken => String::from("SBanken"),
        }
    }
}

#[derive(Debug)]
pub struct SBanken {
    pub accounting_date: Vec<NaiveDate>,
    pub interest_date: Vec<NaiveDate>,
    pub archive_reference: Vec<String>,
    pub counter_account: Vec<String>,
    pub types: Vec<String>,
    pub text: Vec<String>,
    pub out_of_account: Vec<f64>,
    pub into_account: Vec<f64>,
}

impl SBanken {
    pub fn get_transactions(path: &str) -> Result<Self, Box<dyn error::Error>> {
        let book =
            file::lib::open_file(path).map_err(|e| format!("could not open workbook: {:?}", e))?;
        let sheet = book
            .get_sheet_by_name("Kontoutskrift")
            .map_err(|e| format!("could not open worksheet 'Kontoutskrift': {:?}", e))?;

        let mut accounting_date = vec![];
        let mut interest_date = vec![];
        let mut archive_reference = vec![];
        let mut counter_account = vec![];
        let mut types = vec![];
        let mut text = vec![];
        let mut out_of_account = vec![];
        let mut into_account = vec![];

        // sbanken transactions start on excel line 4
        let mut row = 4;
        loop {
            // accounting date
            let accounting_date_str =
                &sheet.get_formatted_value(&(String::from("A") + &row.to_string()));
            // if accounting_date_str is empty, end of transactions
            if accounting_date_str == "" {
                break;
            }
            let s = SBanken::string_to_date(accounting_date_str).map_err(|e| {
                format!(
                    "could not convert accounting date string to date for sbanken transaction: {:?}",
                    e
                )
            })?;
            accounting_date.push(s);

            // interest date
            let interest_date_str =
                &sheet.get_formatted_value(&(String::from("B") + &row.to_string()));
            let s = SBanken::string_to_date(interest_date_str).map_err(|e| {
                format!(
                    "could not convert interest date string to date for sbanken transaction: {:?}",
                    e
                )
            })?;
            interest_date.push(s);

            archive_reference.push(sheet.get_value(&(String::from("C") + &row.to_string())));
            counter_account.push(sheet.get_value(&(String::from("D") + &row.to_string())));
            types.push(sheet.get_value(&(String::from("E") + &row.to_string())));
            text.push(sheet.get_value(&(String::from("F") + &row.to_string())));

            let out_of_account_str = sheet.get_value(&(String::from("G") + &row.to_string()));
            match out_of_account_str.parse::<f64>() {
                Ok(f) => out_of_account.push(f.abs()),
                Err(_) => return Err("could not parse out of account string to f64".into()),
            }

            let into_account_str = sheet.get_value(&(String::from("H") + &row.to_string()));
            match into_account_str.parse::<f64>() {
                Ok(f) => into_account.push(f),
                Err(_) => return Err("could not parse into account string to f64".into()),
            }

            row += 1;
        }

        let sbanken = SBanken {
            accounting_date: accounting_date,
            interest_date: interest_date,
            archive_reference: archive_reference,
            counter_account: counter_account,
            types: types,
            text: text,
            out_of_account: out_of_account,
            into_account: into_account,
        };
        Ok(sbanken)
    }

    pub fn string_to_date(date: &str) -> Result<NaiveDate, Box<dyn error::Error>> {
        excel::lib::string_to_date(date, ".").map_err(|e| format!("date str is not valid: {:?}", e))
    }
}

#[cfg(test)]
mod tests_bank {
    use super::*;

    #[test]
    fn test_string_to_date_sbanken() {
        let date = SBanken::string_to_date("03.06.2022");
        assert!(date.is_ok());
        let date = SBanken::string_to_date("fail");
        assert!(date.is_err());
    }
}
