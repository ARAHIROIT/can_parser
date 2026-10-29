use crate::traits;

trait Fruits {
    fn price(&self) -> u32;
}

struct Apple;
impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}

struct Banana;
impl Fruits for Banana {
    fn price(&self) -> u32 {
        5
    }
}
trait  Summary {
    fn summarize(&self) -> String {
        String::from("00000")
    }
}
trait Message {
    fn message(&self) -> String{
        String::from("Message")
    }
}
struct NewArticle {
    headloine: String,
    location: String,
    author: String,
    content: String,
}
impl Summary for NewArticle{
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})",self.headloine, self.author, self.location)
    // }
}
impl Message for NewArticle{

}

struct Tweet {
    username: String,
    content: String,
    replay: bool,
    retweet: bool,
}
impl Summary for Tweet{
    fn summarize(&self) -> String{ 
        format!("{}: {}",self.username, self.content)
    }
}

pub fn run() {
    let apple = Apple{};
    let banana = Banana{};
    get_price(apple);
    get_price(banana);
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("ssssssssss"),
        replay: false,
        retweet:false,
    };
    println!("{}", tweet.summarize());
    let article = NewArticle {
        headloine: String::from("abcde"),
        location: String::from("fgh"),
        author: String::from("ijkl"),
        content: String::from("mn"),
    };
    println!("{}", article.summarize());
    notify(&article);
    notify_another(&article);
}
fn get_price<T: Fruits>(fruits: T) {
    println!("{}", fruits.price());
}
fn notify(item: &impl Summary) {
    println!("12345 {}",item.summarize());
}
fn notify_another(item: &(impl Summary + Message)) {
    println!("{}", item.summarize());
    println!("{}", item.message())
}