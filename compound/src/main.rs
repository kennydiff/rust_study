fn main() {
    // let tup:(i32,f64,u8,char)=(500,6.4,1,'D');
    // let (x,y,z,s) = tup;
    // println!("{},{},{},{}",x,y,z,tup.3);

    let a = [1,2,3,4,5];
    let months = [  // Kenny@20220605 : [&str; 12] 表明了这个数组是一个字符串地址(指针)数组,有0..11，一共12个成员
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a=[3;5];  // Kenny@20220605 注意3和5之间是分号;  相当于：let a=[3,3,3,3,3];
    let index = [11,12,13,14];
    let month = months[index[0]];
    

    another_function(110);  //argument is 5
    let x = 5;
    
    let y = {
        let mut x = 1;
        x += 6;
        x+5
    };
    // let y = ();
    // println!("{}", y);
    let x = plus_five(9);
    println!("{}", x);
}

fn another_function(x: u8){  // parameter is x, type is i32
    println!("the value of x is:{}", x);
}

fn plus_five(x: i32) -> i32 {
    return x + 5
    // return x + 5;
}   