use std::env;
use rust_todo::TodoList;
use rust_todo::help;

fn main() {
    let mut todo_list = TodoList::new();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "list" => todo_list.list(),
            "add" => {
                let todos = &args[2..];
                todo_list.add(todos);
                todo_list.list();
            },
            "done" => {
                let ids: Vec<i32> = args[2..].iter().map(|id| id.parse().unwrap()).collect();
                todo_list.done(&ids);
                todo_list.list();
            },
            "undone" => {
                let ids: Vec<i32> = args[2..].iter().map(|id| id.parse().unwrap()).collect();
                todo_list.undone(&ids);
                todo_list.list();
            },
            "reset" => {
                todo_list.reset();
            },
            "sort" => {
                todo_list.sort();
                todo_list.list();
            },
            "rm" => {
                let ids: Vec<i32> = args[2..].iter().map(|id| id.parse().unwrap()).collect();
                todo_list.rm(&ids);
                todo_list.list();
            },
            "help" | "--help" | "-h" | _ => help(),
        }
    } else {
        todo_list.list();
    }
}