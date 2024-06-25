use crate::scanner::token::Token;

#[derive(Debug)]
pub enum ASTNode {
    BinOp { op: Token, left: Box<ASTNode>, right: Box<ASTNode> },
    Number(Token),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::token::{Token, TokenType};

    #[test]
    fn test_ast_node() {
        let token = Token::new(TokenType::Int, Some("3"));
        let ast_node = ASTNode::Number(token.clone());

        if let ASTNode::Number(t) = ast_node {
            assert_eq!(t, token);
        } else {
            panic!("ASTNode::Number expected");
        }
    }
}

