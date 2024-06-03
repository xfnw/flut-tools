pub mod primative;
pub mod protocol;

fn main() {
    println!("Hello, {:?}!", primative::scramble(2097151));
    println!(
        "i parsed a {:?}!",
        "PX 1 2 cafe\n".parse::<protocol::Line>().unwrap()
    );
}
