struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
fn main() {

    let email = String::from("hello@gmail.com");
    let username = String::from("me");
    let user1 = build_user(username, email);
    println!("{}", user1.sign_in_count);
}

fn build_user(username: String, email: String) -> User{
    let user1 = User{
        email, //beacause email field and email parameter has the same name
        username, //cuz username fiend and parameter has same name
        active: true,
        sign_in_count: 4
    };
    
    user1
}
