mod adder;
mod trait_object;

fn main() {
    // 动态分发
    trait_object::run();

    println!("======================");

    // 求和运算
    adder::run();
}
