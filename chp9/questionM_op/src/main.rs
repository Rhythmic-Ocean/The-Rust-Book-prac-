use std::fs::File;
use std::io::{self, Read};


fn main(){
    let smthing = uname();
    match smthing{
        Ok(s) => println!("{s}"),
        Err(e) => println!("Some error {e:?}"),
    }
}


fn uname() -> Result<String, io::Error>{
    let mut s = String::new();
    let mut f = File::open("hlo.txt")?; //returns the entire funtion with error e if error occurs, return the value locally if success
    f.read_to_string(&mut s)?;
    Ok(s)
}

//even shorter way is to:
//let mut s = String::new();
//File::open("hlo.txt")?.read_to_string(&mut s)?;
//Ok(username)



//SHORTEST WAY
//use std::io::{self, File}

//fn uname() -> Result<String,io::Error>{
//  fs::read_to_string("hlo.txt")
//}

//this opens the file reads it and returns the string (or error). It's built in