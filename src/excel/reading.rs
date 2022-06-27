use crate::accounting;

use super::lib;
use super::workbook;
use crate::file;

use std::error;

#[derive(Default, Debug)]
pub struct Categories {
    pub from_text: Vec<Vec<String>>,
    pub from_type: Vec<Vec<String>>,
}

impl Categories {
    pub fn new(from_text: Vec<Vec<String>>, from_type: Vec<Vec<String>>) -> Self {
        // checks that all categories contain at least two elements
        let from_text = from_text.into_iter().filter(|vec| vec.len() >= 2).collect();
        let from_type = from_type.into_iter().filter(|vec| vec.len() >= 2).collect();

        Self {
            from_text: from_text,
            from_type: from_type,
        }
    }
}

pub fn get_categories(path: &str) -> Result<Categories, Box<dyn error::Error>> {
    let book =
        file::lib::open_file(path).map_err(|e| format!("could not open workbook: {:?}", e))?;
    let sheet = book
        .get_sheet_by_name("Kontoutskrift")
        .map_err(|e| format!("could not open worksheet 'Kontoutskrift': {:?}", e))?;

    let mut from_text = vec![];
    let mut from_type = vec![];

    let mut col = 1;
    let mut is_from_text = true;
    loop {
        let cat = sheet.get_value_by_column_and_row(&col, &2);
        // if value on row 2 in a column is empty, break (no more categories)
        if cat == "" {
            break;
        };

        let cat_type = sheet.get_value_by_column_and_row(&col, &3);
        // if cat_type does not exists (expense, income etc.) continue
        if cat_type == "" {
            continue;
        }

        // only checks this if is_from_text is true (has not changed yet)
        if is_from_text {
            let text_or_type = sheet.get_value_by_column_and_row(&col, &1);
            // if the text from the first row contains a word named type, change the status is_from_text to false
            if text_or_type.contains("type") {
                is_from_text = false;
            }
        }

        let mut col_vec = vec![];
        col_vec.push(cat);
        col_vec.push(cat_type);

        // the key-words begins on line 4
        let mut row = 4;
        loop {
            let value = sheet.get_value_by_column_and_row(&col, &row);
            if value == "" {
                break;
            }
            col_vec.push(value);
            row += 1;
        }

        if is_from_text {
            from_text.push(col_vec);
        } else {
            from_type.push(col_vec);
        }
        col += 1;
    }
    Ok(Categories::new(from_text, from_type))
}

// read from spreadsheet file
pub fn get_accounts(path: &str) -> Result<Vec<accounting::lib::Account>, Box<dyn error::Error>> {
    let book =
        file::lib::open_file(path).map_err(|e| format!("could not open workbook: {:?}", e))?;
    let sheet = book
        .get_sheet_by_name("Informasjon")
        .map_err(|e| format!("could not open worksheet 'Informasjon': {:?}", e))?;

    let mut accounts = vec![];
    // The account names are in the B file
    // The account number are in the C file
    // The first line is the header, start on line 2
    let mut line = 2;
    loop {
        let account_name = sheet.get_value(&(String::from("B") + &line.to_string()));
        let account_number = match sheet
            .get_value(&(String::from("C") + &line.to_string()))
            .parse::<u64>()
        {
            Ok(num) => num,
            // if the account_number is not valid, just break
            Err(_) => break,
        };
        if account_name == "" {
            break;
        }
        accounts.push(accounting::lib::Account {
            name: account_name,
            number: account_number,
        });
        line += 1;
    }
    Ok(accounts)
}

pub fn get_transactions(
    path: &str,
    bank: accounting::bank::Bank,
    account: &str,
) -> Result<workbook::WorkbookInfo, Box<dyn error::Error>> {
    match bank {
        accounting::bank::Bank::SBanken => {
            match accounting::bank::SBanken::get_transactions(path) {
                // want to reverse the data (plot the oldest data first)
                Ok(sbanken) => {
                    let length = sbanken.accounting_date.len();
                    Ok(workbook::WorkbookInfo::new(
                        sbanken.accounting_date.into_iter().rev().collect(),
                        sbanken.interest_date.into_iter().rev().collect(),
                        sbanken.archive_reference.into_iter().rev().collect(),
                        sbanken.counter_account.into_iter().rev().collect(),
                        sbanken.types.into_iter().rev().collect(),
                        sbanken.text.into_iter().rev().collect(),
                        sbanken.out_of_account.into_iter().rev().collect(),
                        sbanken.into_account.into_iter().rev().collect(),
                        vec![String::from(account); length],
                    )?)
                }
                Err(e) => Err(e),
            }
        }
    }
}

