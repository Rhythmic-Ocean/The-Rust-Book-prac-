fn main() {
    let words = String::from("hello world");
    let space_at = first_word(&words);
    println!("Space at word: {}", space_at);
}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();
    for byte in bytes{
        println!("{}", byte);
    }
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}
