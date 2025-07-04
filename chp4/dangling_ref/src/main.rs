//in rust the compiler prevents dangling reference.

fn main(){
    let ref_to_none = dangling();
}

fn dangling() -> &String{
    let str = String::from("Hello");
    &str
}

