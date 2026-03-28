//! first-lib crate
//!
//! 这是一个包含处理进程相关功能的库，它使得这些任务变得更加便捷

use std::process;
use chrono as time;


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

///此函数可获取当前可执行文件的进程标识符。它会返回一个非零数字
///```
/// fn get_id() {
///    let x = first_lib::get_process_id();
///     println!("{}",x);
/// }
///
/// ```
pub fn get_process_id() -> u32 {
    println!("time now is {:?}",time::Utc::now());
    process::id()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_if_process_id_is_returned() {
        assert!(get_process_id() > 0);
    }
}
