use std::io;
fn main() {

    println!("Enter your string");
    let mut st = String::new();
    io::stdin().read_line(&mut st)
        .expect("Failed to read input");

    let mut new_st = String::new();
    let mut i = 0;
    let f = vec!['a', 'e', 'i' ,'o', 'u'];
    let mut first:char = 'p';
    let mut final_st = String::new();

    for c in st.chars(){
        if i == 0{
            first = c;
            if f.contains(&first){
                i+=2;
            }
            else{
                i+=1;
                continue;
            }
        }
        new_st.push(c);
    }
    if i == 1{
        final_st = format!("{}-{}ay", new_st.trim(), first);
    }
    else{
        final_st = format!("{}-hay",new_st.trim());
    }

    println!("{}", final_st);

}
