use colored::*;
use std::{
    collections::HashMap,
    io::{self, Write},
    thread,
    time::Duration,
};

#[derive(Debug, Clone)]
struct Node {
    id: u32,
    label: String,
    properties: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct Relationship {
    id: u32,
    label: String,
    start_node: u32,
    end_node: u32,
    properties: HashMap<String, String>,
}

#[derive(Debug)]
struct GraphDatabase {
    nodes: HashMap<u32, Node>,
    relationships: HashMap<u32, Relationship>,
}

impl GraphDatabase {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            relationships: HashMap::new(),
        }
    }

    fn add_node(&mut self, id: u32, label: String, properties: HashMap<String, String>) {
        let node = Node {
            id,
            label,
            properties,
        };
        self.nodes.insert(id, node);
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}

fn main() {
    println!("{}", "Welcome to Rust Graph".green().bold());
    thread::sleep(Duration::from_secs(3));
    clear_screen();
    let mut db = GraphDatabase::new();
    loop {}
}