pub fn get_workbook_transactions(
    path: &str,
) -> Result<workbook::WorkbookInfo, Box<dyn error::Error>> {
    let book =
        file::lib::open_file(path).map_err(|e| format!("could not open workbook: {:?}", e))?;
    let sheet = book
        .get_sheet_by_name("Informasjon")
        .map_err(|e| format!("could not open worksheet 'Informasjon': {:?}", e))?;

    let mut accounting_date = vec![];
    let mut interest_date = vec![];
    let mut archive_reference = vec![];
    let mut counter_account = vec![];
    let mut types = vec![];
    let mut text = vec![];
    let mut out_of_account = vec![];
    let mut into_account = vec![];
    let mut account = vec![];

    // sbanken transactions start on excel line 4
    let mut row = 2;
    loop {
        // accounting date
        let accounting_date_str =
            &sheet.get_formatted_value(&(String::from("A") + &row.to_string()));
        // if accounting_date_str is empty, end of transactions
        if accounting_date_str == "" {
            break;
        }
        let delimiter = lib::get_delimiter(accounting_date_str);
        let s = lib::string_to_date(accounting_date_str, &delimiter)
            .map_err(|e| format!("accounting date is not valid: {:?}", e))?;
        accounting_date.push(s);

        // interest date
        let interest_date_str = &sheet.get_formatted_value(&(String::from("B") + &row.to_string()));
        let delimiter = lib::get_delimiter(accounting_date_str);
        let s = lib::string_to_date(interest_date_str, &delimiter)
            .map_err(|e| format!("interest date is not valid: {:?}", e))?;
        interest_date.push(s);

        archive_reference.push(sheet.get_value(&(String::from("C") + &row.to_string())));
        counter_account.push(sheet.get_value(&(String::from("D") + &row.to_string())));
        types.push(sheet.get_value(&(String::from("E") + &row.to_string())));
        text.push(sheet.get_value(&(String::from("F") + &row.to_string())));

        let out_of_account_str = sheet.get_value(&(String::from("G") + &row.to_string()));
        let f = out_of_account_str
            .parse::<f64>()
            .map_err(|e| format!("could not parse out of account string to f64: {:?}", e))?;
        out_of_account.push(f);

        let into_account_str = sheet.get_value(&(String::from("H") + &row.to_string()));
        let f = into_account_str.parse::<f64>().map_err(|e| format!("could not parse into account string to f64: {:?}", e))?;
        into_account.push(f);

        let account_str = sheet.get_value(&(String::from("J") + &row.to_string()));
        account.push(account_str);

        row += 1;
    }

    let workbook = workbook::WorkbookInfo::new(
        accounting_date,
        interest_date,
        archive_reference,
        counter_account,
        types,
        text,
        out_of_account,
        into_account,
        account)?;
    Ok(workbook)
}

pub fn get_first_empty_line(sheet: &mut umya_spreadsheet::Worksheet) -> usize {
    // checks the A file, assumes the rest of the slots also are empty if A row is empty
    let mut row = 1;
    loop {
        let value = sheet.get_value(&(String::from("A") + &row.to_string()));

        // if value is empty (row is empty)
        if value == "" {
            break;
        }
        row += 1;
    }
    row
}

#[cfg(test)]
mod tests_excel_reading {
    use super::*;

    #[test]
    fn test_categories() {
        let v1 = vec![vec![String::new(), String::new()], vec![String::new()]];
        let v2 = vec![vec![String::new(), String::new()], vec![]];

        let cat = Categories::new(v1, v2);
        assert_eq!(cat.from_text, vec![vec![String::new(), String::new()]]);
        assert_eq!(cat.from_type, vec![vec![String::new(), String::new()]]);
    }

    #[test]
    fn test_get_accounts() {
        assert_eq!(get_accounts("test").is_err(), true)
    }
}
