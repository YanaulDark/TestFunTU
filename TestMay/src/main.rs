use std::io;
pub mod control_module;
pub mod frrr;
//pub mod async_module;

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
//    let _AcMeToFn = async_module::tokiofn();
    let may: u64 = frrr::fac(10).into();
//    println!("{:?}", _AcMeToFn);
    println!("{}", may);
    control_module::read_center();
}
