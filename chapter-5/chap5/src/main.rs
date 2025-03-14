struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Rectangle {
    width:u32,
    height:u32
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width*self.height
    }
    fn square(size: u32) -> Self {
        Self {
            width:size,
            height:size,
        }
    }
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



    let user3  = User{
        username: String::from("FlameON!"),
        ..user1
    };
    println!("{}", user3.username);
    println!("{}", user3.email);
    //user1's username/email does not work anymore
    println!("{}", user1.active);

    struct flame(i32, i32,i32);
    let flash = flame(2,3,4);

    let rect1 = Rectangle{
        width:10,
        height:10
    };
    println!("{}", rect1.area());

    let nr = Rectangle::square(10);
    println!("{}", nr.width)


}

fn build_user(email: &String, username: &String) -> User {
    User{
        active:true,
        username: String::from(username),
        email: String::from(email),
        sign_in_count: 1
    }
}