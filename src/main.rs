use std::{io, process::exit};

// needs overflow guard
fn fibonacci(mut current: i32, mut previous: i32, mut limit: i32) {
    println!("{current}");
    current = current + previous;
    previous = current - previous; 
    limit -= 1;
    if limit > 0 {
        fibonacci(current, previous, limit);
    }
}

fn main() {
    println!("input sequence length: ");
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(_) => {},
        Err(_) => {
            println!("error reading input");
            exit(1);
        },
    }

    let limit: i32 = buf.trim().parse().unwrap_or(10);
    fibonacci(1, 0, limit);
}
