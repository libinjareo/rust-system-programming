use first_lib::{add, get_process_id};

fn main() {
    println!("Going to call library function!");
    println!("library add function call result:{}",add(2,3));

    println!("process id is {}",get_process_id());
}