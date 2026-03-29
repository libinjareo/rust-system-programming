/// 此模块读取算术表达式中的字符，并将其转换为标记。
/// 允许的标记在 token 模块中定义。
//Standard lib
use std::iter::Peekable;
use std::str::Chars;

// 其他内部模块
use super::token::Token;

// Tokenizer 结构体包含一个对算术表达式进行遍历的可预览迭代器
pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>
}

// 创建一个新的 Tokenizer 实例
impl<'a> Tokenizer<'a> {
 pub fn new(expr: &'a str) -> Self {
     Tokenizer {
         expr: expr.chars().peekable()
     }
 }
}

// 为 Tokenizer 结构体实现 Iterator trait
// 通过这种方式，我们就可以在 tokenizer 上使用 next() 方法来获取算术表达式中的下一个 token
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let next_char = self.expr.next();
        match next_char {
            Some('0'..='9') => {
                let mut number = next_char?.to_string();
                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_numeric() || next_char == &'.' {
                        number.push(self.expr.next()?);
                    } else if next_char == &'(' {
                        return None;
                    }else {
                         break;
                    }
                }
                Some(Token::Num(number.parse::<f64>().unwrap()))
            },
            Some('+')  => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            Some(_) => None,
            None => Some(Token::EOF),
        }
    }
}

// tests 工具类
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_positive_integer() {
        let mut tokenizer:Tokenizer<'_> = Tokenizer::new("23");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(23.0));
    }

    #[test]
    fn test_decimal_number() {
        let mut tokenizer:Tokenizer<'_> = Tokenizer::new("45.6");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(45.6));
    }

    #[test]
    fn test_invalid_char() {
        let mut tokenizer = Tokenizer::new("#$%");
        assert_eq!(tokenizer.next(), None);
    }
}