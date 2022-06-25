use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);



    // K_22625 以下是实验代码
    let add_one_v3 = |x|x;
    println!("{}", add_one_v3(String::from("STD")));


    // K_22625 闭包定义会为每个参数和返回值推断一个具体类型
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);  // K_22625 上面一行已经将闭包的参数&返回类型已经推断确定了，这里又写不一样的参数类型，则不允许编译通过

}
