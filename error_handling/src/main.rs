use std::fs::File;
use std::io::ErrorKind;

// K_22619
fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error_b| {
        if error_b.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error_d| {
                panic!("Problem creating the file: {:?}", error_d);
            })
        } else {
            panic!("Problem opening the file: {:?}", error_b);
        }
    });



    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });
}

// use std::{fs::File, io::Read};

// fn main() {
//     // panic!("HOLY SHIT!!!");
//     let mut buf = String::new();
//     let mut f = File::open("./config.fish").expect("打不开 hello.txt 呀~~~");
//     f.read_to_string(&mut buf).expect("读不出来呀~~~");
//     print!("{}", buf);
// }
