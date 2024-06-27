#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    FuncDef,
    EndFuncDef,
    OpenParenthes,
    CloseParenthes,
    If,
    Else,
    EndIf,
    While,
    Break,
    Continue,
    EndWhile,
    RelOp,

    AddOp,
    MulOp,
    PowOp,
    Assign,
    Colon,
    Semicolon,
    Comma,
    BoolType,
    Bool,
    IntType,
    Int,
    StringType,
    String,
    Ident,
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
