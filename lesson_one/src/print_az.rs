
pub fn print_chars_az(){
    // a ~ Z
    for i in ('Z'..='a').rev() {
        if i != 'Z' {
            println!("i == {i}");
        }
    }
}