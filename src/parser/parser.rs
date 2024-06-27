// use crate::scanner::scanner::Scanner;
// use crate::scanner::token::{Token, TokenType};
// use crate::parser::ast::ASTNode;


// pub fn parse(sc: &mut Scanner) -> ASTNode {
//     fn match_token(sc: &Scanner, token_type: TokenType) -> bool {
//         !sc.is_end() && sc.peek().token_type == token_type
//     }

//     fn take(sc: &mut Scanner, token_type: TokenType) -> Token {
//         let token = sc.next();
//         if token.token_type != token_type {
//             panic!(
//                 "Syntax Error: Expected {:?}, found {:?} with value {:?}",
//                 token_type, token.token_type, token.val
//             );
//         }
//         token
//     }

//     fn expr(sc: &mut Scanner) -> ASTNode {
//         let mut node = term(sc);
//         while match_token(sc, TokenType::AddOp) {
//             let op = take(sc, TokenType::AddOp);
//             node = ASTNode::BinOp {
//                 op,
//                 left: Box::new(node),
//                 right: Box::new(term(sc)),
//             };
//         }
//         node
//     }

//     fn term(sc: &mut Scanner) -> ASTNode {
//         let mut node = exponent(sc);
//         while match_token(sc, TokenType::MulOp) {
//             let op = take(sc, TokenType::MulOp);
//             node = ASTNode::BinOp {
//                 op,
//                 left: Box::new(node),
//                 right: Box::new(exponent(sc)),
//             };
//         }
//         node
//     }

//     fn exponent(sc: &mut Scanner) -> ASTNode {
//         let mut node = factor(sc);
//         while match_token(sc, TokenType::PowOp) {
//             let op = take(sc, TokenType::PowOp);
//             node = ASTNode::BinOp {
//                 op,
//                 left: Box::new(node),
//                 right: Box::new(factor(sc)),
//             };
//         }
//         node
//     }

//     fn factor(sc: &mut Scanner) -> ASTNode {
//         if match_token(sc, TokenType::Parenthes) && take(sc, TokenType::Parenthes).val == Some("(".to_string()) {
//             let node = expr(sc);
//             let close = take(sc, TokenType::Parenthes);
//             if close.val != Some(")".to_string()) {
//                 panic!(
//                     "Syntax Error: Expected {:?}, found {:?} with value {:?}",
//                     TokenType::Parenthes, close.token_type, close.val
//                 );
//             }
//             node
//         } else {
//             number(sc)
//         }
//     }

//     fn number(sc: &mut Scanner) -> ASTNode {
//         ASTNode::Number(take(sc, TokenType::Int))
//     }

//     expr(sc)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::scanner::scanner::Scanner;
//     use crate::scanner::token::{Token, TokenType};
//     use crate::parser::ast::ASTNode;

//     #[test]
//     fn test_parse_basic() {
//         let input = "3 + 5 * 2";
//         let mut scanner = Scanner::new(input.to_string());
//         let ast = parse(&mut scanner);

//         // AST structure for "3 + 5 * 2"
//         let expected_ast = ASTNode::BinOp {
//             op: Token::new(TokenType::AddOp, Some("+")),
//             left: Box::new(ASTNode::Number(Token::new(TokenType::Int, Some("3")))),
//             right: Box::new(ASTNode::BinOp {
//                 op: Token::new(TokenType::MulOp, Some("*")),
//                 left: Box::new(ASTNode::Number(Token::new(TokenType::Int, Some("5")))),
//                 right: Box::new(ASTNode::Number(Token::new(TokenType::Int, Some("2")))),
//             }),
//         };

//         assert_eq!(format!("{:?}", ast), format!("{:?}", expected_ast));
//     }

//     #[test]
//     fn test_parse_parentheses() {
//         let input = "(3 + 5) * 2";
//         let mut scanner = Scanner::new(input.to_string());
//         let ast = parse(&mut scanner);

//         // AST structure for "(3 + 5) * 2"
//         let expected_ast = ASTNode::BinOp {
//             op: Token::new(TokenType::MulOp, Some("*")),
//             left: Box::new(ASTNode::BinOp {
//                 op: Token::new(TokenType::AddOp, Some("+")),
//                 left: Box::new(ASTNode::Number(Token::new(TokenType::Int, Some("3")))),
//                 right: Box::new(ASTNode::Number(Token::new(TokenType::Int, Some("5")))),
//             }),
//             right: Box::new(ASTNode::Number(Token::new(TokenType::Int, Some("2")))),
//         };

//         assert_eq!(format!("{:?}", ast), format!("{:?}", expected_ast));
//     }
// }

