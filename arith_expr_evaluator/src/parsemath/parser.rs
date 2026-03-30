/// 此程序会读取由分词器(Tokenizer)返回的标记(tokens)，并将其转换为抽象语法树(AST)。

use std::fmt;
use crate::parsemath::token::{OperPrec, Token};
use crate::parsemath::tokenizer::Tokenizer;
use crate::parsemath::ast::Node;

// Parser 结构体
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

// Parser 的 公共方法
impl <'a> Parser<'a> {
    // 创建一个新的Parser实例
    pub fn new(expr: &'a str) -> Result<Self,ParseError> {
        let mut lexer = Tokenizer::new(expr);
        let cur_token = match lexer.next() {
            Some(t) => t,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };

        Ok(Parser{
            tokenizer:lexer,
            current_token: cur_token,
        })
    }

    // 将算术表达式作为输入并返回AST
    pub  fn parse(&mut self) -> Result<Node,ParseError> {
        let ast = self.generate_ast(OperPrec::DefaultZero);
        match ast {
            Ok(ast) => Ok(ast),
            Err(e) => Err(e),
        }
    }
}

// Parser的私有方法
impl<'a> Parser<'a> {

    fn get_next_token(&mut self) -> Result<(),ParseError> {
        // 从算术表达式中获取下一个标记，并将其设置为Parser 结构体中的 current_token 字段值。
        let next_token = match self.tokenizer.next() {
            Some(t) => t,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        self.current_token = next_token;
        Ok(())
    }

    // 核心方法，递归调用
    fn generate_ast(&mut self,oper_prec: OperPrec) -> Result<Node,ParseError> {
        let mut left_expr = self.parse_number()?;
        while oper_prec < self.current_token.get_oper_prec() {
            if self.current_token == Token::EOF {
                break;
            }
            let right_expr = self.convert_token_to_node(left_expr.clone())?;
            left_expr = right_expr;
        }
        Ok(left_expr)
    }

    // 检查平衡括号
    fn check_paren(&mut self,expected:Token) -> Result<(), ParseError> {
        if expected == self.current_token {
            self.get_next_token()?;
            Ok(())
        }else {
            Err(ParseError::InvalidOperator(format!("Expected {:?},got {:?}", expected, self.current_token)))
        }
    }

    // 构建用于表示数字的抽象语法树节点，同时在处理括号时考虑负号前缀的情况
    fn parse_number(&mut self) -> Result<Node,ParseError> {
        let token = self.current_token.clone();
        match token {
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Num(i) => {
                self.get_next_token()?;
                Ok(Node::Number(i))
            }
            Token::LeftParen => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_paren(Token::RightParen)?;
                if self.current_token == Token::LeftParen {
                    let right = self.generate_ast(OperPrec::MulDiv)?;
                    return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
                }

                Ok(expr)
            }
            _ => Err(ParseError::UnableToParse("Unable to parse".to_string())),
        }
    }

    fn convert_token_to_node(&mut self,left_expr:Node) -> Result<Node,ParseError> {
        match self.current_token {
            Token::Add => {
                self.get_next_token()?;
                // 获取右边的表达式
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Subtract => {
                self.get_next_token()?;
                // 获取右边的表达式
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Subtract(Box::new(left_expr),Box::new(right_expr)))
            }
            Token::Multiply => {
                self.get_next_token()?;
                //获取右边的表达式
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Multiply(Box::new(left_expr),Box::new(right_expr)))
            }
            Token::Divide => {
                self.get_next_token()?;
                // 获取右边表达式
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Divide(Box::new(left_expr),Box::new(right_expr)))
            }
            Token::Caret => {
                self.get_next_token()?;
                // 获取右边表达式
                let right_expr = self.generate_ast(OperPrec::Power)?;
                Ok(Node::Caret(Box::new(left_expr),Box::new(right_expr)))
            }
            _ => Err(ParseError::InvalidOperator(format!("Please enter valid operator: {:?}", self.current_token))),
        }
    }

}
// 为Parse 构建的自定义错误
#[derive(Debug)]
pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result {
        match &self {
            ParseError::UnableToParse(e) => write!(f, "Unable to parse: {}", e),
            ParseError::InvalidOperator(e) => write!(f,"Invalid operator: {}", e),
        }
    }
}

// 自动将AST模块中的任何Boxed错误转换为ParseError
impl std::convert::From<std::boxed::Box<dyn std::error::Error>> for ParseError {
    fn from(_evalerr: std::boxed::Box<dyn std::error::Error>) -> Self {
        return ParseError::UnableToParse("Unable to parse".into());
    }
}

#[cfg(test)]
mod tests {
    use crate::parsemath::ast::Node::{Add,Multiply,Number};
    use super::*;

    #[test]
    fn test_addition() {
        let mut  parser = Parser::new("1+2").unwrap();
        let expected = Add(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn test_multiply() {
        let mut parser = Parser::new("1*(2+3)").unwrap();
        let expected = Multiply(Box::new(Number(1.0)),Box::new(Add(Box::new(Number(2.0)), Box::new(Number(3.0)))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
}