// use tokeniser::Token;

use clap::Parser;

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

#[derive(Parser)]
struct Cli {
    /// The expression to evaluate
    expression: String,
}

fn main() {
    let cli = Cli::parse();

    let tokens = tokenise(cli.expression);
    let tokens = tokens.expect("Failed to tokenise expression");

    let tree = parse(&tokens);
    let tree = tree.expect("Failed to parse expression");

    let result = tree.eval();
    let result = result.expect("Failed to evaluate expression");

    println!("{}", result);
}
