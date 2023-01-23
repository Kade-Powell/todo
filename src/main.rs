use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

enum Command {
    Add(String),
    Close(usize),
    Swap(usize, usize),
    Exit,
    History,
    Invalid,
}

fn parse_input(input: &str) -> Command {
    let mut words = input.split_whitespace();
    match words.next() {
        Some("add") => {
            let item = words.collect::<Vec<_>>().join(" ");
            if item.is_empty() {
                Command::Invalid
            } else {
                Command::Add(item)
            }
        }
        Some("close") => match words.next() {
            Some(index) => match index.parse::<usize>() {
                Ok(index) => Command::Close(index),
                _ => Command::Invalid,
            },
            _ => Command::Invalid,
        },
        Some("swap") => match words.next() {
            Some(from_number) => match from_number.parse::<usize>() {
                Ok(from_number) => match words.next() {
                    Some(to_number) => match to_number.parse::<usize>() {
                        Ok(to_number) => Command::Swap(from_number, to_number),
                        _ => Command::Invalid,
                    },
                    _ => Command::Invalid,
                },
                _ => Command::Invalid,
            },
            _ => Command::Invalid,
        },
        Some("exit") => Command::Exit,
        Some("history") => Command::History,
        _ => Command::Invalid,
    }
}

fn read_todo_list() -> Vec<String> {
    let path = Path::new("todo_list.txt");
    if path.exists() {
        let mut file = OpenOptions::new()
            .read(true)
            .open(path)
            .expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        contents.lines().map(|x| x.to_string()).collect()
    } else {
        vec![]
    }
}

fn write_todo_list(todo_list: &[String]) {
    let path = Path::new("todo_list.txt");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .expect("Failed to open file");

    let contents: String = todo_list.join("\n");
    file.write_all(contents.as_bytes())
        .expect("Failed to write file");
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clear_screen();
    let mut todo_list = read_todo_list();
    let mut closed_list = Vec::new();
    loop {
        println!("*******************************");
        println!("📋 Todo list:");
        for (i, item) in todo_list.iter().enumerate() {
            println!("💣 {}. {}", i + 1, item);
        }
        println!("\n💬 Enter command (add, close, exit, swap, history): ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let command = parse_input(&input);
        match command {
            Command::Add(item) => {
                clear_screen();
                println!("➕Added item: {}", item);
                todo_list.push(item);
                write_todo_list(&todo_list);
            }
            Command::Swap(from_number, to_number) => {
                if (from_number > 0 && to_number > 0 ) && (from_number <= todo_list.len() && to_number <= todo_list.len()) {
                    todo_list.swap(from_number-1, to_number-1);
                    clear_screen();
                    println!("♻️Swapped {from_number} with {to_number}");
                    write_todo_list(&todo_list);
                } else {
                    clear_screen();
                    println!("❌Invalid index");
                }
            }
            Command::Close(index) => {
                if index > 0 && index <= todo_list.len() {
                    let item = todo_list.remove(index-1);
                    clear_screen();
                    println!("➖Closed item: {}", item);
                    closed_list.push(item);
                    
                    write_todo_list(&todo_list);
                } else {
                    clear_screen();
                    println!("❌Invalid index");
                }
            }
            Command::Exit => {
                clear_screen();
                break;
            }
            Command::History => {
                clear_screen();
                println!("🧾Completed items:");
                for (i, item) in closed_list.iter().enumerate() {
                    println!("✅{}. {}", i + 1, item);
                }
                println!("\n");
            }
            Command::Invalid => {
                clear_screen();
                println!("❌Invalid command")},
        }
    }
}
