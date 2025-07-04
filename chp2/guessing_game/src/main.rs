use std::io;

use rand::Rng;

use std::cmp::Ordering;

fn main(){
    println!("Guess the number!");
    loop{

        println!("Please input your guess.");

        let sec_num = rand::rng().random_range(1..=100);


        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        println!("The sec num is: {}", sec_num);

        match guess.cmp(&sec_num){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater=> println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}
