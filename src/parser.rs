use crate::tokeniser::Token;

#[derive(Debug)]
pub enum ParseError {
    // UnknownOperator(char),
    // UnknownFunctionOrVariable(String),
    InvalidNumber(String),
    UnexpectedOperator(String),
    UnexpectedFunction(String),
    // UnexpectedNumber,
    // UnexpectedLParen,
    UnexpectedRParen,
} // TODO: figure out which of these are actually needed

#[derive(Debug, Clone)]
pub struct TreeNode {
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
    value: Option<Token>,
}

impl TreeNode {
    fn get_value(self) -> Result<f64, ParseError> {
        if let Some(val) = self.value {
            match val {
                Token::Operator {
                    fun,
                    priority: _,
                    name: _,
                } => {
                    if self.left.is_none() || self.right.is_none() {
                        return Err(ParseError::UnexpectedOperator(
                            "No left or right".to_string(),
                        ));
                    }
                    let left = self.left.unwrap().get_value()?;
                    let right = self.right.unwrap().get_value()?;
                    return Ok(fun(left, right));
                }
                Token::Function {
                    fun,
                    priority: _,
                    after: _,
                    name: _,
                } => {
                    // if after { // ngl don't see a reason to have left or right depend on after
                    if self.left.is_none() {
                        return Err(ParseError::UnexpectedFunction("No left".to_string()));
                    }
                    let left = self.left.unwrap().get_value()?;
                    return Ok(fun(left));
                    // } else {
                    //     if self.right.is_none() {
                    //         return Err(ParseError::UnexpectedFunction("No left".to_string()));
                    //     }
                    //     let left = self.left.unwrap().get_value()?;
                    //     return Ok(fun(left));
                    // }
                }
                Token::Constant(value) => {
                    return Ok(value);
                }
                Token::LParen => todo!("LParen"),
                Token::RParen => todo!("RParen"),
                Token::ParsedTree(..) => panic!("ParsedTree in ParsedTree"),
            }
        } else {
            return Err(ParseError::InvalidNumber("No value".to_string()));
        }
    }

    pub fn eval(self) -> Result<f64, ParseError> {
        return self.get_value();
    }
}

pub fn parse(vec: &Vec<Token>) -> Result<TreeNode, ParseError> {
    let mut vec = vec.clone();

    while vec.len() > 1 || !matches!(vec[0], Token::ParsedTree(_)) {
        // TODO: Make this more efficient.
        find_and_replace(&mut vec, 0)?;
    }

    let first = vec.pop();

    if let Some(t) = first {
        match t {
            Token::ParsedTree(tree) => {
                return Ok(*tree);
            }
            _ => {
                return Err(ParseError::InvalidNumber("No value".to_string()));
            }
        }
    } else {
        return Err(ParseError::InvalidNumber("No value".to_string()));
    }
}

macro_rules! replace_stuff {
    ($left:expr, $right:expr, $val:expr, $vec:expr, $at:expr) => {
        let tree = TreeNode {
            left: Some(Box::new(if let Token::ParsedTree(t) = $left {
                *t.clone()
            } else {
                TreeNode {
                    left: None,
                    right: None,
                    value: Some($left.clone()),
                }
            })),
            right: Some(Box::new(if let Token::ParsedTree(t) = $right {
                *t.clone()
            } else {
                TreeNode {
                    left: None,
                    right: None,
                    value: Some($right.clone()),
                }
            })),
            value: Some($val.clone()),
        };
        $vec.drain($at..$at + 2);
        $vec[$at] = Token::ParsedTree(Box::new(tree));
    };
    ($left:expr, $val:expr, $vec:expr, $at:expr) => {
        let tree = TreeNode {
            left: Some(Box::new(if let Token::ParsedTree(t) = $left {
                *t.clone()
            } else {
                TreeNode {
                    left: None,
                    right: None,
                    value: Some($left.clone()),
                }
            })),
            right: None,
            value: Some($val.clone()),
        };
        $vec.drain($at..$at + 1);
        $vec[$at] = Token::ParsedTree(Box::new(tree));
    };
}

