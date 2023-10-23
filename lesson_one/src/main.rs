use crate::print_chars::print_chars_az;

mod print_chars {
    pub mod print_chars_az;
}

fn main() {
    // a ~ Z
    for i in ('Z'..='a').rev() {
        if i != 'Z' {
            println!("i == {i}");
        }
    }

    //子模块 A ~ z
    print_chars_az::print_chars_az()
}
