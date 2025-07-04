fn main() {
    let mut st = String::from("hello");

    let len = calc_length(&mut st);

    println!("{}",st);
}

fn calc_length(st_ref : &mut String){
    st_ref.push_str(", world");
}