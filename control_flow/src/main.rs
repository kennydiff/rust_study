// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

fn main() {
    let mut number = 107; // Kenny@20220605 8/9/64/77 各种数字 都可能
    number = number % 24;
    match number {
        0 | 4 | 8 | 12 | 24 => println!("number is divisible by 4"),
        0 | 3 | 6 | 9 | 12 | 15 | 18 | 24 => println!("number is divisible by 3"),
        0 | 2 | 4 | 6 | 8 | 10 | 12 | 14 | 16 | 18 | 20 | 22 | 24 => {
            println!("number is divisible by 2")
        }
        _ => println!("number is not divisible by 4, 3, or 2"),
    }

    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;  // Kenny@20220605 从loop中跳出 用 break ，还可以接表达式，直接当作这个loop的整个返回值
    //     }
    // };
    // println!("The result is {}", result);

    let abc = [10u8, 20, 30, 40]; // Kenny@20220605 注意这里10u8的写法，强制声明为u8类型
    for item in abc {
        // Kenny@20220605 element以前在C++里也写作item
        println!("the value is: {}", item);
    }

    // for num in (1..4).rev() {    // Kenny@20220605 从1到4，不包括4，反向
    for num in (1..4).rev() {
        // Kenny@20220605 从1到4，不包括4，反向
        println!("{}!", num);
    }
    println!("LIFTOFF!!!");
}
