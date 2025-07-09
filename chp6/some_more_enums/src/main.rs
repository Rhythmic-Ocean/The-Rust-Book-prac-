enums Message{
    Quit,
    Move{x: i32, y: i32}, //like struct but u don't have the impl functionality
    Write{String},
    ChangeColor(i32, i32, i32)
}

//the above enums in equivalent struct, however the struct examples have the benifit of having their own methods.


struct QuitMessage;
struct MoveMessage{
    x: i32,
    y: i32
}
struct WriteMessage(String);
struct MoveMessage{
    x: i32,
    y: i32
}
struct ChangeColorMessage(i32, i32, i32);


//method for enums

impl Message{
    fn call(&self){
        //method defined here
    }
}

fn main() {

    let m = Message::Write(String::from("Hello"));

    m.call()

}
