
use std::collections::HashMap;

fn main() {
    struct User {
        username: String,
        email: String,
        active: bool,
        sign_in_count: u64
    }

    let user1 = User {
        username: String::from("iwi"),
        email: String::from("iwi@iwi.com"),
        active: false,
        sign_in_count: 1,
    };

    let user2 = User {
        username: String::from("owo"),
        email: String::from("owo@owo.com"),
        ..user1
    };

    println!("{}",user1.active);
    println!("{}",user2.active);

    let text = "hello world wonderful world"; 
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); 
        *count += 1;
        println!("{}",count);
    }
    println!("{:?}", map);
}
