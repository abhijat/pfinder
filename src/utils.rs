use std::io::{Error, ErrorKind, Result};

pub fn is_numeric(path_name: &str) -> bool {
    path_name.chars().all(|c| {
        c.is_ascii_digit()
    })
}

pub fn make_error(message: &str) -> Result<String> {
    Err(Error::new(ErrorKind::Other, message))
}

pub fn truncate(s: &mut String) {
    if !s.is_empty() {
        let new_len = s.len() - 1;
        s.truncate(new_len);
    }
}

pub fn remove_nulls(data: &str) -> String {
    let mut s = data.replace('\0', " ");
    truncate(&mut s);
    s
}
