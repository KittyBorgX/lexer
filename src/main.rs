use logos::Logos;
use std::fs::File;
use std::io::prelude::*;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("fast")]
    Fast,

    #[token(".")]
    Period,

    
    #[regex("[a-zA-Z]+")]
    Text,

    #[regex("[1-9]+")]
    Number,
    
    #[error]
   
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Spaces,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("hi.eng")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(for token in Token::lexer(&contents) {
        dbg!(token);
    })
}

