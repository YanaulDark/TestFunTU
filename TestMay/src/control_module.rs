use std::io;

pub fn read_center() -> i32 {
    let mut read_command = String::new();
    
    io::stdin()
        .read_line(&mut read_command)
        .expect("Don't work.");

    if let Some((command, argument)) = read_command.split_once(' ') {
        let rd_command = &command;
        let _flags = &argument;
    }  else {
        // Если в строке нет пробела, считаем, что аргумент пустой
        match read_command.trim() {
            "help" => help(""),
            "atop" => atop(""),
            _ => 0,
        };
    }
    return 0;
}

pub fn atop(_arg: &str) -> i32 {
    println!("Atop is work");
    return 0;
}
pub fn help(_arg: &str) -> i32 {
    match _arg {
        "atop" => atop_doc(),
        _ => 0,
    };
    println!("Command list:\n
         help(command)\n
         atop(flags)\n");
    return 0;
}

pub fn atop_doc() -> i32 {
    println!("Animation top of progress");
    return 0;
}



