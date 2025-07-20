use std::fs::File;

fn main(){
    let f = File::open("hello.txt");

    let s = match f{
        Ok(file) => file,
        Err(error) => panic!("Problem opening file! {}", error),
    };
}