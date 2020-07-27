use crate::scanner::*;
use std::fmt;

#[derive(Debug)]
pub enum ASTtype {
    Add, Subtract, Multiply, Divide, IntLiteral
}

#[derive(Debug)]
pub enum ASTvalue {
    Int(u32)
}

pub struct ASTnode {
    op: ASTtype,
    left: Option<Box<ASTnode>>,
    right: Option<Box<ASTnode>>,
    value: Option<ASTvalue>,
}

impl fmt::Debug for ASTnode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(value) = &self.value {
            // If it has a value, print the value
            if let ASTvalue::Int(int_value) = value {
                f.write_str(&format!("{}", int_value))
            }else{
                panic!("Something is wrong with the value of the ASTnode")
            }
        }else{
            f.debug_struct(&format!("{:?}", self.op))
            .field("",
                    &format_args!("{}", match &self.left {
                        Some(x) => format!("{:#?}",x),
                        None => "None".to_string()
                    })
            ).field("",
                    &format_args!("{}", match &self.right {
                        Some(x) => format!("{:#?}",x),
                        None => "None".to_string()
                    })
            ).finish()
        }
    }     
}

impl ASTnode {
    fn new(op: ASTtype, left: Option<Box<ASTnode>>,
           right: Option<Box<ASTnode>>, value: Option<ASTvalue>)
        -> ASTnode {
        ASTnode {op, left, right, value}
    }

    fn new_leaf(op: ASTtype, value: ASTvalue) -> ASTnode {
        ASTnode::new(op, None, None, Some(value))
    }
    
    // Only one child
    fn new_unary(op: ASTtype, left: Option<Box<ASTnode>>,
                 value: Option<ASTvalue>) -> ASTnode {
        ASTnode::new(op, left, None, value)
    }
}

// Convert a arithmethic token type to an AST type
fn arith_op(tok: &TokenType) -> Result<ASTtype,String> {
    match tok {
        TokenType::Plus  => Ok(ASTtype::Add),
        TokenType::Minus => Ok(ASTtype::Subtract),
        TokenType::Star  => Ok(ASTtype::Multiply),
        TokenType::Slash => Ok(ASTtype::Divide),
        _                => Err(String::from("Unknown token"))
    }
}

// Parse primary factor
fn primary(tok: &Token) -> Result<ASTnode,String>{
    match tok.token {
        TokenType::IntLiteral => Ok(
            ASTnode::new_leaf(ASTtype::IntLiteral,
                              ASTvalue::Int(
                                if let TokenValue::Int(n) = tok.value.as_ref().unwrap() {
                                    *n
                                }else{
                                    panic!("Shit's broken")
                                }
                              ))
            ),
        _ => Err(String::from("Syntax error"))
    }
}

// Returns a tree whose root is a binary operator
pub fn binexpr(ts: &[Token], ptp: u32)
    -> (ASTnode, Vec<Token>) {
    let mut token_stream = ts.to_vec();
    let mut left = primary(&token_stream[0]).unwrap();
    token_stream = token_stream[1..].to_vec(); // Roll
    

    match token_stream[0].token {
        TokenType::EOF => (left, token_stream.to_vec()),
        _ => {
            let mut token_type = token_stream[0].token;
            while op_prec(&token_type) > ptp {
                let (right, new_ts) = binexpr(&token_stream[1..], op_prec(&token_type));
                token_stream = new_ts;
                left = ASTnode::new(
                    arith_op(&token_type).unwrap(),
                    Some(Box::new(left)),
                    Some(Box::new(right)),
                    None
                );
                token_type = token_stream[0].token;
                if let TokenType::EOF = token_type{
                    break;
                }
            }
            (left, token_stream.to_vec())
        }
    }
}
