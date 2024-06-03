pub mod primative;

fn main() {
    println!("Hello, {:?}!", primative::MemorySlab::scramble(2097151));
    println!(
        "i parsed a {:?}!",
        "PX 1 2 cafe\n".parse::<primative::Line>().unwrap()
    );
}
