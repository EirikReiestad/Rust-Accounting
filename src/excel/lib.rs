use super::reading;
use super::workbook;
use chrono::NaiveDate;
use std::error;

pub fn get_month(
    date_nr: u32,
    date_month_style: &str,
    date_language: &str,
    date_capitalize: &bool,
) -> String {
    let english = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let norwegian = vec![
        "Januar",
        "Februar",
        "Mars",
        "April",
        "Mai",
        "Juni",
        "Juli",
        "August",
        "September",
        "Oktober",
        "November",
        "Desember",
    ];

    let mut month: String;
    month = match date_language {
        "norsk" => String::from(norwegian[date_nr as usize - 1]),
        "english" => String::from(english[date_nr as usize - 1]),
        _ => String::from(norwegian[date_nr as usize]),
    };

    month = match date_month_style {
        "short" => month.chars().take(3).collect(),
        "long" => month,
        _ => month,
    };
    if !*date_capitalize {
        month = month.to_lowercase();
    };
    String::from(month)
}

pub fn get_delimiter(date: &str) -> String {
    // skips the first two numbers and take the delimiter (should be in the third position)
    date.chars().skip(2).take(1).collect()
}

pub fn date_to_string(date: NaiveDate, delimiter: &str) -> String {
    date.format("%d.%m.%Y").to_string().replace(".", delimiter)
}

pub fn string_to_date(date: &str, delimiter: &str) -> Result<NaiveDate, Box<dyn error::Error>> {
    let format = format!("%d{d}%m{d}%Y", d = delimiter);
    Ok(NaiveDate::parse_from_str(date, &format)
        .map_err(|e| format!("date str is not valid: {:?}", e))?)
}

// checks if all vectors are the same length
pub fn all_same_length<T>(vec: &Vec<&Vec<T>>) -> bool {
    vec.iter().all(|ref v| v.len() == vec[0].len())
}

pub fn remove_duplicates(
    wb1: workbook::WorkbookInfo,
    wb2: workbook::WorkbookInfo,
) -> workbook::WorkbookInfo {
    // Want to return wb1 - wb2
    let mut wb = workbook::WorkbookInfo {
        accounting_date: vec![],
        interest_date: vec![],
        archive_reference: vec![],
        counter_account: vec![],
        types: vec![],
        text: vec![],
        out_of_account: vec![],
        into_account: vec![],
        account: vec![],
    };

    for i in 0..wb1.accounting_date.len() {
        let mut same = false;
        for j in 0..wb2.accounting_date.len() {
            // check if the attributes are the same!
            // could replace all if with one if and 'or' (||)
            if wb1.account[i] == wb2.account[j]
                && wb1.accounting_date[i] == wb2.accounting_date[j]
                && wb1.interest_date[i] == wb2.interest_date[j]
                && wb1.archive_reference[i] == wb2.archive_reference[j]
                && wb1.counter_account[i] == wb2.counter_account[j]
                && wb1.types[i] == wb2.types[j]
                && wb1.text[i] == wb2.text[j]
                && wb1.out_of_account[i] == wb2.out_of_account[j]
                && wb1.into_account[i] == wb2.into_account[j]
            {
                same = true;
                break;
            };
        }
        if !same {
            wb.accounting_date.push(wb1.accounting_date[i]);
            wb.interest_date.push(wb1.interest_date[i]);
            wb.archive_reference.push(wb1.archive_reference[i].clone());
            wb.counter_account.push(wb1.counter_account[i].clone());
            wb.types.push(wb1.types[i].clone());
            wb.text.push(wb1.text[i].clone());
            wb.out_of_account.push(wb1.out_of_account[i]);
            wb.into_account.push(wb1.into_account[i]);
            wb.account.push(wb1.account[i].clone());
        }
    }
    wb
}

pub fn get_category(text: &str, types: &str, categories: &reading::Categories) -> (String, String) {
    for cat in &categories.from_type {
        if types.to_lowercase().contains(&cat[0].to_lowercase()) {
            return (String::from(&cat[0]), String::from(&cat[1]));
        };
    }
    for cat in &categories.from_text {
        for i in 2..cat.len() {
            if text.to_lowercase().contains(&cat[i].to_lowercase()) {
                return (String::from(&cat[0]), String::from(&cat[1]));
            }
        }
    }
    (String::new(), String::new())
}

#[cfg(test)]
mod tests_excel_lib {
    use super::*;

    #[test]
    fn test_get_date() {
        assert_eq!(get_month(12, "short", "norsk", &true), "Des");
        assert_eq!(get_month(12, "long", "norsk", &true), "Desember");
        assert_eq!(get_month(12, "long", "norsk", &false), "desember");
        assert_eq!(get_month(5, "short", "english", &true), "May");
        assert_eq!(get_month(5, "long", "english", &false), "may");
        assert_eq!(get_month(3, "long", "english", &false), "march");
    }

    #[test]
    fn test_get_delimiter() {
        let date = "03.06.2022";
        assert_eq!(get_delimiter(date), ".");
        let date = "03/06/2022";
        assert_eq!(get_delimiter(date), "/");
    }

