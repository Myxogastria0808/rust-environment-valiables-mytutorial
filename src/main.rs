use std::env;

fn main() {
    let home = env::var("HOME").expect("HOME is not defined");
    println!("HOME={}", home);
}
