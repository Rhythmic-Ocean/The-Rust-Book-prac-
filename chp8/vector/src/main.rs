//rust has data structures called collection. The data here point to a heap, unlike in array or tuple. Thus the amount does not
//need to be known at compile time, can grow and shrink
//Vector:
//Same as arrays, except data is stored in heap so u can add/ drop from it any time.

//eg:




fn main() {

    let v: Vec<i32> = Vec::new();
    //OR
    let mut m = vec![1, 2, 3];
    //rust compiler can auto infer it's i32, but if u wanna annotatite the vector anyway use first method
    //else use vec! macro

    //adding to vector: we need vector to be mutable

    m.extend([1, 2, 3]);

    let mut n = vec![1, 2, 3];

    n.push(5);

    //vec is freed once it goes out of scope

    //reading in vector

    let third: &i32 = &n[2]; //since all elements of vector are stored in heap as ref, we need &

    let thrid: Option<&i32> = n.get(2);

    //use first option if u wanna crash when accessing invalid memory
    //use sec option if u wanna manually handle None case
    //no need for type annotation, it's done auto.


    //NOTE: You can't read and mutate a vector's value at the same time
    //cuz, if u mutate a vector, the vec might go to a diff memory place due to size change

    //Iterating over a vector and chaning mutable values:

    let vc = vec![1, 2, 3, 4];

    for i in &vc{
        println!("{}", *i);
    }

    let mut vcc = vec![1, 2, 3, 4];

    for i in &mut vcc{
        *i += 1; //we need to defreference here to mutate the value
        println!("{}", i);
    }

    
}


//using enum to store multiple types in vector:

enum Spreadsheet{
    Int(i32),
    Float(f64),
    Text(String),
}

let multi_type_vec = vec![
    Spreadsheet::Int(3),
    Spreadsheet::Float(3.5),
    Spreadsheet::Text("Me"),
]


