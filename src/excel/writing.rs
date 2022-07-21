use chrono::Datelike;
use std::error;
use umya_spreadsheet::*;

use super::lib;
use super::reading;
use super::workbook;
use crate::accounting;
use crate::file;

pub fn write_to_workbook(
    workbook_path: &str,
    transaction_path: &str,
    bank: accounting::bank::Bank,
    account: &str,
    categories: &reading::Categories,
    date_delimiter: &str,
    date_month_style: &str,
    date_language: &str,
    date_capitalize: &bool,
) -> Result<(), Box<dyn error::Error>> {
    match reading::get_transactions(&transaction_path, bank, account) {
        Ok(transaction_info) => {
            let workbook_info = reading::get_workbook_transactions(&workbook_path)?;
            let transaction_info = lib::remove_duplicates(transaction_info, workbook_info);
            write(
                &workbook_path,
                transaction_info,
                categories,
                date_delimiter,
                date_month_style,
                date_language,
                date_capitalize,
            )
        }
        Err(e) => Err(e),
    }
}

pub fn write(
    path: &str,
    info: workbook::WorkbookInfo,
    categories: &reading::Categories,
    date_delimiter: &str,
    date_month_style: &str,
    date_language: &str,
    date_capitalize: &bool,
) -> Result<(), Box<dyn error::Error>> {
    let mut book =
        file::lib::open_file(path).map_err(|e| format!("could not open workbook: {:?}", e))?;
    let sheet = book
        .get_sheet_by_name_mut("Kontoutskrift")
        .map_err(|e| format!("could not open worksheet 'Kontoutskrift': {:?}", e))?;

    // start on the first empty line
    let mut row = reading::get_first_empty_line(sheet);
    for i in 0..info.accounting_date.len() {
        // accounting date
        let accounting_date_str = lib::date_to_string(info.accounting_date[i], date_delimiter);
        sheet
            .get_cell_mut(&(String::from("A") + &row.to_string()))
            .set_value(accounting_date_str);

        // interest date
        let interest_date_str = lib::date_to_string(info.interest_date[i], date_delimiter);
        sheet
            .get_cell_mut(&(String::from("B") + &row.to_string()))
            .set_value(interest_date_str);

        // archive reference
        sheet
            .get_cell_mut(&(String::from("C") + &row.to_string()))
            .set_value(&info.archive_reference[i]);

        // counter account
        sheet
            .get_cell_mut(&(String::from("D") + &row.to_string()))
            .set_value(&info.counter_account[i]);

        // type
        sheet
            .get_cell_mut(&(String::from("E") + &row.to_string()))
            .set_value(&info.types[i]);

        // text
        sheet
            .get_cell_mut(&(String::from("F") + &row.to_string()))
            .set_value(&info.text[i]);

        // out of account
        sheet
            .get_cell_mut(&(String::from("G") + &row.to_string()))
            .set_value(&info.out_of_account[i].to_string());

        // into account
        sheet
            .get_cell_mut(&(String::from("H") + &row.to_string()))
            .set_value(&info.into_account[i].to_string());

        // amount
        let amount = &info.into_account[i] - &info.out_of_account[i];
        sheet
            .get_cell_mut(&(String::from("I") + &row.to_string()))
            .set_value(amount.to_string());

        // account
        sheet
            .get_cell_mut(&(String::from("J") + &row.to_string()))
            .set_value(&info.account[i]);

        // group
        let category = lib::get_category(&info.text[i], &info.types[i], categories);
        sheet
            .get_cell_mut(&(String::from("K") + &row.to_string()))
            .set_value(category.0);

        // income/expense
        sheet
            .get_cell_mut(&(String::from("L") + &row.to_string()))
            .set_value(category.1);

        // year
        let year = info.accounting_date[i].format("%Y").to_string();
        sheet
            .get_cell_mut(&(String::from("M") + &row.to_string()))
            .set_value(year);

        // month
        let month = lib::get_month(
            info.accounting_date[i].month(),
            date_month_style,
            date_language,
            &date_capitalize,
        );
        sheet
            .get_cell_mut(&(String::from("N") + &row.to_string()))
            .set_value(month);
        row += 1;
    }

    let _ = writer::xlsx::write(&book, path);
    Ok(())
}

