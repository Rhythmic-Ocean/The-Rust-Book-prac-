//in rust the compiler prevents dangling reference.
//ERROR, this will not run

fn main(){
    let ref_to_none = dangling();
}

fn dangling() -> &String{
    let str = String::from("Hello");
    &str
}//here str's scope ends so outside this function &str holds pointer to a place that has already been freed
//would have been a runtime error in C, but the compiler catches it in rust
//instead if u returned the string directly, it would do far better



//NOTE: You can only have 1 mutable reference or any number of immutable ref, not both

