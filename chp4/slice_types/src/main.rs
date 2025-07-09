fn main() {
    let words = String::from("hello world");
    let space_at = first_word(&words);
    println!("Space at word: {}", space_at);
}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();//has an array of the given string in byte form
    for byte in bytes{
        println!("{}", byte); //printing each array
    }
    for (i, &item) in bytes.iter().enumerate(){ //&item is each item in byte while i is it's corresponding position
    //.enumerate() returns each element as a part of a tuple
        if item == b' '{ //b' ' is basically space in byte format, it's like 32 in u8 format
            return i;
        }
    }
    s.len()
}
//NOTE IN RUST WE CAN USE TUPLES TO DESTRUCTURE PATTERNS!!!
