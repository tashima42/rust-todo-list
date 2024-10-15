use std::{fmt::Display, io::{self, Write}};

enum OperationErrors {
    NotFoundErr,
}

impl Display for OperationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OperationErrors::NotFoundErr => write!(f, "not found"),
        }
    }
}

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
    fn remove(&mut self, id: u64) -> Result<(), OperationErrors> {
        let mut index: Option<usize> = None;
        for (i, t) in self.todos.iter().enumerate() {
            if t.id == id {
                index = Some(i); 
            }
        } 

        match index {
            None => Err(OperationErrors::NotFoundErr),
            Some(index) => {
                self.todos.remove(index);
                Ok(())
            },
        }
    }
}

fn main() -> io::Result<()> {
    print!("============ TODO LIST =============\n");
    io::stdout().flush()?;


    let mut todos = Todos::new();

    loop {
     match menu()? {
            1 => {
                let todo = read_string("Type your todo: ").unwrap();
                todos.add(todo);
                wait()?;
            },
            2 => {
                for todo in todos.todos.iter() {
                    print!("[{}] - {}\n", todo.id, todo.text);
                }
                io::stdout().flush()?;
                wait()?;
            },
            3 => {
                let id = read_uint("Type the id of the todo you want to remove: ")?;
                match todos.remove(id) {
                    Ok(()) => println!("removed: {}", id),
                    Err(e) => println!("failed to remove: {}", e)
                };
            },
            4 => {
                println!("Update");
                todo!();
            },
            5 => {
                println!("Exiting...");
                break;
            },
            _ => {
                println!("Invalid option");
            }
        }
    }
    Ok(())
}

fn menu() -> io::Result<i32> {
    print!("1 - Add\n");
    print!("2 - List\n");
    print!("3 - Remove\n");
    print!("4 - Update\n");
    print!("5 - Exit\n");
    io::stdout().flush()?;

    read_int("Select an option: ")
}

fn read_uint(question: &str) -> io::Result<u64> {
    let int = read_int(question)?;
    Ok(int as u64)
}

fn read_int(question: &str) -> io::Result<i32> {
    let mut selected: i32 = 0;
    while selected == 0 {
        let buffer = read_string(question).expect("failed to read string");
        selected = match buffer.trim().parse() {
            Ok(int) => int,
            Err(error) => {
                print!("input is not an integer: {}", error);
                io::stdout().flush()?;
                0
            } 
        }
    }
    Ok(selected)
}

fn read_string(question: &str) -> io::Result<String> {
    print!("{}", question);
    io::stdout().flush()?;
    let mut buffer = String::new(); 
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("failed to read line");
    Ok(buffer.trim().to_owned())
}

fn wait() -> io::Result<()> {
    print!("press any key to continue...");
    io::stdout().flush()?;
    let mut buffer = String::new(); 
    io::stdin().read_line(&mut buffer).expect("failed to read");
    Ok(())
}
