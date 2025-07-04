fn main(){
    let num = 5;

    if num < 5 {
        println!("True");
    }
    else if num > 5{
        println!("False");
    }
    else{
        println!("god knows");
    }
    //note, the condition must be bool, num < 5 return a bool value so it works. Just giving a non bool value, like a number would be bad

    //we can also use if-else with let:

    let condition = true;
    let x = if condition{
        5
    } else {
        6
    }; //note the expression inside the if and else block should be of the same type, if it was 5 and "six" instead, it would be pretty bad
    println!("x = {}", x);

    /*loop{
        //println!("hi?");
    //} infinite looping */

    //while

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIfTOFF!!!");



    let a = [10, 20, 30, 40, 50];

    for element in a.iter(){
        println!("the val is: {}", element);
    }

    for numb in (1..=4).rev(){ //.rev() reverses thru the number 1 to 4
        println!("{}!", numb);
    }

    println!("Liftoff!!");


}