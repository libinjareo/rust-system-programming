use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;

// ---------- 1. 基础类型与结构体 ----------
struct Student  {
    id:u32, // 整数类型
    name:String, //字符串类型
    age:u8, //基础类型
    scores:Vec<u32>, //动态数组（集合类型）
}

// ---------- 2. 生命周期注解 ----------
// 这个结构体包含一个引用，需要明确生命周期
#[derive(Debug)]
struct Report<'a> {
    student_name: &'a str, //引用的字符串
    total_score: u32,
}

impl <'a> Report<'a> {
    // 生命周期省略规则在这里适用，但为了明确可以写出来
    fn new(name:&'a str,total:u32) -> Self {
        Report {
            student_name:name,
            total_score:total,
        }
    }
}

// ---------- 3. 泛型函数 ----------
// 计算任意数字类型集合的总和（要求类型实现加法+Copy）
fn sum_generic<T:std::ops::Add<Output=T> + Copy> (items: &[T]) -> T {
    let mut total = items[0];
    for &item in &items [1..] {
        total = total + item;
    }
    total
}

// 泛型函数计算平均值（需要从整数转换为浮点数）
fn average_generic <T:Into<f64> + Copy>(items: &[T]) -> f64 {
    let sum:f64 = items.iter().map(|&x| x.into()).sum();
    sum / items.len() as f64
}

// ---------- 4. 借用与所有权演示 ----------
fn demonstrate_ownership() {
    let s1 = String::from("hello");
    let s2 = s1; // s1的所有权转移到s2
    //println!("{}",s1) // 编译错误：s1已经失效
    println!("Ownership moved: s2= {}",s2);

    let s3 = String::from("world");
    let len = calculate_length(&s3); // 借用（不可变引用）
    println!("The length of '{}' is {}",s3,len);

    let mut s4 = String::from("hello");
    change(&mut s4); //可变借用
    println!("After change: {}",s4);
}

// ---------- 5. 集合类型与动态数组操作 ----------
fn manipulate_collections(students:&mut Vec<Student>) { 
    // 添加新学生
    students.push(Student {
        id:3,
        name:"Charlie".to_string(),
        age:20,
        scores:vec![85,90,88],
    });
    
    // 遍历并修改，为每个学生增加一门成绩
    for student in students.iter_mut() {
        student.scores.push(95); // 动态数组增加元素
    }
    
    //使用HashMap 统计每个学生的总分
    let mut total_scores:HashMap<String,u32> = HashMap::new();
    for student in students.iter() {
        let total:u32 = student.scores.iter().sum();
        total_scores.insert(student.name.clone(), total);
    }

    println!("Total scores : {:?}", total_scores);

    // 使用泛型函数计算平均分(转换为f64)
    let all_scores:Vec<u32> = students.iter().flat_map(|s| s.scores.clone()).collect();
    let avg = average_generic(&all_scores);
    println!("Average Generic Value: {:.2}",avg);
}

// ---------- 6. 并发：使用多线程和共享状态 ----------
fn concurrency_demo(students: Vec<Student>) {
    // 将Vec包装到Arc<Mutex>中，以便在线程间安全共享和修改
     let shared_students = Arc::new(Mutex::new(students));

    let mut handlers = vec![];

    //创建3个线程，每个线程计算某个学科的平均分（模拟不同任务）
    for thread_id in 0..3 {
        let data = Arc::clone(&shared_students);
        let handle = thread::spawn(move || {
            // 锁定 Mutex 以获取内部数据的访问权
            let students_guard = data.lock().unwrap();
            // 模拟计算：每个线程计算所有学生的第thread_id门成绩的平均分
            //注意：这里假设每个学生至少有3门成绩（在前面的操作已经添加）
            let mut sum = 0;
            let mut count = 0;
            for student in students_guard.iter() {
               if let Some(&score) = student.scores.get(thread_id){
                    sum += score;
                    count += 1;
                }
            }
            let average = if count > 0 { sum as f64 / count as f64 } else { 0.0 };
            println!("Thread {}: Average score of subject {} = {:.2}",thread_id,sum,average);
        });

        handlers.push(handle);
    }

    //等待所有线程结束
    for handler in handlers {
        handler.join().unwrap();
    }
}

// ---------- 7. 生命周期使用示例 ----------
fn lifetime_demo() {
    let student_name = String::from("Alice");
    let report = {
        let name_ref = &student_name;
        Report::new(name_ref,270)
    };
    println!("Report: {:?}", report);
    // report的生命周期不能超过student_name，但是这里的student_name依然有效，所以没有问题
}

fn calculate_length(s: &String) -> usize {
    s.len() // 返回长度，不获取所有权
}

fn change(s: &mut String) {
    s.push_str(", world");
}



fn main() {
    // 1. 基础类型演示
    let _integer: i32 = 42;
    let _float: f64 = 3.14;
    let _bool: bool = true;
    let _char: char = 'R';
    let _tuple: (i32, f64, &str) = (10, 2.5, "rust");
    let _array: [i32; 3] = [1, 2, 3];
    println!("Basic types demo done.");

    // 2. 所有权与借用演示
    demonstrate_ownership();

    // 3. 创建学生集合（Vec<Student>）
    let mut students = vec![
        Student {
            id: 1,
            name: "Alice".to_string(),
            age: 20,
            scores: vec![85, 92, 78],
        },
        Student {
            id: 2,
            name: "Bob".to_string(),
            age: 21,
            scores: vec![88, 76, 90],
        },
    ];

    // 4. 集合类型操作
    manipulate_collections(&mut students);

    // 5. 泛型函数测试
    let numbers = [10, 20, 30];
    let sum = sum_generic(&numbers);
    let avg = average_generic(&numbers);
    println!("Sum of numbers: {}, Average: {:.2}", sum, avg);

    // 6. 生命周期演示
    lifetime_demo();

    // 7. 并发演示
    concurrency_demo(students);
}
