/// 这是算术表达式求值器的主要命令行应用程序

// 注册 parsemath 模块
mod parsemath;

use std::io;
use parsemath::ast;
use parsemath::parser::{ParseError,Parser};

// 函数调用解析器并计算表达式
fn evaluate(expr:String) -> Result<f64,ParseError> {
    let expr = expr.split_whitespace().collect::<String>(); //移除空格
    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;
    println!("The generated AST is {:?}", ast);

    Ok(ast::eval(ast)?)
}

// Main函数从命令行读取算术表达式并显示结果和错误
// 它调用evaluate函数来执行计算
fn main() {
    println!("Hello! Welcome to Arithmetic expression evaluator.");
    println!("You can calculate value for expression such as 2*3+(4-5)+2^3/4. ");
    println!("Allowed numbers: positive, negative and decimals.");
    println!("Supported operations: Add, Subtract, Multiply, Divide, PowerOf(^). ");
    println!("Enter your arithmetic expression below:");

    loop {
        let mut input = String::new();
        match io::stdin().read_line( & mut input ) {
            Ok(_) => {
                match evaluate(input) {
                    Ok(val) => println!("The computed number is {:?}", val),
                    Err(_) => println!("Error in evaluating expression,Please enter valid expression\n"),
                };
            }
            Err(error) => println!("error: {}", error)
        }
    }
}
