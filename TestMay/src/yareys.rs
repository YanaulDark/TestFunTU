use std::io;
use std::collections::HashMap;

#[tokio::main]
pub async fn main() {
    let mut username = String::new();

    io::stdin()
        .read_line(&mut username)
        .expect("Username, please.");
    let _res = hello_user(&username).await;
    println!("Ok. {}, so {}.", username, username);
}

pub async fn hello_user(username: &String) -> u8 {
    println!("You are {}?", username);
    return 0;
}
