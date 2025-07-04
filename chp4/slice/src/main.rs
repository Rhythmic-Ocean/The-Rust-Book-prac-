fn main() {
    let s = String::from("hello world");

    let hello = &s[..5];

    let world = &s[6..];

    println!("First word {}, second word {}", hello, world);
}
