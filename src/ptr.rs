use std::str::Chars;

pub struct Ptr<'a> {
    text: &'a str,
    cursor: usize,
}

impl<'a> Ptr<'a> {
    pub fn new(text: &'a str) -> Ptr {
        Ptr {
            text,
            cursor: 0,
        }
    }

    pub fn chars(&self) -> Chars {
        self.text[self.cursor..].chars()
    }

    pub fn next(&mut self) -> Option<char> {
        let chr = self.chars().next();
        self.cursor += 1;
        chr
    }

    pub fn is_number(&self) -> bool {
        let chr = self.text[self.cursor..].chars().next();
        if let Some(chr) = chr {
            chr.is_numeric()
        } else {
            false
        }
    }

    pub fn parse_number(&mut self) -> String {
        let start_cursor = self.cursor;
        while  self.is_number() {
            self.next();
        }
        self.text[start_cursor..self.cursor].chars().collect()
    }

}

#[test]
fn test_ptr() {
    let mut ptr = Ptr::new("a12bc");
    assert_eq!("a".chars().next().unwrap(), ptr.next().unwrap());
    assert_eq!("12", ptr.parse_number());
    assert_eq!("b".chars().next().unwrap(), ptr.next().unwrap());
    assert_eq!("c".chars().next().unwrap(), ptr.next().unwrap());
}
