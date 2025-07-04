fn main(){
    let guess: u32 = "42".parse()
        .expect("Not a num!");
    println!("{}",guess);

    let y: f32 = 3.0;
    println!("{}", y);
}
//rust is strictly typed, in most cases compiler knows what the type is but in cases where the type can be a little ambigious like in above eg, make sure the type is properly annotated.
//i8 u8 i16 u16 i32 u32 i64 u64 isize usize