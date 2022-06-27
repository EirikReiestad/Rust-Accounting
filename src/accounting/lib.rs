#[derive(PartialEq, Default, Debug)]
pub struct Account {
    pub name: String,
    pub number: u64,
}

impl Account {
    pub fn clone(account: &Account) -> Account {
        Account {
            name: account.name.clone(),
            number: account.number.clone(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}: {}", self.name.clone(), self.number.clone())
    }

    pub fn is_valid(&self) -> bool {
        // checks if name and number is not empty
        if self.name == String::from("") && self.number == 0 {
            return false;
        };
        true
    }
}

#[cfg(test)]
mod tests_accounting_lib {
    use super::*;

    #[test]
    fn test_account_to_string() {
        let a = Account {
            name: String::from("a"),
            number: 0,
        };
        assert_eq!(a.to_string(), String::from("a: 0"));
    }
}
