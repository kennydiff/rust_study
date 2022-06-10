fn main() {
    let sb = String::from("hello world");
    println!("The first word of '{}' is '{}'.", sb, first_word(&sb));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }    
    // Kenny@20220610 SO COOL -- Github Copilot, SO COOL -- Rust
    // Kenny@20220610 可以被理解为rust因为安全原因，要求所有所有分支可能都要覆盖返回值
    &s[..] // Kenny@20220610 It can be understood that Rust requires all scopes to cover the return value for security reasons : return the whole string if no space is found.
}

// let len = calculate_length(&mut s1);
// first_word(&s1);


/*  // Kenny@20220610 
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]    // Kenny@20220610 这里没有跳出函数吗？ 已经跳出了，何必最后还加那么一句废话。。。 fuck...
        }
    }    
    &s[..]    // Kenny@20220610 没有这句的话,这里会报错...类型不匹配，，如何返回？表达式？
} */

// fn calculate_length(s: &mut String) -> usize {
//     s.push('!');
//     s.len() // Kenny@20220608 这里s是一个引用，没有对传入参数的Heap中的数据的所有权,所以离开作用域的时候，Heap里的空间不会被释放
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// fn main() {
//     let s1 = gives_ownership(); // gives_ownership 将返回值
//                                    // 转移给 s1
//                                 // println!("{}", s1);
//     let s2 = String::from("hello"); // s2 进入作用域

//     let s3 = takes_and_gives_back(s2); // s2 被移动到
//                                        // takes_and_gives_back 中,
//                                        // 它也将返回值移给 s3
//     println!("{}", s3);
// } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
//   // 所以什么也不会发生。s1 离开作用域并被丢弃

// fn gives_ownership() -> String {
//     // gives_ownership 会将
//     // 返回值移动给
//     // 调用它的函数

//     let some_string = String::from("yours"); // some_string 进入作用域.

//     // 返回 some_string
//     // 并移出给调用的函数
//     println!("{}", some_string);
//     some_string
// }

// // takes_and_gives_back 将传入字符串并返回该值
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string 进入作用域
//     a_string // 返回 a_string 并移出给调用的函数
// }

// fn main() {
//     // println!("Hello, world!");
//     let mut s_old = "bitch...";
//     s_old = "ddq";
//     // println!("{}",s);

//     let mut s = String::from("hello");
//     s.push_str(", world!");  // push_str() 在字符串后追加字面值,不是append,而是push
//     s.push('?');
//     // println!("{}", s);  // 将打印 `hello, world!`
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("{} ...clone... {}", s1,s2);  // Kenny@20220607 这里s1已经无法访问，被Move(移动)了

//     let x = 5;
//     let y = x;  // Kenny@20220607 这里y = x.clone(); 是废话,x是Stack里的数据，默认完整复制,没有Move/ Deep Copy / Clone的必要
//     println!("x = {}, y = {}", x, y);

// }  // Kenny@20220607 "s"的作用域结束,被释放，还给了操作系统的堆空间

// fn main() {
//     let s = String::from("hello");  // s 进入作用域
//     takes_ownership(s);             // s 的值移动到函数里 ...
//     // println!("{}",s);                                // ... 所以到这里不再有效
//     let x = 5;                      // x 进入作用域

//     makes_copy(x);                  // x 应该移动函数里，
//     println!("{}",x);                                 // 但 i32 是 Copy 的，
//                                     // 所以在后面可继续使用 x

// } // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
//   // 没有特殊之处

// fn takes_ownership(some_string: String) { // some_string 进入作用域
//     println!("{}", some_string);
// } // 这里，some_string 移出作用域并调用 `drop` 方法。
//   // 占用的内存被释放

// fn makes_copy(some_integer: i32) { // some_integer 进入作用域
//     println!("{}", some_integer);
// } // 这里，some_integer 移出作用域。没有特殊之处
