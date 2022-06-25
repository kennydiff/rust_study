
#![allow(unused)]

fn main() {

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;    
    
    let x: i8 = 5;
    let y: Option<i8> = Some(5);  

    let sum = x + y.unwrap();
    
    let v = Some(3u8);
    // match v {
    //     Some(3) => println!("three"),
    //     _ => println!("not three"),
    // }

    if let Some(2) = v {  // K_22622 if let is a pattern match
        println!("three");
    } else {
        println!("not three");
    }
    
    // println!("{}",sum);
    //

    // #[derive(Debug)]
    // enum IpAddr {
    //     V4(u8, u8, u8, u8),
    //     V6(String),
    // }

    // let home = IpAddr::V4(127, 0, 0, 1);
    // let am = IpAddr::V6(String::from("2001:db8:2de::e13"));

    // // let loopback = IpAddr::V6(String::from("::1"));
    // println!("{:?}", home);
    // println!("{:?}", am);
}
