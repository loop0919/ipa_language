use super::token::{Token, TokenType};
use regex::Regex;

pub struct Scanner {
    tokens: Vec<Token>,
    pos: usize,
}

impl Scanner {
    pub fn new(text: &str) -> Self {
        let tokens = Self::split(text);
        Scanner { tokens, pos: 0 }
    }

    fn tokenize(word: &str) -> Token {
        match word {
            "fn" => Token::new(TokenType::FuncDef, Some("fn")),
            "fnend" => Token::new(TokenType::EndFuncDef, Some("fnend")),
            "(" => Token::new(TokenType::OpenParenthes, Some("(")),
            ")" => Token::new(TokenType::CloseParenthes, Some(")")),
            "if" => Token::new(TokenType::If, Some("if")),
            "else" => Token::new(TokenType::Else, Some("else")),
            "endif" => Token::new(TokenType::EndIf, Some("endif")),
            "while" => Token::new(TokenType::While, Some("while")),
            "break" => Token::new(TokenType::Break, Some("break")),
            "continue" => Token::new(TokenType::Continue, Some("continue")),
            "endwhile" => Token::new(TokenType::EndWhile, Some("endwhile")),
            ":" => Token::new(TokenType::Colon, Some(":")),
            ";" => Token::new(TokenType::Semicolon, Some(";")),
            "," => Token::new(TokenType::Comma, Some(",")),
            "<-" => Token::new(TokenType::Assign, Some("<-")),
            "=" | "<" | ">" | "<=" | ">=" | "!=" => Token::new(TokenType::RelOp, Some(word)),
            "+" | "-" => Token::new(TokenType::AddOp, Some(word)),
            "*" | "/" | "%" => Token::new(TokenType::MulOp, Some(word)),
            "^" => Token::new(TokenType::PowOp, Some(word)),
            "Bool" => Token::new(TokenType::BoolType, Some("Bool")),
            "Int" => Token::new(TokenType::IntType, Some("Int")),
            "String" => Token::new(TokenType::StringType, Some("String")),
            "true" | "false" => Token::new(TokenType::Bool, Some(word)),
            x => {
                let token: Token;
                if Regex::new("^\"(.*)\"$").unwrap().is_match(x) {
                    token = Token::new(TokenType::String, Some(&x[1..word.len() - 1]));
                } else if Regex::new("\\d+").unwrap().is_match(x) {
                    token = Token::new(TokenType::Int, Some(x));
                } else {
                    token = Token::new(TokenType::Ident, Some(x));
                }
                token
            }
        }
    }

