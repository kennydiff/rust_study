
#![allow(unused)]

fn main() {

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;    
    
    let x: i8 = 5;
    let y: Option<i8> = Some(5);  

    let sum = x + y.unwrap();
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
