//Hashmap is not included in the prelude we need to import it
use std::collections::HashMap;
fn main() {
    let mut scr = HashMap::new();
    scr.insert(String::from("Hello"), 3);
    //like vectors all key must have same type, all val must have same type.
    //data in hashmap is stored in heap
    //you can also store data in hash maps thru vectors like:
    let m = vec![String::from("Yellow"), String::from("White")];
    let k = vec![1, 2, 3];
    for (color, number) in m.iter().zip(k.iter()) {
        println!("{:?} => {:?}", color, number);
    }
    let h: HashMap<_,_> = m.iter().zip(k.iter()).collect();
    println!("{:?}", h);

    //entering referenced values into 

    let str = String::from("Hello");
    let str1 = String::from("Hi");
    let mut ha = HashMap::new();
    ha.insert(str, str1); //now str and str1 loses the ownership of the strings

    //selecting value from hash map

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 40);
    scores.insert(String::from("Yellow"), 30);

    let team_blu = String::from("Blue");

    let score = scores.get(&team_blu);

    println!("{:?}", score);

    //getting the value form hash map in iteration

    for (key, val) in scores{
        println!("{} : {}", key, val);
    }
    //updating hash maps
    
    //1. Updating thru overwriting
    let mut sc = HashMap::new();
    sc.insert(String::from("Hell"), 10);
    sc.insert(String::from("Hell"), 50);


    //2. Updating for a key only if there is no value



    let mut mc = HashMap::new();
    mc.insert(String::from("Hihi"), 50);
    
    mc.entry(String::from("Nah")).or_insert(4);  //checks if Nah key exists, if not makes it and gives it value 4
    mc.entry(String::from("Hihi")).or_insert(1); //checks if Hihi key exists, if not makes it and gives it value 1
    println!("{:?}", mc);


    // 3. Updating value based on old value

    let text = "hello world beautiful world";
    
    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0); //this one returns the value of the key value pair
        *count += 1;
    }
    
    println!("{:?}", map);



    //Hashing Function:
    //the in built one is secure but u can look for custom crate too if u want.

}
