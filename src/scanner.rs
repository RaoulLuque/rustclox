pub struct Scanner<'a> {
    source: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        todo!();
    }
}

#[derive(Debug)]
pub struct Token {}
