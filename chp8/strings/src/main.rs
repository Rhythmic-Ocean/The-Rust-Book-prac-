fn main() {
    let s = "hello_world";
    let p = s.to_string();//converts str to String type


    let mut s = String::from("Hello");
    s.push_str(", world");

    println!("{}", s);

    let mut str = String::from("Hi, ");
    let s2 = String::from("Hello");
    str.push_str(&s2[..]);

    let mut s3 = String::from("Oh");
    let s4 = " god";
    s3.push_str(s4);

    println!("{}", s3);

    println!("{}", str);

    let s1 = String::from("Hell");
    let s2 = String::from(" no");
    let s3 = s1 + &s2; //s2 is borrowed while s1 is no longer in scope 

    //NOTE: the + operator is supposed to only accept a String + &str, however, it works even if we use &String, due to dref coercion which automatically
    //does it. i.e &String -> &str without having to do &s2[..]

    println!("{}", s3);

    let s9 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    let s7 = s9 + "-" + &s5 + "-" + &s6;

    println!("{}", s7);

    //OR you can use format! macro

    let s8 = format!("{}-{}-{}", s4, s5, s6);

    println!("{}", s8);


    //NOTE: not possible to index strings in Rust like:

    // let char1 = str[1];

    //OR

    //let char1 = &str[1];

    //cuz characters are stored as sequence of u8 numbers i.e each char as per utf-8 sequenceing rule is stored as a sequence of u8 bytes.
    //so if allowed to access each char like above it can only give u bytes, so it doesn't allow u at all/



    //3 ways to look at a string,bytes, scalar values and graphene clustures (look pg 140 for examples)
    //THAT'S WHY indexing in a string is a bad idea, u dunno if u will get a byte, scalar val or a grapheme clusture


    //Similarly string slices are dangerous too. Cuz if u do:

    //let strm = String::from("Здравствуйте");
    //let st = strm[0..1];

    //rust will panic, cuz 3 is stored in 2 bytes space, we need 0..2 at lest.

    //but for string "hello", one byte is good to access. cuz in UTF-8 encoding char uses 1-4 byte u8.

    //To iterate over strings, we have 3 ways, iterate over bytes, iterate over scalar values and iterate over grapheme clusture (need unicode-segmentation crate)

    //for s in strm.bytes() --> iterates over bytes
    //for s in strm.char() --> iterates over scalar values

}
