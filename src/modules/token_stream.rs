//use crate::error_handling::Result;
use super::tokens::AnalyzedToken;

pub struct TokenStream {
    token_list: Vec<AnalyzedToken>,
}

impl TokenStream {
    pub fn new(list: Vec<AnalyzedToken>) -> TokenStream {
        TokenStream { token_list: list }
    }
    pub fn iter(&self) -> TokenStreamIter<'_> {
        TokenStreamIter {
            token_list: &self.token_list,
            index: 0,
        }
    }
}

pub struct TokenStreamIter<'a> {
    token_list: &'a [AnalyzedToken],
    index: usize,
}

impl<'a> Iterator for TokenStreamIter<'a> {
    type Item = &'a AnalyzedToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.token_list.len() {
            None
        } else {
            let item = &self.token_list[self.index];
            self.index += 1;
            Some(item)
        }
    }
}
