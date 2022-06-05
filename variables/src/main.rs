// use std::io;

fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);


    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    let sss = "Hello, world! BBB".to_string();
    println!("{}", sss);

    // let mut guess = String::new();
    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");
    // println!("You guessed:{}", guess);
    // let mut _greeting = guess.contains("Hello");
    // println!("{}", _greeting);

    
    println!("{}", MAX_POINTS);


    let x = 5;
    println!("The value of x is: {}", x);
    let x = x+1;
    println!("The value of x is: {}", x);
    let x = x*2;
    let x = x.to_string() + "_BBQ";  // 变成字符串，然后加上后缀...这样也行...
    println!("The value of x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len();  // Kenny@20220605 usize 根据计算机架构(字长)来确定
    println!("{}",spaces);

}

const MAX_POINTS:u32 = 1030_0780;  // Kenny@20220605 const定义放到这里也是OK的
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;