/** find_and_replace

Algorithm:
1. If num op num found or fun with after as false and num found
1.1. If next is an operator of strictly higher priority than op or fun or function with strictly higher priority and after is true
     call find_and_replace at second num's index
1.2. Else
     Replace the tokens with ParsedToken
2. If num and fun with after as true found
2.1 Replace the tokens with ParsedToken

num can be either a number, a lparen, or a ParsedToken. If it's an lparen, call replace_paren with position of lparen
*/
// TODO: DRY this up using macros probably. I have lots of repeated code
// TODO: CLEAN THIS TF UP
fn find_and_replace(vec: &mut Vec<Token>, at: usize) -> Result<(), ParseError> {
    let val1 = vec.get(at);
    let val2 = vec.get(at + 1);
    let val3 = vec.get(at + 2);
    let val4 = vec.get(at + 3);

    if let Some(t1) = val1 {
        match t1 {
            Token::LParen => {
                replace_paren(vec, at)?;
                return Ok(());
            }
            Token::RParen => {
                return Err(ParseError::UnexpectedRParen);
            }
            Token::Constant(_) | Token::ParsedTree(_) => {
                if let Some(t2) = val2 {
                    match t2 {
                        Token::LParen => todo!("Handle implicit multiplication"),
                        Token::RParen => {
                            return Err(ParseError::UnexpectedRParen);
                        }
                        Token::Operator {
                            fun: _,
                            priority: p1,
                            name: _,
                        } => {
                            if let Some(t3) = val3 {
                                match t3 {
                                    Token::LParen => {
                                        replace_paren(vec, at + 2)?;
                                        return Ok(()); // let the next iteration handle it
                                    }
                                    Token::RParen => {
                                        return Err(ParseError::UnexpectedRParen);
                                    }
                                    Token::Constant(_) | Token::ParsedTree(_) => {
                                        if let Some(t4) = val4 {
                                            match t4 {
                                                Token::LParen => {
                                                    todo!("Handle implicit multiplication")
                                                }
                                                Token::RParen => {
                                                    replace_stuff!(t1, t3, t2, vec, at);
                                                    return Ok(());
                                                }
                                                Token::Operator {
                                                    fun: _,
                                                    priority: p2,
                                                    name: _,
                                                } => {
                                                    if p2 > p1 {
                                                        find_and_replace(vec, at + 2)?;
                                                        return Ok(());
                                                    } else {
                                                        replace_stuff!(t1, t3, t2, vec, at);
                                                        return Ok(());
                                                    }
                                                }
                                                Token::Function {
                                                    fun: _,
                                                    priority: p2,
                                                    after,
                                                    name: _,
                                                } => {
                                                    if *after && p2 > p1 {
                                                        find_and_replace(vec, at + 2)?;
                                                        return Ok(());
                                                    } else {
                                                        replace_stuff!(t1, t3, t2, vec, at);
                                                        return Ok(());
                                                    }
                                                }
                                                Token::Constant(_) => {
                                                    todo!("Handle implicit multiplication")
                                                }
                                                Token::ParsedTree(_) => {
                                                    todo!("Handle implicit multiplication")
                                                }
                                            }
                                        } else {
                                            replace_stuff!(t1, t3, t2, vec, at);
                                            return Ok(());
                                        }
                                    }
                                    Token::Operator {
                                        fun: _,
                                        priority: _,
                                        name: _,
                                    } => todo!(),
                                    Token::Function {
                                        fun: _,
                                        priority: _,
                                        after: _,
                                        name: _,
                                    } => todo!(),
                                }
                            }
                        }
                        Token::Function {
                            fun: _,
                            priority: _,
                            after,
                            name: _,
                        } => {
                            if *after {
                                // for ex 5!. So no need to check for next token
                                let tree = TreeNode {
                                    left: Some(Box::new(if let Token::ParsedTree(t) = t1 {
                                        *t.clone()
                                    } else {
                                        TreeNode {
                                            left: None,
                                            right: None,
                                            value: Some(t1.clone()),
                                        }
                                    })),
                                    right: None,
                                    value: Some(t2.clone()),
                                };
                                vec.drain(at..at + 1);
                                vec[at] = Token::ParsedTree(Box::new(tree));
                                return Ok(());
                            } else {
                                todo!("Handle implicit multiplication")
                            }
                        }
                        Token::Constant(_) => todo!(),
                        Token::ParsedTree(_) => todo!(),
                    }
                } else {
                    let tree = TreeNode {
                        left: None,
                        right: None,
                        value: Some(t1.clone()),
                    };
                    vec[at] = Token::ParsedTree(Box::new(tree));
                    return Ok(());
                }
            }
            Token::Operator {
                fun: _,
                priority: p1,
                name,
            } => {
                if name == "-" {
                    if let Some(t2) = val2 {
                        match t2 {
                            Token::LParen => {
                                replace_paren(vec, at + 1)?;
                                return Ok(()); // let the next iteration handle it
                            }
                            Token::RParen => {
                                return Err(ParseError::UnexpectedRParen);
                            }
                            Token::Operator {
                                fun: _,
                                priority: _,
                                name,
                            } => {
                                return Err(ParseError::UnexpectedOperator(name.to_string()));
                            }
                            Token::Function {
                                fun: _,
                                priority: _,
                                after: _,
                                name,
                            } => {
                                return Err(ParseError::UnexpectedOperator(name.to_string()));
                            }
                            Token::Constant(_) | Token::ParsedTree(_) => {
                                if let Some(t3) = val3 {
                                    match t3 {
                                        Token::LParen => {
                                            replace_paren(vec, at + 2)?;
                                            return Ok(()); // let the next iteration handle it
                                        }
                                        Token::RParen => {
                                            return Err(ParseError::UnexpectedRParen);
                                        }
                                        Token::Operator {
                                            fun: _,
                                            priority: p2,
                                            name: _,
                                        } => {
                                            if p2 > p1 {
                                                find_and_replace(vec, at + 1)?;
                                                return Ok(());
                                            } else {
                                                let tree = TreeNode {
                                                    left: Some(Box::new(TreeNode {
                                                        left: None,
                                                        right: None,
                                                        value: Some(Token::Constant(0.0)),
                                                    })),
                                                    right: Some(Box::new(TreeNode {
                                                        left: None,
                                                        right: None,
                                                        value: Some(t2.clone()),
                                                    })),
                                                    value: Some(Token::Operator {
                                                        fun: |a, b| a - b,
                                                        priority: 2,
                                                        name: "-".to_string(),
                                                    }),
                                                };
                                                vec.drain(at..at + 1);
                                                vec[at] = Token::ParsedTree(Box::new(tree));
                                                return Ok(());
                                            }
                                        }
                                        Token::Function {
                                            fun: _,
                                            priority: p2,
                                            after,
                                            name: _,
                                        } => {
                                            if *after && p2 > p1 {
                                                find_and_replace(vec, at + 1)?;
                                                return Ok(());
                                            } else {
                                                let tree = TreeNode {
                                                    left: Some(Box::new(TreeNode {
                                                        left: None,
                                                        right: None,
                                                        value: Some(Token::Constant(0.0)),
                                                    })),
                                                    right: Some(Box::new(TreeNode {
                                                        left: None,
                                                        right: None,
                                                        value: Some(t2.clone()),
                                                    })),
                                                    value: Some(Token::Operator {
                                                        fun: |a, b| a - b,
                                                        priority: 2,
                                                        name: "-".to_string(),
                                                    }),
                                                };
                                                vec.drain(at..at + 1);
                                                vec[at] = Token::ParsedTree(Box::new(tree));
                                                return Ok(());
                                            }
                                        }
                                        _ => {
                                            let tree = TreeNode {
                                                left: Some(Box::new(TreeNode {
                                                    left: None,
                                                    right: None,
                                                    value: Some(Token::Constant(0.0)),
                                                })),
                                                right: Some(Box::new(TreeNode {
                                                    left: None,
                                                    right: None,
                                                    value: Some(t2.clone()),
                                                })),
                                                value: Some(Token::Operator {
                                                    fun: |a, b| a - b,
                                                    priority: 2,
                                                    name: "-".to_string(),
                                                }),
                                            };
                                            vec.drain(at..at + 1);
                                            vec[at] = Token::ParsedTree(Box::new(tree));
                                            return Ok(());
                                        }
                                    }
                                } else {
                                    let tree = TreeNode {
                                        left: Some(Box::new(TreeNode {
                                            left: None,
                                            right: None,
                                            value: Some(Token::Constant(0.0)),
                                        })),
                                        right: Some(Box::new(
                                            if let Token::ParsedTree(t) = t2 {
                                                *t.clone()
                                            } else {
                                                TreeNode {
                                                    left: None,
                                                    right: None,
                                                    value: Some(t2.clone()),
                                                }
                                            },
                                        )),
                                        value: Some(Token::Operator {
                                            fun: |a, b| a - b,
                                            priority: 2,
                                            name: "-".to_string(),
                                        }),
                                    };
                                    vec.drain(at..at + 1);
                                    vec[at] = Token::ParsedTree(Box::new(tree));
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
                return Err(ParseError::UnexpectedOperator(name.to_string())); // TODO: Handle negative numbers
            }
            Token::Function {
                fun: _,
                priority: p1,
                after,
                name,
            } => {
                if !after {
                    if let Some(t2) = val2 {
                        match t2 {
                            Token::LParen => {
                                replace_paren(vec, at + 1)?;
                                return Ok(()); // let the next iteration handle it
                            }
                            Token::RParen => {
                                return Err(ParseError::UnexpectedRParen);
                            }
                            Token::Operator {
                                fun: _,
                                priority: _,
                                name,
                            } => {
                                return Err(ParseError::UnexpectedOperator(name.to_string()));
                                // TODO: Handle negative numbers
                            }
                            Token::Function {
                                fun: _,
                                priority: _,
                                after: _,
                                name: _,
                            } => todo!(),
                            Token::Constant(_) | Token::ParsedTree(_) => {
                                if let Some(t3) = val3 {
                                    match t3 {
                                        Token::LParen => {
                                            replace_paren(vec, at + 2)?;
                                            return Ok(()); // let the next iteration handle it
                                        }
                                        Token::RParen => {
                                            return Err(ParseError::UnexpectedRParen);
                                        }
                                        Token::Operator {
                                            fun: _,
                                            priority: p2,
                                            name: _,
                                        } => {
                                            if p2 > p1 {
                                                find_and_replace(vec, at + 2)?;
                                                return Ok(());
                                            } else {
                                                replace_stuff!(t2, t1, vec, at);
                                                return Ok(());
                                            }
                                        }
                                        Token::Function {
                                            fun: _,
                                            priority: p2,
                                            after,
                                            name: _,
                                        } => {
                                            if *after && p2 > p1 {
                                                find_and_replace(vec, at + 2)?;
                                                return Ok(());
                                            } else {
                                                replace_stuff!(t2, t1, vec, at);
                                                return Ok(());
                                            }
                                        }
                                        Token::Constant(_) => {
                                            todo!("Handle implicit multiplication")
                                        }
                                        Token::ParsedTree(_) => {
                                            todo!("Handle implicit multiplication")
                                        }
                                    }
                                } else {
                                    replace_stuff!(t2, t1, vec, at);
                                    return Ok(());
                                }
                            }
                        }
                    } else {
                        return Err(ParseError::UnexpectedFunction(name.to_string()));
                    }
                } else {
                    return Err(ParseError::UnexpectedFunction(name.to_string()));
                }
            }
        }
    }
    Ok(())
}

/** replace_paren

Algorithm:
1. Make subvec from at+1 to the next rparen of same depth
2. Call find_and_replace on subvec until its length is 1
3. Replace everything from at to the next rparen with the ParsedToken which is the only element in the subvec
*/
fn replace_paren(vec: &mut Vec<Token>, at: usize) -> Result<(), ParseError> {
    let mut subvec = Vec::new();
    let mut i = at + 1;
    let mut depth = 1;

    loop {
        match vec[i] {
            Token::LParen => {
                depth += 1;
            }
            Token::RParen => {
                depth -= 1;
            }
            _ => {}
        }
        if depth == 0 {
            break;
        }
        subvec.push(vec[i].clone());
        i += 1;
    }

    while subvec.len() > 1 || !matches!(subvec[0], Token::ParsedTree(_)) {
        find_and_replace(&mut subvec, 0)?;
    }

    let first = subvec.pop();

    if let Some(t) = first {
        match t {
            Token::ParsedTree(tree) => {
                vec[at] = Token::ParsedTree(tree);
                vec.drain(at + 1..i + 1);
                return Ok(());
            }
            _ => {
                return Err(ParseError::InvalidNumber("No value".to_string()));
            }
        }
    } else {
        return Err(ParseError::InvalidNumber("No value".to_string()));
    }
}
