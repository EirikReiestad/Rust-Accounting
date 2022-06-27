use std::error;
use std::path::Path;
use umya_spreadsheet::*;

pub fn open_file(path: &str) -> Result<umya_spreadsheet::Spreadsheet, Box<dyn error::Error>> {
    Ok(reader::xlsx::read(path).map_err(|e| format!("could not open file: {:?}", e))?)
}

pub fn valid_file(path: &str) -> Result<String, Box<dyn error::Error>> {
    // Makes the slashes the right way... windows is wierd
    let s = path.replace("\\", "/");
    if !Path::new(&s).is_file() {
        return Err(format!("{} is not a file", &s).into());
    };
    Ok(s)
}

#[cfg(test)]
mod test_file {
    use super::*;
    #[test]
    fn test_valid_file() {
        assert!(valid_file("test").is_err());
        assert!(valid_file("C:/").is_err());
        assert!(valid_file("C:\\").is_err());
    }

    #[test]
    fn test_open_file() {
        assert!(open_file("test").is_err());
    }
}
