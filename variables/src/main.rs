use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    let mut sss: String = String::new();
    sss = "Hello, world!".to_string();
    println!("{}", sss);

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed:{}", guess);
    let mut _greeting = guess.contains("Hello");
    println!("{}", _greeting);
}
