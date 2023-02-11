use std::ops::{Add, Div, Mul, Sub};

use crate::{math::{factorial, nCr, nPr}, parser::TreeNode};

#[derive(Debug, Clone)]
pub enum Token {
    LParen,
    RParen,
    Operator {
        fun: fn(f64, f64) -> f64,
        priority: i32,
        name: String,
    }, // 5 + 4   or   6 nCr 2   etc.
    Function {
        fun: fn(f64) -> f64,
        priority: i32,
        after: bool,
        name: String,
    }, // sqrt 9 (before)    or    5! (after)
    Constant(f64),
    ParsedTree(Box<TreeNode>),
}

#[derive(Debug)]
pub enum TokenError {
    InvalidToken(char),
    InvalidName(String),
    InvalidNumber(String),
}

const LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWCYZ";
const NUMS: &'static str = "1234567890.";
const OPS: &'static str = "+-*/%^!";

#[derive(PartialEq)]
enum TokenType {
    Letter,
    Num,
    LParen,
    RParen,
    Op,
    None,
}

fn get_token_type(c: char) -> Result<TokenType, TokenError> {
    if LETTERS.contains(c) {
        return Ok(TokenType::Letter);
    }
    if NUMS.contains(c) {
        return Ok(TokenType::Num);
    }
    if OPS.contains(c) {
        return Ok(TokenType::Op);
    }
    return match c {
        '(' => Ok(TokenType::LParen),
        ')' => Ok(TokenType::RParen),
        ' ' | '\n' => Ok(TokenType::None),
        _ => Err(TokenError::InvalidToken(c)),
    };
}

fn get_thing_str(s: &str) -> Result<Token, TokenError> {
    match s {
        // Functions
        "sin" => Ok(Token::Function {
            fun: f64::sin,
            priority: 3,
            after: false,
            name: "sin".to_string(),
        }),
        "cos" => Ok(Token::Function {
            fun: f64::cos,
            priority: 3,
            after: false,
            name: "cos".to_string(),
        }),
        "tan" => Ok(Token::Function {
            fun: f64::tan,
            priority: 3,
            after: false,
            name: "tan".to_string(),
        }),
        "asin" => Ok(Token::Function {
            fun: f64::asin,
            priority: 3,
            after: false,
            name: "asin".to_string(),
        }),
        "acos" => Ok(Token::Function {
            fun: f64::acos,
            priority: 3,
            after: false,
            name: "acos".to_string(),
        }),
        "atan" => Ok(Token::Function {
            fun: f64::atan,
            priority: 3,
            after: false,
            name: "atan".to_string(),
        }),
        "sinh" => Ok(Token::Function {
            fun: f64::sinh,
            priority: 3,
            after: false,
            name: "sinh".to_string(),
        }),
        "cosh" => Ok(Token::Function {
            fun: f64::cosh,
            priority: 3,
            after: false,
            name: "cosh".to_string(),
        }),
        "tanh" => Ok(Token::Function {
            fun: f64::tanh,
            priority: 3,
            after: false,
            name: "tanh".to_string(),
        }),
        "asinh" => Ok(Token::Function {
            fun: f64::asinh,
            priority: 3,
            after: false,
            name: "asinh".to_string(),
        }),
        "acosh" => Ok(Token::Function {
            fun: f64::acosh,
            priority: 3,
            after: false,
            name: "acosh".to_string(),
        }),
        "atanh" => Ok(Token::Function {
            fun: f64::atanh,
            priority: 3,
            after: false,
            name: "atanh".to_string(),
        }),
        "sqrt" => Ok(Token::Function {
            fun: f64::sqrt,
            priority: 3,
            after: false,
            name: "sqrt".to_string(),
        }),
        "cbrt" => Ok(Token::Function {
            fun: f64::cbrt,
            priority: 3,
            after: false,
            name: "cbrt".to_string(),
        }),
        "exp" => Ok(Token::Function {
            fun: f64::exp,
            priority: 3,
            after: false,
            name: "exp".to_string(),
        }),
        "ln" => Ok(Token::Function {
            fun: f64::ln,
            priority: 3,
            after: false,
            name: "ln".to_string(),
        }),
        "log" => Ok(Token::Function {
            fun: f64::log10,
            priority: 3,
            after: false,
            name: "log".to_string(),
        }),
        "abs" => Ok(Token::Function {
            fun: f64::abs,
            priority: 3,
            after: false,
            name: "abs".to_string(),
        }),
        "floor" => Ok(Token::Function {
            fun: f64::floor,
            priority: 3,
            after: false,
            name: "floor".to_string(),
        }),
        "ceil" => Ok(Token::Function {
            fun: f64::ceil,
            priority: 3,
            after: false,
            name: "ceil".to_string(),
        }),
        "round" => Ok(Token::Function {
            fun: f64::round,
            priority: 3,
            after: false,
            name: "round".to_string(),
        }),
        // Operators
        "nCr" => Ok(Token::Operator {
            fun: nCr,
            priority: 5, // on my calculator, nCr and nPr are higher than multiplication and division
            name: "nCr".to_string(),
        }),
        "nPr" => Ok(Token::Operator {
            fun: nPr,
            priority: 5,
            name: "nPr".to_string(),
        }),
        // Constants
        "pi" => Ok(Token::Constant(std::f64::consts::PI)),
        "e" => Ok(Token::Constant(std::f64::consts::E)),
        _ => Err(TokenError::InvalidName(s.to_string())),
    }
}

