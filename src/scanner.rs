#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    EOF, Plus, Minus, Star, Slash, IntLiteral
}

pub fn op_prec(token_type: &TokenType) -> u32 {
    match token_type {
        TokenType::Plus | TokenType::Minus => 10,
        TokenType::Star | TokenType::Slash => 20,
        _ => 0
    }
}

#[derive(Debug,Clone,Copy)]
pub enum TokenValue {
    Int(u32),
}

#[derive(Debug,Clone,Copy)]
pub struct Token {
    pub token: TokenType,
    pub value: Option<TokenValue>,
}


pub fn scan(input: &str) -> Vec<Token> {
    let input: Vec<char> = input.chars().collect();
    let mut token_stream = Vec::new();

    let mut i = 0;
    while i < input.len() {
        match input[i] {
            '+' => {
                    token_stream.push(Token {
                        token: TokenType::Plus,
                        value: None,
                    });
                }
            '-' => {
                    token_stream.push(Token {
                        token: TokenType::Minus,
                        value: None,
                    });
                }
            '*' => {
                    token_stream.push(Token {
                        token: TokenType::Star,
                        value: None,
                    });
                }
            '/' => {
                    token_stream.push(Token {
                        token: TokenType::Slash,
                        value: None,
                    });
                }
            ' ' | '\n' | '\t' | '\r' => {/* Do nothing */}
            c   => {
                if c.is_digit(10) {
                    let mut j = i;
                    let mut val = 0;
                    while input[j].is_digit(10) {
                        val *= 10;
                        val += input[j].to_digit(10).unwrap();
                        j += 1;
                        if j >= input.len() {
                            break;
                        }
                    } 
                    token_stream.push(Token {
                        token: TokenType::IntLiteral,
                        value: Some(TokenValue::Int(val)),
                    });
                    i = j-1;
                } else {
                    panic!("Not a number");
                }
            }
        }
        i += 1;
    }

    token_stream.push(Token {
        token: TokenType::EOF,
        value: None,
    });

    token_stream
}
