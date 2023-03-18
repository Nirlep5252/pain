use crate::utils::error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const KEYWORDS: [&str; 1] = ["print"];
const OPENING: [char; 4] = ['[', '(', '<', '{'];
const CLOSING: [char; 4] = [']', ')', '>', '}'];

#[derive(Debug)]
enum TokenType {
    Keyword,
    String,
    Opening,
    Closing,
}

#[derive(Debug)]
#[allow(unused)]
struct Token {
    typ: TokenType,
    val: String,
}

fn is_string(token: &str) -> bool {
    (token.starts_with("\"") && token.ends_with("'") && !token.ends_with("\\'"))
        || (token.starts_with("'") && token.ends_with("\"") && !token.ends_with("\\\""))
}

fn tokenize(code: String) -> Vec<Token> {
    let mut tokens = vec![];
    let mut temp = String::new();

    fn token_type(tk: &str) -> Result<TokenType, ()> {
        if KEYWORDS.contains(&tk) {
            Ok(TokenType::Keyword)
        } else if is_string(tk) {
            Ok(TokenType::String)
        } else {
            Err(())
        }
    }

    // TODO: reverse tokenization
    for line in code.split("\n") {
        for ch in line.chars() {
            // dbg!(&tokens, &ch, &temp);
            if ch == ' ' {
                if temp.starts_with("\"") || temp.starts_with("'") {
                    temp.push(ch);
                    continue;
                }
                if temp == "" {
                    continue;
                }
                match token_type(temp.as_str()) {
                    Ok(t) => tokens.push(Token {
                        typ: t,
                        val: temp.clone(),
                    }),
                    Err(_) => error(format!("`{temp}` is not identified.")),
                };
                temp = "".to_string();
            } else if OPENING.contains(&ch) || CLOSING.contains(&ch) {
                if temp != "" {
                    match token_type(temp.as_str()) {
                        Ok(t) => tokens.push(Token {
                            typ: t,
                            val: temp.clone(),
                        }),
                        Err(_) => error(format!("`{temp}` is not identified.")),
                    };
                    temp = "".to_string();
                }
                tokens.push(Token {
                    typ: if OPENING.contains(&ch) {
                        TokenType::Opening
                    } else {
                        TokenType::Closing
                    },
                    val: String::from(format!("{ch}")),
                });
            } else {
                // letters
                temp.push(ch);
            }
        }
    }
    tokens
}

pub fn interpret(filename: &str) {
    let path = Path::new(filename);
    let mut file = match File::open(&path) {
        Err(e) => {
            error(format!("ERROR: Unable to open: {filename}\n{e}"));
            panic!();
        }
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(e) => error(format!("ERROR: Unable to read: {filename}\n{e}")),
        Ok(_) => {
            let tokens = tokenize(s);
            dbg!(&tokens);
            // TODO: actually interpret the fucking file lmfao
        }
    }
}
