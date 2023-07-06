pub trait MyStringExtensions {
    fn remove_newline(&self) -> String;
    fn check_utf16(&self) -> String;
    fn create_string_from_str(s: &str) -> String;
}

/// Check if the string is UTF-16 and convert it to UTF-8 if it is
impl MyStringExtensions for String {
    fn remove_newline(&self) -> String {
        let mut ret = String::new();
        for c in self.chars() {
            if c != '\n' && c != '\r' {
                ret.push(c);
            }
        }
        ret
    }

    fn check_utf16(&self) -> String {
        let mut ret = String::new();
        let mut chars = self.chars();
        let mut c = chars.next();
        while c.is_some() {
            if c.unwrap() == '\u{feff}' {
                c = chars.next();
                continue;
            }
            ret.push(c.unwrap());
            c = chars.next();
        }
        ret
    }

    fn create_string_from_str(s: &str) -> String {
        let mut ret = String::new();
        for c in s.chars() {
            if c == '\0' {
                break;
            }
            ret.push(c);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_newline() {
        let s = String::from("test\n");
        assert_eq!(s.remove_newline(), "test");
    }

    #[test]
    fn test_check_utf16() {
        let s = String::from("\u{feff}test");
        assert_eq!(s.check_utf16(), "test");
    }

    #[test]
    fn test_create_string_from_str() {
        let s = String::create_string_from_str("test");
        assert_eq!(s, "test");
    }

    #[test]
    fn test_create_string_from_str_with_null() {
        let s = String::create_string_from_str("test\0");
        assert_eq!(s, "test");
    }
}
