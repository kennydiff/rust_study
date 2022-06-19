fn main() {
    let hello_0 = String::from("السلام عليكم");
    let hello_1 = String::from("Dobrý den");
    let hello_2 = String::from("Hello");
    let hello_3 = String::from("שָׁלוֹם");
    let hello_4 = String::from("नमस्ते");
    let hello_5 = String::from("こんにちは");
    let hello_6 = String::from("안녕하세요");
    let hello_7 = String::from("你好");
    let hello_8 = String::from("Здравствуйте");

    // println!("length of \"السلام عليكم\" is: {}", hello_0.len());
    // println!("length of \"Dobrý den\" is: {}", hello_1.len());
    // println!("length of \"Hello\" is: {}", hello_2.len());  // KEN_220618 英文的UTF-8字符串每个字符都是占用1个字节
    // println!("length of \"שָׁלוֹם\" is: {}", hello_3.len());
    // println!("length of \"नमस्ते\" is: {}", hello_4.len());
    // println!("length of \"こんにちは\" is: {}", hello_5.len());
    // println!("length of \"안녕하세요\" is: {}", hello_6.len());
    // println!("length of \"你好\" is: {}", hello_7.len());  // KEN_220618 中日韩的UTF-8字符串每个字符是占用3个字节
    // println!("length of \"Здравствуйте\" is: {}", hello_8.len());

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s2_5 = String::from(" OK?");
    // let s2 = "wrold!";
    let s3 = s1 + &s2 + &s2_5; // KEN_220618 注意 s1 被移动了，不能继续使用
    println!("{}", s3);
    // println!("{}", s1);  // KEN_220618
    // println!("{}", s2);

    let hello = "您好,世界。 哈哈哈，中英文混合的字符串,English";
    // let s = &hello[6..10];
    // println!("{}", s);
    for c in hello.chars() {
        println!("{}", c);
    }
    println!("{}", hello.chars().count());  // KEN_220618 取出字符串的个数(一个汉字算一个，一个英文字符算一个)
    println!("{}", hello.len());
}
