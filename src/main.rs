use std::{fs, iter::{self, from_fn}, fmt};

#[derive(Debug)]
struct SyntaxError {
    message : String,
}

impl SyntaxError {
    fn new(message : String) -> Self {
        SyntaxError {
            message
        }
    }
}

enum Token {
    Number(i64),
    Plus,
    Dash,
    Star,
    Slash,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    IntKeyword,
    Main,
    Return,
    Eof,
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
           match self {
               Token::Number(val) => write!(f, "NUMBER {}", val),
               Token::Dash => write!(f, "DASH"),
               Token::Plus => write!(f, "PLUS"),
               Token::Star => write!(f, "STAR"),
               Token::Slash => write!(f, "SLASH"),
               Token::LeftParen => write!(f, "L_PAREN"),
               Token::RightParen => write!(f, "R_PAREN"),
               Token::LeftBrace => write!(f, "L_BRACE"),
               Token::RightBrace => write!(f, "R_BRACE"),
               Token::Semicolon => write!(f, "SEMICOLON"),
               Token::IntKeyword => write!(f, "INT"),
               Token::Main => write!(f, "MAIN"),
               Token::Return => write!(f, "RETURN"),
               Token::Eof => write!(f, "EOF"),
           }
        }
}

fn main() {
    lex();
}

// Will exist as a minor Lexer for developing tokens for AST
fn lex() {
    
    // Currently hardcoded, will eventually intake argument of path
    let file_path = "/Users/braydbstro/Repositories/C-Compiler/tests/return_2.c";
    

    println!("In file {file_path}");
    
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let tokens : Result<Vec<Token>, SyntaxError> = tokenization(contents);
    if let Ok(val) = tokens {
        println!("{:?}", val);
    }
    else if let Err(val) = tokens {
        println!("{:?}", val.message);
    }
    
}

fn tokenization(contents : String) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens : Vec<Token>  = Vec::new();
    let mut iter = contents.chars().peekable();
    
    while let Some(ch) = iter.next() {
        match ch {
            ' ' | '\n' => continue,
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Dash),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            ';' => tokens.push(Token::Semicolon),
            '1'..='9' => {
                let n : i64 = iter::once(ch)
                    .chain(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit())))
                    .collect::<String>()
                    .parse()
                    .unwrap();
                
                tokens.push(Token::Number(n));
            }
            'a'..='z' | 'A'..='Z' => {
                let n : String = iter::once(ch)
                    .chain(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_alphabetic())))
                    .collect::<String>();
                match n.as_str() {
                    "int" => tokens.push(Token::IntKeyword),
                    "main" => tokens.push(Token::Main),
                    "return" => tokens.push(Token::Return),
                    _ => return Err(SyntaxError::new(format!("unrecognized string {}", n))),
                }
            },
            _ => return Err(SyntaxError::new(format!("unrecognized character {}", ch))),
        }
    }
    
    tokens.push(Token::Eof);
    Ok(tokens)
}
