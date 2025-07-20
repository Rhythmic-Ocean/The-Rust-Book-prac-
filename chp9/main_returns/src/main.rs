use std::error::Error;
use std::fs::File;


fn main() -> Result<(), Box<dyn Error>>{
    let g = File::open("hlo.txt")?;
    Ok(())
}



//NOTE: main() can only return 2 stuffs, () and Result<(),E>  