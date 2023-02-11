use tokeniser::Token;

use crate::{parser::parse, tokeniser::tokenise};

mod math;
mod parser;
mod tokeniser;

// fn to_str(vec: Vec<Token>) -> String {
//     let mut s = String::default();

//     for t in vec {
//         let a: String;
//         s.push_str(match t {
//             Token::LParen => "(",
//             Token::RParen => ")",
//             Token::Operator { fun: _, priority: _, name } => &name,
//             Token::Function { fun: _, priority: _, after: _, name } => &name,
//             Token::Constant(i) => {a = i.to_string(); &a},
//             Token::ParsedTree(_) => todo!(),
//         });
//     }

//     s
// }

fn main() {
    let tokens = tokenise("((((pi))))".to_owned());
    let tokens = tokens.expect("Failed to tokenise");
    println!("Tokens: {:?}", tokens);
    let parsed = parse(&tokens);
    let parsed = parsed.expect("Failed to parse");
    println!("Parsed: {:?}", parsed);
    let value = parsed.get_value();
    let value = value.expect("Failed to get value");
    println!("Value: {}", value);
}
