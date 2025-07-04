fn main(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_,y,_) = tup;
    println!("{}", y);
    println!("{}", tup.0);

    let a = [1, 2, 3, 4, 5];
    println!("{}",a[0]);
}

//in a tuple u can have different data types but in array, u can only have one data type
//array can only be of fixed length, no changes after decleration, 
//for array data is allocated in stack rather than heap
//VECTOR IS SIMILAR TO array but it can grow in size, if unsure, always use vector instead