use std::ops::Add;

// 自定义类型
// #[derive(Debug)]
struct Number {
    x: i32,
}

// 实现 Add trait
impl Add for Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        Number {
            x: self.x + other.x,
        }
    }
}

// Trait 定义
trait Sum {
    fn sum(&self);
}

// 实现 Trait
impl Sum for Number {
    fn sum(&self) {
        println!("Number Sum is : {}", self.x);
    }
}

// 函数接受 Trait Object 作为参数并调用类型方法
fn dynamic_call(obj: &dyn Sum) {
    obj.sum();
}
pub fn run() {
    let num1 = Number { x: 5 };
    let num2 = Number { x: 10 };

    // 求和运算
    let res = num1 + num2;
    dynamic_call(&res);
}
