mod adder;
mod dynamic_call;

fn main() {
    // 动态分发
    dynamic_call::run();
    println!("======================");
    // 求和运算
    adder::run();
}
