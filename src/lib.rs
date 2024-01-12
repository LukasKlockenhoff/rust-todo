use std::fs::OpenOptions;
use std::io::{BufWriter, Write, BufReader, BufRead};
use std::{env, process};
use colored::*;

pub struct TodoList {
    pub todos: Vec<String>,
    pub todo_path: String,
}

impl TodoList {
    pub fn new() -> Self {

        let todo_path: String = match env::var("TODO_PATH") {
            Ok(t) => t,
            Err(_) => {
                eprintln!("TODO_PATH is not set");
                process::exit(1);
            }
        };

        let todofile = OpenOptions::new()
            .read(true) 
            .open(&todo_path);
        
        match todofile {
            Ok(file) => {
                let reader = BufReader::new(file);
                let todos = reader.lines().map(|l| l.unwrap()).collect();
                Self { todos, todo_path }
            },
            Err(_) => {
                eprintln!("Couldn't open the todofile");
                process::exit(1);
            }
        }

    }

    pub fn add(&mut self, todos: &[String]) {
        if todos.is_empty() {
            eprintln!("todo add takes at least 1 argument");
            process::exit(1);
        }

        let todofile = OpenOptions::new()
            .create(true) // a) create the file if it does not exist
            .append(true) // b) append a line to it
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");

        let mut buffer = BufWriter::new(todofile);

        for todo in todos {
            if todo.trim().is_empty() {
                continue;
            }

            // Appends a new task/s to the file with a new line character
            buffer
                .write_all(todo.as_bytes())
                .expect("unable to write data");
            buffer
                .write_all(b"\n")
                .expect("unable to write data");
        }
    }

    pub fn rm(&mut self, ids: &[i32]) {
        let mut todos = Vec::new();
        for (index, todo) in self.todos.iter().enumerate() {
            if !ids.contains(&(index as i32 + 1)) {
                todos.push(todo.clone());
            }
        }

        let _ = OpenOptions::new()
            .write(true) 
            .truncate(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");

        let todofile = OpenOptions::new()
            .create(true) // a) create the file if it does not exist
            .append(true) // b) append a line to it
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");
        
        let mut buffer = BufWriter::new(todofile);
        
        for todo in &todos {
            buffer
                .write_all(todo.as_bytes())
                .expect("unable to write data");
            buffer
                .write_all(b"\n")
                .expect("unable to write data");
        }
    }

    pub fn done(&mut self, ids: &[i32]) {
        for (index, todo) in self.todos.iter_mut().enumerate() {
            if ids.contains(&(index as i32 + 1)) {
                *todo = todo.strikethrough().to_string();
            }
        }

        let _ = OpenOptions::new()
            .write(true) 
            .truncate(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");

        let todofile = OpenOptions::new()
            .create(true) // a) create the file if it does not exist
            .append(true) // b) append a line to it
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");
        
        let mut buffer = BufWriter::new(todofile);
        
        for todo in &self.todos {
            buffer
                .write_all(todo.as_bytes())
                .expect("unable to write data");
            buffer
                .write_all(b"\n")
                .expect("unable to write data");
        }
    }

    pub fn undone(&mut self, ids: &[i32]) {
        for (index, todo) in self.todos.iter_mut().enumerate() {
            if ids.contains(&(index as i32 + 1)) {
                println!("contains");
                *todo = todo.replace("[9m", "").replace("[0m", "");
                println!("{}", todo);
            }
        }

        let _ = OpenOptions::new()
            .write(true) 
            .truncate(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");

        let todofile = OpenOptions::new()
            .create(true) // a) create the file if it does not exist
            .append(true) // b) append a line to it
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");
        
        let mut buffer = BufWriter::new(todofile);
        
        for todo in &self.todos {
            buffer
                .write_all(todo.as_bytes())
                .expect("unable to write data");
            buffer
                .write_all(b"\n")
                .expect("unable to write data");
        }
    }

    pub fn list(&self) {
        let todofile = OpenOptions::new()
            .read(true) 
            .open(&self.todo_path);
        
        match todofile {
            Ok(file) => {
                let reader = BufReader::new(file);
                for (index, line) in reader.lines().enumerate() {
                    let line = line.unwrap();
                    println!(r"{} {}", (index + 1).to_string().bold(), line);
                }
            },
            Err(_) => {
                eprintln!("Couldn't open the todofile");
                process::exit(1);
            }
        }
    }

    pub fn sort(&self) {
        let todofile = OpenOptions::new()
            .read(true) 
            .open(&self.todo_path);

        match todofile {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut todos : Vec<String> = reader.lines()
                .map(|l| l.expect("Could not parse line"))
                .collect();
                todos.sort_by(|a, b| b.cmp(a));
                let todofile = OpenOptions::new()
                    .write(true) 
                    .truncate(true)
                    .open(&self.todo_path);

                match todofile {
                    Ok(file) => {
                        let mut buffer = BufWriter::new(file);
                        for todo in todos {
                            buffer
                                .write_all(todo.as_bytes())
                                .expect("unable to write data");
                            buffer
                                .write_all(b"\n")
                                .expect("unable to write data");
                        }
                    },
                    Err(_) => {
                        eprintln!("Couldn't open the todofile");
                        process::exit(1);
                    }
                }
            },
            Err(_) => {
                eprintln!("Couldn't open the todofile");
                process::exit(1);
            }
        }
    }

    pub fn reset(&mut self) {
        let todofile = OpenOptions::new()
            .write(true) 
            .truncate(true)
            .open(&self.todo_path);
        
        match todofile {
            Ok(_) => {
                println!("{}", "Todo list reset".bold());
            },
            Err(_) => {
                eprintln!("Couldn't open the todofile");
                process::exit(1);
            }
        }
    }



}

pub fn help() {
    println!("Usage: todo <command> [options]");
    println!();
    println!("Commands:");
    println!("  list                  List all todos");
    println!("  add <title>           Add a new todo");
    println!("  rm <id>               Remove a todo");
    println!("  done <id>             Mark a todo as done");
    println!("  undone <id>           Mark a todo as undone");
    println!("  sort                  Sort todos by id");
    println!("  reset                 Reset the todo list");
    println!("  restore               Restore the todo list");
    println!("  help                  Show this help message");
}