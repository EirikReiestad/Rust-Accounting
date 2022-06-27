use std::io::{Error, ErrorKind};
use std::path::Path;
use umya_spreadsheet::*;

pub fn open_file(path: &str) -> Result<umya_spreadsheet::Spreadsheet, Error> {
    match reader::xlsx::read(path) {
        Ok(book) => Ok(book),
        Err(_) => Err(Error::new(
            ErrorKind::ConnectionRefused,
            "could not open file",
        )),
    }
}

pub fn valid_file(path: &str) -> Result<String, Error> {
    // Makes the slashes the right way... windows is wierd
    let s = path.replace("\\", "/");
    if !Path::new(&s).is_file() {
        return Err(Error::new(ErrorKind::NotFound, "not a file"));
    };
    Ok(s)
}

// pub fn valid_filename(dir: &str, filename: &str) -> Result<String, Error> {
//     let s;
//     let chars: char;
//     match &dir.chars().last() {
//         Some(c) => chars = *c,
//         None => return Err(Error::new(ErrorKind::NotFound, "not a file")),
//     }
//     if !['/', '\\'].contains(&chars) {
//         s = String::from(dir) + "/" + filename;
//     } else {
//         s = String::from(dir) + filename;
//     }
//     if !Path::new(&s).is_file() {
//         return Err(Error::new(ErrorKind::NotFound, "not a file"));
//     };
//     Ok(s)
// }

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