    fn split(text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut idx = 0;
        let chars: Vec<char> = text.chars().collect();

        while idx < chars.len() {
            let x = chars[idx];
            if x == '\"' {
                let mut string_literal = x.to_string();
                idx += 1;
                while chars[idx] != '\"' {
                    if chars[idx] == '\\' {
                        idx += 1;
                        match chars[idx] {
                            '\"' => { string_literal += "\""; },
                            'n' => { string_literal += "\n"; },
                            't' => { string_literal += "\t"; },
                            '\\' => { string_literal += "\\"; },
                            c => { string_literal += &c.to_string(); },
                        }
                    }
                    string_literal += &chars[idx].to_string();
                    idx += 1;
                }
                string_literal += "\"";
                idx += 1;
                tokens.push(Self::tokenize(&string_literal));
            } else if x == '<' {
                let mut op = chars[idx].to_string();
                idx += 1;
                match chars[idx] {
                    '-' => { 
                        op += "-";
                        idx += 1;
                    },
                    '=' => {
                        op += "=";
                        idx += 1;
                    },
                    _ => {},
                }
                tokens.push(Self::tokenize(&op));
            } else if ">!".contains(x) {
                let mut op = chars[idx].to_string();
                idx += 1;
                if chars[idx] == '=' {
                    op += "=";
                    idx += 1;
                }
                tokens.push(Self::tokenize(&op));
            } else if x == '/' {
                idx += 1;
                match chars[idx] {
                    '/' => {
                        while idx < chars.len() && chars[idx] != '\n' {
                            idx += 1;
                        }
                        idx += 1;
                    },
                    '*' => {
                        idx += 1;
                        while idx < chars.len() && !(chars[idx] == '*' && chars[idx + 1] == '/') { 
                            idx += 1;
                        }
                        idx += 2;
                    }
                    _ => { tokens.push(Self::tokenize("/")); },
                }
            } else if x.is_ascii_alphabetic() {
                let mut latter = String::new();
                while idx < chars.len() && (chars[idx].is_ascii_alphabetic() || chars[idx].is_digit(10)) {
                    latter += &chars[idx].to_string();
                    idx += 1;
                }
                tokens.push(Self::tokenize(&latter));
            } else if x.is_digit(10) {
                let mut num = String::new();
                while idx < chars.len() && chars[idx].is_digit(10) {
                    num.push(chars[idx]);
                    idx += 1;
                }
                tokens.push(Self::tokenize(&num));
            } else if x.is_whitespace() || x == '\n' {
                idx += 1;
            } else {
                tokens.push(Self::tokenize(&x.to_string()));
                idx += 1;
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
    fn test_scanner_basic1() {
        let input = "3 + 5 * (10 - 4)";
        let scanner = Scanner::new(input);
        
        let tokens = scanner.tokens;
        let expected_tokens = vec![
            Token::new(TokenType::Int, Some("3")),
            Token::new(TokenType::AddOp, Some("+")),
            Token::new(TokenType::Int, Some("5")),
            Token::new(TokenType::MulOp, Some("*")),
            Token::new(TokenType::OpenParenthes, Some("(")),
            Token::new(TokenType::Int, Some("10")),
            Token::new(TokenType::AddOp, Some("-")),
            Token::new(TokenType::Int, Some("4")),
            Token::new(TokenType::CloseParenthes, Some(")")),
        ];
    
        assert_eq!(tokens, expected_tokens, "expected: {expected_tokens:?}, actual: {tokens:?}");
    }
    
    #[test]
    fn test_scanner_str() {
        let input = "String: str <- \"Hello\";";

        let scanner = Scanner::new(input);

        let tokens = scanner.tokens;
        let expected_tokens = vec![
            Token::new(TokenType::StringType, Some("String")),
            Token::new(TokenType::Colon, Some(":")),
            Token::new(TokenType::Ident, Some("str")),
            Token::new(TokenType::Assign, Some("<-")),
            Token::new(TokenType::String, Some("Hello")),
            Token::new(TokenType::Semicolon, Some(";")),
        ];

        assert_eq!(tokens, expected_tokens, "expected: {expected_tokens:?}, actual: {tokens:?}");
    }

    #[test]
    fn test_scanner_basic2() {
        let input = "
fn Bool: fizzbuzz(Int: number)
  Int: i <- 1;
  /* i が 1 から number までの間繰り返す */
  while (i <= number) 
    // i が 3 の倍数かつ 5 の倍数の場合
    if (i % 15 = 0)
      println(\"FizzBuzz\");
    else if (i % 3 = 0)
      println(\"Fizz\");
    else if (i % 5 = 0)
      println(\"Buzz\");
    else
      println(i);
    endif
    i <- i + 1;
  endwhile
endfn
";
        let scanner = Scanner::new(input);

        let tokens = scanner.tokens;
        let expected_tokens = vec![
            Token::new(TokenType::FuncDef, Some("fn")),
            Token::new(TokenType::BoolType, Some("Bool")),
            Token::new(TokenType::Colon, Some(":")),
            Token::new(TokenType::Ident, Some("fizzbuzz")),
            Token::new(TokenType::OpenParenthes, Some("(")),
            Token::new(TokenType::IntType, Some("Int")),
            Token::new(TokenType::Colon, Some(":")),
            Token::new(TokenType::Ident, Some("number")),
            Token::new(TokenType::CloseParenthes, Some(")")),
            Token::new(TokenType::IntType, Some("Int")),
            Token::new(TokenType::Colon, Some(":")),
            Token::new(TokenType::Ident, Some("i")),
            Token::new(TokenType::Assign, Some("<-")),
            Token::new(TokenType::Int, Some("1")),
            Token::new(TokenType::Semicolon, Some(";")),
            Token::new(TokenType::While, Some("while")),
            Token::new(TokenType::OpenParenthes, Some("(")),
            Token::new(TokenType::Ident, Some("i")),
            Token::new(TokenType::RelOp, Some("<=")),
            Token::new(TokenType::Ident, Some("number")),
            Token::new(TokenType::CloseParenthes, Some(")")),
            Token::new(TokenType::If, Some("if")),
            Token::new(TokenType::OpenParenthes, Some("(")),
            Token::new(TokenType::Ident, Some("i")),
            Token::new(TokenType::MulOp, Some("%")),
            Token::new(TokenType::Int, Some("15")),
            Token::new(TokenType::RelOp, Some("=")),
            Token::new(TokenType::Int, Some("0")),
            Token::new(TokenType::CloseParenthes, Some(")")),
            Token::new(TokenType::Ident, Some("println")),
            Token::new(TokenType::OpenParenthes, Some("(")),
            Token::new(TokenType::String, Some("FizzBuzz")),
            Token::new(TokenType::CloseParenthes, Some(")")),
            Token::new(TokenType::Semicolon, Some(";")),
            Token::new(TokenType::Else, Some("else")),
            Token::new(TokenType::If, Some("if")),
            Token::new(TokenType::OpenParenthes, Some("(")),
            Token::new(TokenType::Ident, Some("i")),
            Token::new(TokenType::MulOp, Some("%")),
            Token::new(TokenType::Int, Some("3")),
            Token::new(TokenType::RelOp, Some("=")),
            Token::new(TokenType::Int, Some("0")),
            Token::new(TokenType::CloseParenthes, Some(")")),
            Token::new(TokenType::Ident, Some("println")),
            Token::new(TokenType::OpenParenthes, Some("(")),
            Token::new(TokenType::String, Some("Fizz")),
            Token::new(TokenType::CloseParenthes, Some(")")),
            Token::new(TokenType::Semicolon, Some(";")),
            Token::new(TokenType::Else, Some("else")),
            Token::new(TokenType::If, Some("if")),
            Token::new(TokenType::OpenParenthes, Some("(")),
            Token::new(TokenType::Ident, Some("i")),
            Token::new(TokenType::MulOp, Some("%")),
            Token::new(TokenType::Int, Some("5")),
            Token::new(TokenType::RelOp, Some("=")),
            Token::new(TokenType::Int, Some("0")),
            Token::new(TokenType::CloseParenthes, Some(")")),
            Token::new(TokenType::Ident, Some("println")),
            Token::new(TokenType::OpenParenthes, Some("(")),
            Token::new(TokenType::String, Some("Buzz")),
            Token::new(TokenType::CloseParenthes, Some(")")),
            Token::new(TokenType::Semicolon, Some(";")),
            Token::new(TokenType::Else, Some("else")),
            Token::new(TokenType::Ident, Some("println")),
            Token::new(TokenType::OpenParenthes, Some("(")),
            Token::new(TokenType::Ident, Some("i")),
            Token::new(TokenType::CloseParenthes, Some(")")),
            Token::new(TokenType::Semicolon, Some(";")),
            Token::new(TokenType::EndIf, Some("endif")),
            Token::new(TokenType::Ident, Some("i")),
            Token::new(TokenType::Assign, Some("<-")),
            Token::new(TokenType::Ident, Some("i")),
            Token::new(TokenType::AddOp, Some("+")),
            Token::new(TokenType::Int, Some("1")),
            Token::new(TokenType::Semicolon, Some(";")),
            Token::new(TokenType::EndWhile, Some("endwhile")),
            Token::new(TokenType::EndFuncDef, Some("endfn")),
        ];

        for i in 0..expected_tokens.iter().len() {
            let act = &tokens[i];
            let exp = &expected_tokens[i];
            assert_eq!(tokens[i], expected_tokens[i], "expected: {exp:?}, actual: {act:?}");
        }
    }

    #[test]
    fn test_scanner_whitespace() {
        let input = " 3+4^                                   5     ";
        let scanner = Scanner::new(input);
        
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
        let scanner = Scanner::new(input);

        let tokens = scanner.tokens;
        let expected_tokens = vec![
            Token::new(TokenType::Int, Some("1000000007")),
            Token::new(TokenType::MulOp, Some("%")),
            Token::new(TokenType::Int, Some("998244353")),
        ];

        assert_eq!(tokens, expected_tokens, "expected: {expected_tokens:?}, actual: {tokens:?}");
    }
}

