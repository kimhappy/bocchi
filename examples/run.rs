extern crate bocchi;

fn main() {
    let a = 10;
    let b = 20;
    let c = bocchi::add(a, b);
    println!("{} + {} = {}", a, b, c);
}
