fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}", six, none);
    let matcher = placeholder_eg(3u8);
    println!("{:?}", matcher);
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1), 
//NOTE: MATCHES IN RUST ARE EXHAUSTIVE. If u did not cover all cases (some and none in this case), it will be a compile time error
    }
}

//NOTE: data in struct is stored in stack (if all varients are primitive), but by default when u assign a struct instance to another, the data is moved
//if u want to copy the data of an instance, and all varients of the struct are primitive type, give it the copy trait
//can't give copy trait if the struct has heap allocated data types. Althought the pointer, len and cap of the heaper is in stack. The heaper will be moved stil. And the original instance wil be unusabled
//Same for enums

//_ placeholder for match:


fn placeholder_eg(matcher: u8)->Option<String>{
    match matcher {
        1 => Some(String::from("one")),
        2 => Some(String::from("one")),
        3 => Some(String::from("one")),
        4 => Some(String::from("one")),
        _ => None,   //_ placeholder means any other value, since match is exhaustive, this is mandatory
    }
} 