    #[test]
    fn test_string_to_date() {
        let date = string_to_date("03.06.2022", ".");
        assert!(date.is_ok());
        let date = string_to_date("03/06/2022", "/");
        assert!(date.is_ok());
        let date = string_to_date("fail", ".");
        assert!(date.is_err());
    }

    #[test]
    fn test_get_category_from_text() {
        let from_text = "this is a test";
        let from_types = "test";
        let texts = vec![
            vec![
                String::from("row1"),
                String::from("row2"),
                String::from("no"),
            ],
            vec![
                String::from("row3"),
                String::from("row4"),
                String::from("test"),
            ],
        ];
        let types = vec![
            vec![String::from("row1"), String::from("row2")],
            vec![String::from("test"), String::from("row4")],
        ];
        let categories = reading::Categories::new(texts, types);
        assert_eq!(
            get_category(from_text, from_types, &categories),
            (String::from("test"), String::from("row4"))
        );
    }

    #[test]
    fn test_all_same_length() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![1, 2, 3, 4];
        let vec = vec![&vec1, &vec2, &vec1];
        assert_eq!(all_same_length(&vec), false);
        let vec = vec![&vec1, &vec1, &vec1];
        assert_eq!(all_same_length(&vec), true);
    }

    #[test]
    fn test_date_to_string() {
        let date = NaiveDate::parse_from_str("1.1.2022", "%d.%m.%Y").unwrap();
        assert_eq!(date_to_string(date, "."), "01.01.2022");
        let date = NaiveDate::parse_from_str("1/1/2022", "%d/%m/%Y").unwrap();
        assert_eq!(date_to_string(date, "/"), "01/01/2022");
    }

    #[test]
    fn test_remove_duplicates() {
        let accounting_date = vec![
            NaiveDate::parse_from_str("01.01.2022", "%d.%m.%Y").unwrap(),
            NaiveDate::parse_from_str("01.02.2022", "%d.%m.%Y").unwrap(),
            NaiveDate::parse_from_str("01.02.2022", "%d.%m.%Y").unwrap(),
        ];
        let interest_date = vec![
            NaiveDate::parse_from_str("01.01.2022", "%d.%m.%Y").unwrap(),
            NaiveDate::parse_from_str("01.02.2022", "%d.%m.%Y").unwrap(),
            NaiveDate::parse_from_str("01.02.2022", "%d.%m.%Y").unwrap(),
        ];
        let archive_reference = vec![String::from("1"), String::from("1"), String::from("1")];
        let counter_account = vec![
            String::from("counter"),
            String::from("counter"),
            String::from("counter"),
        ];
        let types = vec![
            String::from("type1"),
            String::from("type2"),
            String::from("type3"),
        ];
        let text = vec![
            String::from("text1"),
            String::from("text2"),
            String::from("text3"),
        ];
        let out_of_account = vec![0.0, 1.0, 1.0];
        let into_account = vec![1.0, 0.0, 0.0];
        let account = vec![String::from("a"), String::from("a"), String::from("a")];

        let wb1 = workbook::WorkbookInfo::new(
            accounting_date,
            interest_date,
            archive_reference,
            counter_account,
            types,
            text,
            out_of_account,
            into_account,
            account,
        )
        .unwrap();
        let accounting_date = vec![
            NaiveDate::parse_from_str("01.01.2022", "%d.%m.%Y").unwrap(),
            NaiveDate::parse_from_str("01.02.2022", "%d.%m.%Y").unwrap(),
            NaiveDate::parse_from_str("01.02.2022", "%d.%m.%Y").unwrap(),
        ];
        let interest_date = vec![
            NaiveDate::parse_from_str("01.01.2022", "%d.%m.%Y").unwrap(),
            NaiveDate::parse_from_str("01.02.2022", "%d.%m.%Y").unwrap(),
            NaiveDate::parse_from_str("01.02.2022", "%d.%m.%Y").unwrap(),
        ];
        let archive_reference = vec![String::from("1"), String::from("1"), String::from("1")];
        let counter_account = vec![
            String::from("counter"),
            String::from("counter"),
            String::from("counter"),
        ];
        let types = vec![
            String::from("type1"),
            String::from("type2"),
            String::from("type3"),
        ];
        let text = vec![
            String::from("text1"),
            String::from("text2"),
            String::from("text3"),
        ];
        let out_of_account = vec![0.0, 0.0, 1.0];
        let into_account = vec![1.0, 1.0, 0.0];
        let account = vec![String::from("a"), String::from("a"), String::from("a")];

        let wb2 = workbook::WorkbookInfo::new(
            accounting_date,
            interest_date,
            archive_reference,
            counter_account,
            types,
            text,
            out_of_account,
            into_account,
            account,
        )
        .unwrap();

        let wb = remove_duplicates(wb1, wb2);
        let res = workbook::WorkbookInfo::new(
            vec![NaiveDate::parse_from_str("01.02.2022", "%d.%m.%Y").unwrap()],
            vec![NaiveDate::parse_from_str("01.02.2022", "%d.%m.%Y").unwrap()],
            vec![String::from("1")],
            vec![String::from("counter")],
            vec![String::from("type2")],
            vec![String::from("text2")],
            vec![1.0],
            vec![0.0],
            vec![String::from("a")],
        )
        .unwrap();
        assert_eq!(wb, res);
    }
}
