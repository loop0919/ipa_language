// use crate::parser::ast::ASTNode;
// use crate::scanner::token::{Token, TokenType};

// pub struct Evaluator;

// impl Evaluator {
//     pub fn apply(node: &ASTNode) -> i32 {
//         match node {
//             ASTNode::Number(Token { val: Some(value), .. }) => {
//                 value.parse().expect("Failed to parse number")
//             },
//             ASTNode::BinOp { op, left, right } => {
//                 let left_val = Self::apply(left);
//                 let right_val = Self::apply(right);
//                 match op.token_type {
//                     TokenType::AddOp => match op.val.as_deref() {
//                         Some("+") => left_val + right_val,
//                         Some("-") => left_val - right_val,
//                         _ => panic!("Unknown operator: {:?}", op.val),
//                     },
//                     TokenType::MulOp => match op.val.as_deref() {
//                         Some("*") => left_val * right_val,
//                         Some("/") => left_val / right_val,
//                         Some("%") => left_val % right_val,
//                         _ => panic!("Unknown operator: {:?}", op.val),
//                     },
//                     _ => panic!("Unexpected token type: {:?}", op.token_type),
//                 }
//             },
//             _ => panic!("Invalid AST Node"),
//         }
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::scanner::scanner::Scanner;
//     use crate::parser::parser::parse;

//     #[test]
//     fn test_evaluator_basic() {
//         let input = "3 + 5 * 2";
//         let mut scanner = Scanner::new(input.to_string());
//         let ast = parse(&mut scanner);
//         let result = Evaluator::apply(&ast);
//         assert_eq!(result, 13);
//     }

//     #[test]
//     fn test_evaluator_parentheses() {
//         let input = "(3 + 5) * 2";
//         let mut scanner = Scanner::new(input.to_string());
//         let ast = parse(&mut scanner);
//         let result = Evaluator::apply(&ast);
//         assert_eq!(result, 16);
//     }

//     #[test]
//     fn test_evaluator_division() {
//         let input = "10 / 2";
//         let mut scanner = Scanner::new(input.to_string());
//         let ast = parse(&mut scanner);
//         let result = Evaluator::apply(&ast);
//         assert_eq!(result, 5);
//     }
// }

