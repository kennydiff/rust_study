fn main() {
    let stack_i8: i8 = 10;
    let _stack_f32: f32 = 20.0;
    let _stack_bool: bool = true;
    let _stack_char: char = 'a';
    // println!("{}",y);
    // if stack_i8 == 3 {
    //     let inside_scope = 9;
    //     println!("{}",inside_scope);
    // }
    // println!("{}",stack_i8);

    let stack_i8_2 = stack_i8;
    // println!("{}",stack_i8);
    // println!("{}",stack_i8_2);

    let heap_i8: Box<i8> = Box::new(30);
    let heap_i8_2 = &heap_i8;
    // let heap_i8_2: Box<i8> = Box::new(32);
    // println!("heap_i8: {}",heap_i8);
    // println!("heap_i8_2: {}",heap_i8_2);


    let mut some_string: String =  String::from("Hey Kenny~");
    let some_str: &str = "COOL";

    let some_string2 = some_proc(some_string,some_str);
    println!("{} {}",some_string2,some_str);
}

fn some_proc(mut param_a: String,param_b:&str) -> String{
    println!("{} {}",param_a,param_b);
    param_a = String::from("Hey Kenny~ Kiss.");
    return param_a;
}