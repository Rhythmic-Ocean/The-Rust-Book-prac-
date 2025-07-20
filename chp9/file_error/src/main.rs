use std::fs::File;
use std::io::ErrorKind;


fn main(){
    let f = File::open("hlo.txt");

    let g = match f{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hlo.txt"){
                Ok(fc) => fc,
                Err(e)=> panic!("Error file creation {e:?}"),
            }
            _ => panic!("Error opening file"),
        }
    };
}


//alternative to match is .unwrap() and .except
//.unwrap() auto does the thing done by match for u
//.expect() gives u an extra option to have ur own message in

//eg:  let f = File::open("h.txt").unwarp();
//eg: let f = File::open("h.txt").expect("Hlo.txt file opening problem");