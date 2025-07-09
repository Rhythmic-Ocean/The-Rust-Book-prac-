fn main(){
    let h = String::from("hello world");    

    let word = first_word(&h);

    println!("Here's the first word: {}", word);
}
//NOTE: IN rust slicing stuff works based on the number of bytes in the string, not the number of character.
//So, while working with slicing string, we convert string to bytes cuz some characters are made of multiple bytes (u8)
fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes(); //when it's a single level referencing, it's auto dereferenced by the compiler, but for multi lvl referencing, u have to do it by urself
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }

    &s[..]

}