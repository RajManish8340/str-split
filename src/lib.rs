
pub struct StrSplit<'haystack, 'remainder> {
    remainder: Option<&'haystack str>,
    delimeter: &'remainder str,
}

impl<'haystack, 'remainder> StrSplit<'haystack, 'remainder> {
    pub fn new(haystack: &'haystack str, delimeter: &'remainder str) -> Self {
        Self {
            remainder: Some(haystack),
            delimeter: delimeter ,
        }
    }
}

// trait Delimeter {
//     fn find_next(&self, s: &str) -> Option<(usize, usize)>;
// }

impl<'haystack, 'remainder> Iterator for StrSplit <'haystack, 'remainder> {
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delim) = remainder.find(self.delimeter) {
            let until_delimeter = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimeter.len())..];
            Some(until_delimeter)
        } else {
            self.remainder.take()
        }
    }
}