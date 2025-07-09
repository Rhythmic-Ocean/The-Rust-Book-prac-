#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    //-snip
}

#[derive(Debug)] //NOTE: It must be atop evevery enum/ struct that u intend to print using debug trait
enum Coin{
    Penny,
    Nickle,
    Dime,
    Quater(UsState)
}
fn main() {

    let coin = Coin::Quater(UsState::Alabama);
    let val = value_in_cents(&coin);
    println!("The {:?} coin has value {}.", coin, val);

}


fn value_in_cents(coin:&Coin) -> u32{
    match coin{
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quater(state) =>{
            println!("It's from {:?} state!", state);
            25
        }
    }
}