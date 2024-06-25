use super::token::{Token, TokenType};

pub struct Scanner {
    tokens: Vec<Token>,
    pos: usize,
}

impl Scanner {
    pub fn new(text: String) -> Self {
        let tokens = Self::split(text);
        Scanner { tokens, pos: 0 }
    }

    fn split(text: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut idx = 0;
        let chars: Vec<char> = text.chars().collect();

        while idx < chars.len() {
            let x = chars[idx];
            if x.is_whitespace() {
                idx += 1;
            } else if "+-".contains(x) {
                tokens.push(Token::new(TokenType::AddOp, Some(&x.to_string())));
                idx += 1;
            } else if "*/%".contains(x) {
                tokens.push(Token::new(TokenType::MulOp, Some(&x.to_string())));
                idx += 1;
            } else if x == '^' {
                tokens.push(Token::new(TokenType::PowOp, Some(&x.to_string())));
                idx += 1;
            } else if "()".contains(x) {
                tokens.push(Token::new(TokenType::Parenthes, Some(&x.to_string())));
                idx += 1;
            } else if x.is_digit(10) {
                let mut num = String::new();
                while idx < chars.len() && chars[idx].is_digit(10) {
                    num.push(chars[idx]);
                    idx += 1;
                }
                tokens.push(Token::new(TokenType::Int, Some(&num)));
            } else {
                panic!("Unexpected character: {}", x);
            }
        }
        tokens
    }

    pub fn is_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    pub fn next(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        self.pos += 1;
        token
    }

    pub fn peek(&self) -> Token {
        self.tokens[self.pos].clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_basic() {
        let input = "3 + 5 * (10 - 4)";
        let scanner = Scanner::new(input.to_string());
        
        let tokens = scanner.tokens;
        let expected_tokens = vec![
            Token::new(TokenType::Int, Some("3")),
            Token::new(TokenType::AddOp, Some("+")),
            Token::new(TokenType::Int, Some("5")),
            Token::new(TokenType::MulOp, Some("*")),
            Token::new(TokenType::Parenthes, Some("(")),
            Token::new(TokenType::Int, Some("10")),
            Token::new(TokenType::AddOp, Some("-")),
            Token::new(TokenType::Int, Some("4")),
            Token::new(TokenType::Parenthes, Some(")")),
        ];
    
        assert_eq!(tokens, expected_tokens, "expected: {expected_tokens:?}, actual: {tokens:?}");
    }
    
    #[test]
    fn test_scanner_whitespace() {
        let input = " 3+4^                                   5     ";
        let scanner = Scanner::new(input.to_string());
        
        let tokens = scanner.tokens;
        let expected_tokens = vec![
            Token::new(TokenType::Int, Some("3")),
            Token::new(TokenType::AddOp, Some("+")),
            Token::new(TokenType::Int, Some("4")),
            Token::new(TokenType::PowOp, Some("^")),
            Token::new(TokenType::Int, Some("5")),
        ];
    
        assert_eq!(tokens, expected_tokens, "expected: {expected_tokens:?}, actual: {tokens:?}");
    }
    
    #[test]
    fn test_scanner_big_integer() {
        let input = "1000000007 % 998244353";
        let scanner = Scanner::new(input.to_string());

        let tokens = scanner.tokens;
        let expected_tokens = vec![
            Token::new(TokenType::Int, Some("1000000007")),
            Token::new(TokenType::MulOp, Some("%")),
            Token::new(TokenType::Int, Some("998244353")),
        ];

        assert_eq!(tokens, expected_tokens, "expected: {expected_tokens:?}, actual: {tokens:?}");
    }
}

