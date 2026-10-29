#[derive(Debug)]
struct User {
    username :String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn create(width: u32,height: u32) -> Self {
        Self {width,height}
    }
    fn area(&self) {
        println!("{}", self.width * self.height);
    }

}
pub fn run() {
    let mut user1 = User {
        username: String::from("abcd.com"),
        email: String::from("abcd123"),
        sign_in_count: 1,
        active: true,
    };
    // let user2 = user1;
    // println!("{}", user2);
    user1.email = String::from("a");
    println!("{:?}", user1);
    let user2 = build_user(String::from("`111"),String::from("222"));
    println!("{:?}", user2);

    let rect = Rectangle::create(20,20);
    println!("{:#?}", rect);
    rect.area();
    println!("{:#?}", rect);
}
fn build_user(username: String,email: String) -> User{
    User {
        username,
        email,
        sign_in_count: 333,
        active: true,
    }

}