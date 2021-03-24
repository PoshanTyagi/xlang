pub trait StringUtils {
    fn substring(&self, start: i64, end: i64) -> Self;
    fn char_at(&self, index: i64) -> char;
}

impl StringUtils for String {
    fn substring(&self, start: i64, end: i64) -> Self {
        self.chars()
            .skip(start as usize)
            .take((end - start) as usize)
            .collect()
    }

    fn char_at(&self, index: i64) -> char {
        return match self.chars().nth(index as usize) {
            Some(c) => c,
            None => '\0',
        };
    }
}
