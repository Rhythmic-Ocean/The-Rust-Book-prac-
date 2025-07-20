use std::io::{self, Read};
use std::fs::File;


fn main(){

    let m = uname();
    println!("{:?}", m);
}

fn uname() -> Result<String, io::Error> {
    let f = File::open("hlo.txt");
    let mut g = match f{
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut st = String::new();

    match g.read_to_string(&mut st){
        Ok(d)=> Ok(st),
        Err(e)=> Err(e),
    }


}