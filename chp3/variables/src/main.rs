fn main(){
    let x = 5; //constants are always immutable while variables are immutable by default only
    println!("The value of x is: {}", x);
    let x = 6; //shadowing the first x
    println!("New x val: {}", x);
    let x = x*2; //shadowing the sec x
    println!("Newest x: {}", x);
    let x = "Sam";
    println!("Newest x2 val of x: {}", x);
}
//shadowing id diff from doing mut cuz, we get error if we assing to immutable var without let
//we can change type by showding too
//you can't change type for mutable variables