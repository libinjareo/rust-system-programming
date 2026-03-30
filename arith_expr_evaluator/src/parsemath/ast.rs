use std::error;

/// 此程序包含了可构建的合法抽象语法树节点列表，并且还能够对抽象语法树进行评估以计算出一个值。


// 由解析器可构建的允许使用的抽象语法树节点列表
// Tokens 可以是算术运算符，也可以是数字。
#[derive(Debug,Clone,PartialEq)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(f64)
}

// 给定AST,计算数值
pub fn eval(expr:Node) -> Result<f64,Box<dyn error::Error>> {
    use Node::*;
    match expr {
        Number(i) => Ok(i),
        Add(expr1,expr2) => Ok(eval(*expr1)? + eval(*expr2)?),
        Subtract(expr1,expr2) => Ok(eval(*expr1)? - eval(*expr2)?),
        Multiply(expr1,expr2) => Ok(eval(*expr1)? * eval(*expr2)?),
        Divide(expr1,expr2) => Ok(eval(*expr1)? / eval(*expr2)?),
        Caret(expr1,expr2) => Ok(eval(*expr1)?.powf( eval(*expr2)?)),
        Negative(expr1) => Ok(-eval(*expr1)?),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr1() {
        use crate::parsemath::parser::Parser;

        let ast = Parser::new("1+2-3").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 0.0);
    }

    #[test]
    fn test_expr2() {
        use crate::parsemath::parser::Parser;

        let ast = Parser::new("3+2-1*5/4").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 3.75);
    }
}