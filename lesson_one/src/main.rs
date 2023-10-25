
mod print_az;
mod print_chars;

fn main() {
    // a ~ Z
    print_az::print_chars_az();

    //子模块 A ~ z
    print_chars::print_a_to_z();
}
