use crate::{scanner::*, parser::*};

fn interpret_AST(node: &ASTnode) -> u32 {
    match node.op {
        ASTtype::Add => {
            interpret_AST(&node.left.as_ref().unwrap()) + interpret_AST(&node.right.as_ref().unwrap())
        }
        ASTtype::Subtract => {
            interpret_AST(&node.left.as_ref().unwrap()) - interpret_AST(&node.right.as_ref().unwrap())
        }
        ASTtype::Multiply => {
            interpret_AST(&node.left.as_ref().unwrap()) * interpret_AST(&node.right.as_ref().unwrap())
        }
        ASTtype::Divide => {
            interpret_AST(&node.left.as_ref().unwrap()) / interpret_AST(&node.right.as_ref().unwrap())
        }
        ASTtype::IntLiteral => match node.value.as_ref().unwrap() {
            ASTvalue::Int(x) => *x,
            _ => panic!("Deu merda!")
        },
        _ => panic!("Deu merda!")
    }
}

pub fn interpret(s: &str) -> u32 {
    let token_stream = scan(s);
    let (ast, _) = binexpr(&token_stream, 0);
    interpret_AST(&ast)
}
