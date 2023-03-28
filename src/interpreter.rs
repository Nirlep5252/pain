use crate::utils::error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const KEYWORDS: [&str; 2] = ["print", "ignorethis"];
const OPENING: [char; 3] = ['[', '(', '{'];
const CLOSING: [char; 3] = [']', ')', '}'];
const OPERATORS: [char; 10] = ['+', '-', '*', '/', '^', '&', '|', '!', '>', '<'];
// TODO: return statement's name => fuckoff

#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    Keyword,
    String,
    Opening,
    Closing,
    Identifier,
    Operator,
    Seperator,
    Assignment,
}

#[derive(Debug, PartialEq, Clone)]
struct Token {
    typ: TokenType,
    val: String,
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    val: Option<Token>,
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
            let lines = tokenize(s);
            for line in lines {
                let parse_tree = build_parse_tree(&line);
            }
        }
    }
}

fn build_parse_tree(tokens: &Vec<Token>) -> Node {
    let mut x = Node {
        children: vec![],
        val: None,
    };
    let assignment = Token {
        typ: TokenType::Assignment,
        val: "=".to_string(),
    };

    let mut has_operator = false;
    let mut operator: Option<Token> = None;

    for token in tokens {
        if token.typ == TokenType::Operator {
            has_operator = true;
            operator = Some(token.clone());
            break;
        }
    }

    if tokens.len() == 1 {
        x.val = Some(tokens[0].clone());
    } else if tokens.contains(&assignment) {
        // IGNORE PREVIOUS CODE
        // TAKE 2 tokens from left and right of the operator and then uwu it 

        // let ass_index = tokens.iter().position(|e| *e == assignment).unwrap();
        // let left_tokens = &tokens[..ass_index].to_vec();
        // let right_tokens = &tokens[ass_index + 1..].to_vec();
        // x.children = vec![build_parse_tree(left_tokens), build_parse_tree(right_tokens)];
        // x.val = Some(assignment);
    } else if has_operator {
        // let operator = operator.unwrap();
        // let op_index = tokens.iter().position(|e| *e == operator).unwrap();
        // let left_tokens = &tokens[..op_index].to_vec();
        // let right_tokens = &tokens[op_index + 1..].to_vec();
        // x.children = vec![build_parse_tree(left_tokens), build_parse_tree(right_tokens)];
        // x.val = Some(operator);
    } else {
    }

    x
}

fn tokenize(code: String) -> Vec<Vec<Token>> {
    let mut tokens: Vec<Vec<Token>> = vec![];
    let mut temp = String::new();

    fn token_type(tk: &str) -> TokenType {
        if KEYWORDS.contains(&tk) {
            TokenType::Keyword
        } else if is_string(tk) {
            TokenType::String
        } else {
            TokenType::Identifier
        }
    }

    fn is_comment(tkns: &Vec<Token>) -> bool {
        tkns.last()
            .unwrap_or(
                &(Token {
                    typ: TokenType::Keyword,
                    val: "".to_string(),
                }),
            )
            .val
            == "ignorethis".to_string()
    }

    for line in code.split('\n').rev() {
        let mut tokens_tmp: Vec<Token> = vec![];

        fn push_curr_token(tokens_tmp: &mut Vec<Token>, temp: &mut String) {
            tokens_tmp.push(Token {
                typ: token_type(temp.as_str()),
                val: temp.clone(),
            });
            *temp = "".to_string();
        }

        for ch in line.chars() {
            if is_comment(&tokens_tmp) {
                tokens_tmp.pop();
                break;
            }
            if ch == ' ' {
                if temp.starts_with("\"") || temp.starts_with("'") {
                    temp.push(ch);
                    continue;
                }
                if temp == "" {
                    continue;
                }
                push_curr_token(&mut tokens_tmp, &mut temp);
            } else if OPENING.contains(&ch) || CLOSING.contains(&ch) {
                if temp != "" {
                    push_curr_token(&mut tokens_tmp, &mut temp);
                }
                tokens_tmp.push(Token {
                    typ: if OPENING.contains(&ch) {
                        TokenType::Opening
                    } else {
                        TokenType::Closing
                    },
                    val: format!("{ch}"),
                });
            } else if OPERATORS.contains(&ch) {
                if temp != "" {
                    push_curr_token(&mut tokens_tmp, &mut temp);
                }
                tokens_tmp.push(Token {
                    typ: TokenType::Operator,
                    val: format!("{ch}"),
                });
            } else if ch == ',' {
                if temp != "" {
                    push_curr_token(&mut tokens_tmp, &mut temp);
                }
                tokens_tmp.push(Token {
                    typ: TokenType::Seperator,
                    val: format!("{ch}"),
                });
            } else if ch == '=' {
                if temp != "" {
                    push_curr_token(&mut tokens_tmp, &mut temp);
                }
                tokens_tmp.push(Token {
                    typ: TokenType::Assignment,
                    val: format!("{ch}"),
                });
            } else {
                // letters
                temp.push(ch);
            }
            // dbg!(&tokens, &temp, &ch);
        }
        if temp != "" {
            push_curr_token(&mut tokens_tmp, &mut temp);
        }
        tokens.push(tokens_tmp);
    }
    tokens
}

fn is_string(token: &str) -> bool {
    (token.starts_with("\"") && token.ends_with("'") && !token.ends_with("\\'"))
        || (token.starts_with("'") && token.ends_with("\"") && !token.ends_with("\\\""))
}
