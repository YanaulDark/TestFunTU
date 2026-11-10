use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn main() {
    let strings = [
        "==========", // 0: пустая строка
        "#=========", // 1: левый край
        "=#========", // 2
        "==#=======", // 3
        "===#======", // 4
        "====#=====", // 5
        "=====#====", // 6
        "======#===", // 7
        "=======#==", // 8
        "========#=", // 9
        "=========#", // 10: правый край
    ];

    let mut reward = 1;
    let delay = Duration::from_millis(200); 

    while reward <= 10 {
        // 1. Движение вниз по правому краю
        for i in 0..10 {
            for _ in 0..i {
                println!("{}", strings[0]);
            }
            println!("{}", strings[10]);
            for _ in 0..(9 - i) {
                println!("{}", strings[0]);
            }
            thread::sleep(delay);
            clear_screen();
        }

        // 2. Движение влево по нижнему краю (индексы от 10 до 1)
        for m in (1..=10).rev() {
            for _ in 0..9 {
                println!("{}", strings[0]);
            }
            println!("{}", strings[m]);
            thread::sleep(delay);
            clear_screen();
        }

        // 3. Движение вверх по левому краю
        for i in 0..10 {
            for _ in 0..(9 - i) {
                println!("{}", strings[0]);
            }
            println!("{}", strings[1]);
            for _ in 0..i {
                println!("{}", strings[0]);
            }
            thread::sleep(delay);
            clear_screen();
        }

        // 4. Движение вправо по верхнему краю (индексы от 1 до 10)
        for m in 1..=10 {
            println!("{}", strings[m]);
            for _ in 0..9 {
                println!("{}", strings[0]);
            }
            thread::sleep(delay);
            clear_screen();
        }

        reward += 1;
    }
}

