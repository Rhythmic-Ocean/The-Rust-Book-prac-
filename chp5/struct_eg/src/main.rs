#[derive(Debug)]

struct rect (i32, i32);

fn main() {
    let rect1 = rect(4, 5);
    let area = find_area(&rect1);

    println!("Printing the width, length: {:?}", rect1); //user #[deriva[Debug]] to print stuff with Debug state
    println!("Area:{}", area);
}

fn find_area(rect1: &rect) -> i32{
    rect1.0 * rect1.1
}
//Primitive data types can be printed with display trait cuz there's only one way to print it.
//for non primitive types, there are multiple ways, so debug printing is the way
//Debug trait is to view stuff in debug more for other devs
//Display trait is for end user
//To print in debug trait use header #[derive(Debug)] and use {:?} or {:#?}
