use super::lib;
use chrono::NaiveDate;
use std::io::{Error, ErrorKind};

#[derive(Debug, PartialEq)]
pub struct WorkbookInfo {
    pub accounting_date: Vec<NaiveDate>,
    pub interest_date: Vec<NaiveDate>,
    pub archive_reference: Vec<String>,
    pub counter_account: Vec<String>,
    pub types: Vec<String>,
    pub text: Vec<String>,
    pub out_of_account: Vec<f64>,
    pub into_account: Vec<f64>,
    pub account: Vec<String>,
}

impl WorkbookInfo {
    pub fn new(
        accounting_date: Vec<NaiveDate>,
        interest_date: Vec<NaiveDate>,
        archive_reference: Vec<String>,
        counter_account: Vec<String>,
        types: Vec<String>,
        text: Vec<String>,
        out_of_account: Vec<f64>,
        into_account: Vec<f64>,
        account: Vec<String>,
    ) -> Result<Self, Error> {
        // check if all the vectors are equal length
        let date_vec = vec![&accounting_date, &interest_date];
        let string_vec = vec![&archive_reference, &counter_account, &types, &text];
        let f64_vec = vec![&out_of_account, &into_account];

        if !lib::all_same_length(&date_vec)
            || !lib::all_same_length(&string_vec)
            || !lib::all_same_length(&f64_vec)
        {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "different number of information",
            ));
        }
        if &date_vec[0].len() != &string_vec[0].len() || &date_vec[0].len() != &f64_vec[0].len() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "different number of information",
            ));
        }

        Ok(WorkbookInfo {
            accounting_date: accounting_date,
            interest_date: interest_date,
            archive_reference: archive_reference,
            counter_account: counter_account,
            types: types,
            text: text,
            out_of_account: out_of_account,
            into_account: into_account,
            account: account,
        })
    }
}

#[cfg(test)]
mod tests_excel_workbook {
    use super::*;

    #[test]
    fn test_new_woorkbook_info() {
        let accounting_date = vec![NaiveDate::parse_from_str("01.02.2022", "%d.%m.%Y").unwrap()];
        let interest_date = vec![NaiveDate::parse_from_str("01.02.2022", "%d.%m.%Y").unwrap()];
        let archive_reference = vec![String::from("1")];
        let counter_account = vec![String::from("counter")];
        let types = vec![String::from("type3")];
        // len of text vec is different
        let text = vec![String::from("text2")];
        let out_of_account = vec![3.0, 1.0];
        let into_account = vec![0.0];
        let account = vec![String::from("account")];

        let wb = WorkbookInfo::new(
            accounting_date,
            interest_date,
            archive_reference,
            counter_account,
            types,
            text,
            out_of_account,
            into_account,
            account,
        );

        assert!(wb.is_err());

        let accounting_date = vec![NaiveDate::parse_from_str("01.02.2022", "%d.%m.%Y").unwrap()];
        let interest_date = vec![NaiveDate::parse_from_str("01.02.2022", "%d.%m.%Y").unwrap()];
        let archive_reference = vec![String::from("1")];
        let counter_account = vec![String::from("counter")];
        let types = vec![String::from("type3")];
        // len of text vec is different
        let text = vec![String::from("text2")];
        let out_of_account = vec![3.0];
        let into_account = vec![0.0];
        let account = vec![String::from("account")];

        let wb = WorkbookInfo::new(
            accounting_date,
            interest_date,
            archive_reference,
            counter_account,
            types,
            text,
            out_of_account,
            into_account,
            account,
        );
        assert!(wb.is_ok());
    }
}
