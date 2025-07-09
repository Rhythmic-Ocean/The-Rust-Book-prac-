struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Color (i32, i32, i32); //struct tuple!!
fn main() {

    let email = String::from("hello@gmail.com");
    let username = String::from("me");
    let user1 = build_user(username, email);
    println!("{}", user1.sign_in_count);
}

let black = Color(0, 0, 0);
fn build_user(username: String, email: String) -> User{
    let user1 = User{
        email, //beacause email field and email parameter has the same name
        username, //cuz username fiend and parameter has same name
        active: true,
        sign_in_count: 4
    };
    
    user1
//CREATING INSTANCES FROM OTHER INSTANCES!!
    let user2 = User{
        email:String::from("hi@gmail.com"),
        username: String::from("hi"),
        ..user1 //active and sign_in_count from user1 is stored here!
    }
}