pub fn fill_empty_rows(path: &str, range: u32, margin: u32) -> Result<(), Box<dyn error::Error>> {
    let mut book =
        file::lib::open_file(path).map_err(|e| format!("could not open workbook: {:?}", e))?;
    let sheet = book
        .get_sheet_by_name_mut("Kontoutskrift")
        .map_err(|e| format!("could not open worksheet 'Kontoutskrift': {:?}", e))?;

    // Start on row 2
    let mut row = 1;
    loop {
        row += 1;
        // check if it is valid (accounting date is not empty), if empty, it is finished and break
        if sheet.get_value(&(String::from("A") + &row.to_string())) == "" {
            break;
        };
        // check if group is none, else continue
        if sheet.get_value(&(String::from("K") + &row.to_string())) != "" {
            continue;
        };

        let mut start: i32 = row as i32 - range as i32;
        // check if the start of the check is less than 1
        if start < 1 {
            start = 1;
        }
        let end = row as i32 + range as i32;

        let row_value = sheet
            .get_value(&(String::from("I") + &row.to_string()))
            .parse::<f32>()
            .map_err(|e| format!("row value is not a float: {:?}", e))?;
        for r in start..end {
            // check if it is valid (accounting date is not empty), if empty, it is finished and break
            if sheet.get_value(&(String::from("A") + &r.to_string())) == "" {
                break;
            };
            // check if group exist, else continue
            let group = sheet.get_value(&(String::from("K") + &r.to_string()));
            if group == "" {
                continue;
            };

            let r_value: f32 = match sheet
                .get_value(&(String::from("I") + &r.to_string()))
                .parse::<f32>()
            {
                Ok(value) => value,
                Err(_) => continue,
            };
            // compare it to -r_value, because want to get the transactions that close each other
            if row_value as f32 - margin as f32 <= -r_value
                && row_value as f32 + margin as f32 >= -r_value
            {
                sheet
                    .get_cell_mut(&(String::from("K") + &row.to_string()))
                    .set_value(group);
                let r_expense = sheet.get_value(&(String::from("L") + &r.to_string()));
                sheet
                    .get_cell_mut(&(String::from("L") + &row.to_string()))
                    .set_value(r_expense);
            };
        }
    }
    let _ = writer::xlsx::write(&book, path);
    Ok(())
}

pub fn re_group(path: &str, categories: &reading::Categories) -> Result<(), Box<dyn error::Error>> {
    let mut book =
        file::lib::open_file(path).map_err(|e| format!("could not open workbook: {:?}", e))?;
    let sheet = book
        .get_sheet_by_name_mut("Kontoutskrift")
        .map_err(|e| format!("could not open worksheet 'Kontoutskrift': {:?}", e))?;
    // start on row 2
    let mut row = 1;
    loop {
        row += 1;
        // check if it is valid (accounting date is not empty), if empty, it is finished and break
        if sheet.get_value(&(String::from("A") + &row.to_string())) == "" {
            break;
        };

        // if the row is modified (change group manually) it should either have a bold group text
        // or a text in the rightmost column (column 'O')
        if sheet.get_value(&(String::from("O") + &row.to_string())) != "" {
            break;
        };

        let group = sheet.get_value(&(String::from("K") + &row.to_string()));
        if group != "" {
            let is_bold = match sheet
                .get_style(&(String::from("K") + &row.to_string()))
                .get_font()
            {
                Some(font) => font.get_bold(),
                _ => &false, // this should not be an error if the text is bold...
            };
            if *is_bold {
                continue;
            }
        };

        let types = sheet.get_value(&(String::from("E") + &row.to_string()));
        let text = sheet.get_value(&(String::from("F") + &row.to_string()));

        let cat = lib::get_category(&text, &types, &categories);
        // if it does not have a category, skip this
        if cat.0 == "" || cat.1 == "" {
            continue;
        };

        sheet
            .get_cell_mut(&(String::from("K") + &row.to_string()))
            .set_value(cat.0);
        sheet
            .get_cell_mut(&(String::from("L") + &row.to_string()))
            .set_value(cat.1);
    }
    let _ = writer::xlsx::write(&book, path);
    Ok(())
}

pub fn re_date(
    path: &str,
    delimiter: &str,
    date_month_style: &str,
    date_language: &str,
    date_capitalize: &bool,
) -> Result<(), Box<dyn error::Error>> {
    let mut book =
        file::lib::open_file(path).map_err(|e| format!("could not open workbook: {:?}", e))?;
    let sheet = book
        .get_sheet_by_name_mut("Kontoutskrift")
        .map_err(|e| format!("could not open worksheet 'Kontoutskrift': {:?}", e))?;
    // start on row 2
    let mut row = 1;
    loop {
        row += 1;
        // check if it is valid (accounting date is not empty), if empty, it is finished and break
        if sheet.get_value(&(String::from("A") + &row.to_string())) == "" {
            break;
        };

        let accounting_date = sheet.get_value(&(String::from("A") + &row.to_string()));
        let cur_del = lib::get_delimiter(&accounting_date);
        let interest_date = sheet.get_value(&(String::from("B") + &row.to_string()));
        sheet
            .get_cell_mut(&(String::from("A") + &row.to_string()))
            .set_value(accounting_date.replace(&cur_del, delimiter));
        sheet
            .get_cell_mut(&(String::from("B") + &row.to_string()))
            .set_value(interest_date.replace(&cur_del, delimiter));

        match lib::string_to_date(&accounting_date, &lib::get_delimiter(&accounting_date)) {
            Ok(date) => {
                sheet
                    .get_cell_mut(&(String::from("N") + &row.to_string()))
                    .set_value(lib::get_month(
                        date.month(),
                        date_month_style,
                        date_language,
                        date_capitalize,
                    ));
            }
            Err(_) => (),
        };
    }
    let _ = writer::xlsx::write(&book, path);
    Ok(())
}

#[cfg(tests)]
mod writing_tests {
    use super::super::test_setup::initialize;
    use super::*;

    
}
