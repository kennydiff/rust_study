use std::thread;
use std::time::Duration;



struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg); // K_22626 调用闭包
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let v2: Vec<_> = vec![1, 2, 3];
    let v3: Vec<_> = v2.iter().map(|x| x * 2).collect(); // K_22628 |x| x * 2 这里是个闭包,匿名函数
                                                         // println!("{}", tot);

    let v1 = vec![1, 2, 3, 4, 5];
    // let v1_iter = v1.iter();
    for item in v1.iter() {
        println!("{}", item);
    }

    /*
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;
        generate_workout(simulated_user_specified_value, simulated_random_number);

        // K_22625 以下是实验代码
        // let add_one_v3 = |x|x;
        // println!("{}", add_one_v3(String::from("STD")));

        // K_22625 闭包定义会为每个参数和返回值推断一个具体类型
        // let example_closure = |x| x;
        // let s = example_closure(String::from("hello"));
        // let n = example_closure(5);  // K_22625 上面一行已经将闭包的参数&返回类型已经推断确定了，这里又写不一样的参数类型，则不允许编译通过

        // #[test]
        // fn call_with_different_values() {
        //     let mut c = Cacher::new(|a| a);

        //     let v1 = c.value(1);
        //     let v2 = c.value(2);

        //     assert_eq!(v2, 2);

        let x = 4;
        let equal_to_x = |z| z == x;
        let y = 4;
        assert!(equal_to_x(y));

        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;
        // println!("can't use x here: {:?}", x);
        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));
    */
}
