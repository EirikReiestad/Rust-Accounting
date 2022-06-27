use std::error;

use crate::file;
use file::lib;

// e.g. same_path(("C:/dir/", "test.xlsx"), ("C:/dir/", "empty.xlsx")) should return an Error
pub fn same_path(path1: &str, path2: &str) -> Result<bool, Box<dyn error::Error>> {
    let p1 = match lib::valid_file(path1) {
        Ok(p1) => (p1, true),
        Err(_) => (String::new(), false),
    };
    let p2 = match lib::valid_file(path2) {
        Ok(p2) => (p2, true),
        Err(_) => (String::new(), false),
    };

    if !p1.1 && !p2.1 {
        return Err("could not find path".into());
    };
    if p1.0 == p2.0 {
        return Ok(true);
    };
    Ok(false)
}

pub fn get_flash_error(error: Box<dyn error::Error>) -> String {
    error.to_string()
}

#[cfg(test)]
mod test_settings_lib {
    use super::*;

    #[test]
    fn test_same_path() {
        // cannot test if it is true, because it also checks if the file is valid
        assert!(same_path("dir/filename", "dir/filename").is_err());
    }

    #[test]
    fn test_get_flash_error() {
        let err: Box<dyn error::Error> = "test".into();
        assert_eq!(get_flash_error(err), "error: test");
    }
}
