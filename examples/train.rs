extern crate bocchi;

fn main() {
    let a = 1;
    let b = 2;
    let c = bocchi::add(a, b);
    println!("{} + {} = {}", a, b, c);
}
