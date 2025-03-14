fn main() {
    struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64
    }

    let user1 = User {
        active: true, 
        username: String::from("Myapple"),
        email: String::from("blapple@gmail.com"),
        sign_in_count: 1
    };

    println!("{}", user1.username);
}
