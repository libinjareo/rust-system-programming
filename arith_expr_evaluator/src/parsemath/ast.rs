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