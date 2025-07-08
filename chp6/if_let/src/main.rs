#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Dime,
    Nickle,
    Quater(UsState),
}



fn main() {
    let val = Some(4u8);
    if let Some(3) = val{
        println!("Three");
    } else if let Some(4) = val{
        println!("Four");
    }else {println!("None");}

    let coin = Coin::Quater(UsState::Alabama);
    some_fnc(coin);
}



fn some_fnc(coin: Coin){
    let mut count = 0;
    if let Coin::Quater(state) = coin{
        println!("The quater is from state {:?}", state);
    } else{
        count += 1;
    }
}