fn get_thing_char(c: char) -> Result<Token, TokenError> {
    match c {
        '+' => Ok(Token::Operator {
            fun: Add::add,
            priority: 1,
            name: "+".to_string(),
        }),
        '-' => Ok(Token::Operator {
            fun: Sub::sub,
            priority: 1,
            name: "-".to_string(),
        }),
        '*' => Ok(Token::Operator {
            fun: Mul::mul,
            priority: 2,
            name: "*".to_string(),
        }),
        '/' => Ok(Token::Operator {
            fun: Div::div,
            priority: 2,
            name: "/".to_string(),
        }),
        '^' => Ok(Token::Operator {
            fun: f64::powf,
            priority: 4,
            name: "^".to_string(),
        }),
        '%' => Ok(Token::Operator {
            fun: f64::rem_euclid,
            priority: 2,
            name: "%".to_string(),
        }),
        '!' => Ok(Token::Function {
            fun: factorial,
            priority: 3,
            name: "!".to_string(),
            after: true,
        }),
        _ => Err(TokenError::InvalidToken(c)),
    }
}

fn push(
    current_type: TokenType,
    tokens: &mut Vec<Token>,
    current_str: &String,
) -> Result<(), TokenError> {
    match current_type {
        TokenType::Num => {
            if let Ok(f) = current_str.parse() {
                tokens.push(Token::Constant(f));
            } else {
                return Err(TokenError::InvalidNumber(current_str.to_owned()));
            }
        }
        TokenType::Letter => {
            tokens.push(get_thing_str(current_str)?);
        }
        _ => {}
    }
    Ok(())
}

pub fn tokenise(input: String) -> Result<Vec<Token>, TokenError> {
    let mut tokens: Vec<Token> = vec![];

    let mut current_type = TokenType::None;
    let mut current_str: String = "".to_owned();

    for c in input.chars() {
        let t = get_token_type(c)?;
        match t {
            TokenType::Letter | TokenType::Num => {
                if current_type != TokenType::None {
                    if current_type == t {
                        current_str.push(c);
                    } else {
                        push(current_type, &mut tokens, &current_str)?;
                        current_type = t;
                        current_str = c.to_string();
                    }
                } else {
                    current_type = t;
                    current_str = c.to_string();
                }
            }
            TokenType::LParen => {
                push(current_type, &mut tokens, &current_str)?;
                current_type = TokenType::None;
                tokens.push(Token::LParen);
            }
            TokenType::RParen => {
                push(current_type, &mut tokens, &current_str)?;
                current_type = TokenType::None;
                tokens.push(Token::RParen);
            }
            TokenType::Op => {
                push(current_type, &mut tokens, &current_str)?;
                current_type = TokenType::None;
                tokens.push(get_thing_char(c)?);
            }
            TokenType::None => {
                match current_type {
                    TokenType::Num => {
                        // don't do anything as numbers can be represented as 6 312 503 (six million, three hundred and twelve thousand, five hundred and three)
                    }
                    TokenType::Letter => {
                    }
                    _ => {
                        current_type = TokenType::None;
                    }
                }
            }
        }
    }

    push(current_type, &mut tokens, &current_str)?;

    Ok(tokens)
}
