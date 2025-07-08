enum Option<T>{ //use this when there is a chance the value can be null
    Some(T),
    None
}
enum Coin{
    Penny,
    Nickle,
    Dime,
    Quater
}

fn main() {
    let some_num = Some(5);
    let some_string = Some("a string");

    let abs_num = Option<i32> = None;
}

fn value_in_cents(coin: Coin) -> u32{
    match coin{
        //BIG DIFFERENCE BETWEEN IF AND MATCH: if needs to return boolean but match does not!!
        Coin::Penny => { //use curly braces if the match stuff is longer
            println!("Lucky penny!");
            1
        },
        Coin:: Nickle => 5,
        Coin::Dime => 10,
        Coin::Quater => 25
    }
}

//Option<i32> and i32 types are not the same!
//u have to unwrap Option<i32>, check if it has some or None, and handle each case individually!!
