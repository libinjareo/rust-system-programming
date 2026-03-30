/// 这里包含了一个枚举类型，用于表示令牌列表，并处理运算符的优先级规则。

// 通过Tokenizer（分词器）可从算术表达式中构建出的有效标记列表
#[derive(Debug, PartialEq,Clone)]
pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    Caret,
    LeftParen,
    RightParen,
    Num(f64),
    EOF,
}

// 根据运算符优先级规则排列的运算符顺序（从低到高）
#[derive(Debug,PartialEq,PartialOrd)]
pub enum OperPrec {
    DefaultZero,
    AddSub,
    MulDiv,
    Power,
    Negative,
}

// 检索给定算术运算符的运算符优先级
impl Token {
    pub fn get_oper_prec(&self) -> OperPrec {
        use OperPrec::*;
        use Token::*;
        match *self {
          Add | Subtract => AddSub,
          Multiply | Divide => MulDiv,
          Caret => Power,

          _ => DefaultZero,
        }
    }
}