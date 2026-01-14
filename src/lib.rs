
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimeter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimeter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimeter: delimeter ,
        }
    }
}

trait Delimeter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit <'haystack, D>
where 
D: Delimeter {
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((delim_start , delim_end)) = self.delimeter.find_next(remainder) {
            let until_delimeter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimeter)
        } else {
            self.remainder.take()
        }
    }
}

impl Delimeter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start , start + self.len()))
    }
}

impl Delimeter for char {
    
}