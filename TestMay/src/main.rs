use std::io;



fn maybe() -> u8 {
    println!("Maybe. Maybe... Don't today.");
    return 0
}



fn main() {
    println!("---------- Process is running ----------");
    let mut later = String::new();
    io::stdin()
        .read_line(&mut later)
        .expect("Please write code word");
    let later = later.trim();
    maybe();
}
