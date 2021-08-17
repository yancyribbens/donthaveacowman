// Code demonstrating using Cow to provide an API for both
// borrowed and owned types (&str and String for example).
//
// https://jwilm.io/blog/from-str-to-cow/
//
use std::borrow::Cow;

struct Token<'a> {
    raw: Cow<'a, str>,
}

impl<'a> Token<'a> {
    pub fn new<S>(raw: S) -> Token<'a> where S: Into<Cow<'a, str>>{
        Token { raw: raw.into() }
    }
}

fn main() {
    let _token = Token::new("abc");
    let a: String = "abc".to_string();
    let _token = Token::new(a);
    ()
}
