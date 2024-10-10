use std::io::{self, Write};

struct Todos {
    last_id: u64,
    todos: Vec<Todo>,
}

struct Todo {
    id: u64,
    text: String,
}

impl Todos {
    fn new() -> Self {
        Self {
            last_id: 0,
            todos: vec![],
        }
    }
    fn add(&mut self, text: String) {
        self.last_id += 1;
        self.todos.push(Todo{ id: self.last_id, text });
    }
}

fn main() {
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout);

    writeln!(writer, "TODO LIST").unwrap();
    writeln!(writer, "=========================").unwrap();
    writer.flush().unwrap();


    let mut todos = Todos::new();

    loop {
     match menu() {
            1 => {
                writeln!(writer, "\n\n").unwrap();
                let todo = read_string("Type your todo").unwrap();
                todos.add(todo);
                wait();
            },
            2 => {
                writeln!(writer, "\n\n").unwrap();
                for todo in todos.todos.iter() {
                    writeln!(writer, "[{}] - {}", todo.id, todo.text).unwrap();
                }
                writer.flush().unwrap();
                wait();
            },
            3 => {
                println!("Remove");
            },
            4 => {
                println!("Update");
            },
            5 => {
                writeln!(writer, "Exiting...").unwrap();
                writer.flush().unwrap();
                break;
            },
            _ => {
                println!("Invalid option");
            }
        }
    }
}

fn menu() -> i32 {
    println!("1 - Add");
    println!("2 - List");
    println!("3 - Remove");
    println!("4 - Update");
    println!("5 - Exit");

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
