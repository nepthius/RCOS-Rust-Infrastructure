struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let user1 = User {
        active: true, 
        username: String::from("Myapple"),
        email: String::from("blapple@gmail.com"),
        sign_in_count: 1
    };

    println!("{}", user1.username);
    let user2 = build_user(&user1.email, &user1.username);
    println!("{}", user2.username);
    println!("{}", user1.username);
}

fn build_user(email: &String, username: &String) -> User {
    User{
        active:true,
        username: String::from(username),
        email: String::from(email),
        sign_in_count: 1
    }
}