#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    AddOp,
    MulOp,
    PowOp,
    Parenthes,
    Int,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub val: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, val: Option<&str>) -> Self {
        Token {
            token_type,
            val: val.map(|v| v.to_string()),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new(TokenType::AddOp, Some("+"));
        assert_eq!(token.token_type, TokenType::AddOp);
        assert_eq!(token.val, Some("+".to_string()));
    }
}
