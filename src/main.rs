use std::io::{self};

fn main() {
    println!("TODO LIST");
    println!("=========================");

    let mut length = 0;
    let list= &mut [const { String::new() }; 1024];

    loop {
     match menu() {
            1 => {
                println!("\n\n");
                add(list, length);
                length += 1;
                wait();
            },
            2 => {
                println!("\n\n");
                println!("List");
                for i in 0..length {
                    println!("[{}] - {}", i, list[i])
                }
                wait();
            },
            3 => {
                println!("Remove");
            },
            4 => {
                println!("Update");
            },
            _ => {
                println!("Invalid option");
            }
        }
    }
}

fn add(list: &mut [String; 1024], length: usize) {
    let todo = read_string("Type your todo").unwrap();
    list[length] = todo;
}

fn menu() -> i32 {
    println!("1 - Add");
    println!("2 - List");
    println!("3 - Remove");
    println!("4 - Update");

    read_int("Select an option: ")
}

fn read_int(question: &str) -> i32 {
    let mut selected: i32 = 0;
    while selected == 0 {
        let buffer = read_string(question).expect("failed to read string");
        selected = match buffer.trim().parse() {
            Ok(int) => int,
            Err(error) => {
                println!("input is not an integer: {}", error);
                0
            } 
        }
    }
    selected
}

fn read_string(question: &str) -> io::Result<String> {
    println!("{}", question);
    let mut buffer = String::new(); 
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("failed to read line");
    Ok(buffer.trim().to_owned())
}

fn wait() {
    println!("press any key to continue...");
    let mut buffer = String::new(); 
    io::stdin().read_line(&mut buffer).expect("failed to read");
}
