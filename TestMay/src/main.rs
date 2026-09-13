use std::io;



fn maybe() -> u8 {
    println!("Maybe. Maybe... Don't today.");
    return 0
}

struct User {
    username: String,
    usermail: String,
    userpasswd: String,
    useractive: bool,
    
}


fn main() {
    println!("---------- Process is running ----------");
    let mut hellotext = String::from("Hello! Please write your username and passwd");
    let mut username_text = String::new();
    let mut passwd = String::new();
    println!("Username:");
    io::stdin()
        .read_line(&mut username_text)
        .expect("Plase print USER name");
    println!("Passwd:");
    io::stdin()
        .read_line(&mut passwd)
        .expect("This isn't passwd");
     let usersname = User {
        username: String::from(hellotext),
        userpasswd: String::from(passwd),
        usermail: String::from("somemail@examle.com"),
        useractive: true,
    };
    maybe();
}
