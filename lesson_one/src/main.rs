mod print_az_rev;
mod print_chars;

fn main() {
    println!("=====  a ~ Z  =====");
    print_az_rev::az_rev();

    println!("=====  A ~ z  =====");
    print_chars::print_az();
}
