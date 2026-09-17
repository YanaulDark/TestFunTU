use std::io;
pub mod frrr;

fn maybe() -> u8 {
    println!("Maybe. Maybe... Don't today.");
    return 0;
}

struct User {
    username: String,
    usermail: String,
    userpasswd: String,
    useractive: bool,
}

fn main() {
    println!("---------- Process is running ----------");
    let hellotext = String::from("Hello! Please write your username and passwd");
    println!("{}", hellotext );

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
    
    let _usersname = User {
        username: String::from(hellotext),
        userpasswd: String::from(passwd),
        usermail: String::from("somemail@examle.com"),
        useractive: true,
    };
    maybe();
    frrr::frrr();
    let may: u64 = frrr::fac(10).into();
    println!("{}", may)
}
