use dialoguer::{theme::ColorfulTheme, Input, Select};
use fetch_modi::{dave, john, rose, FetchResult, InsertResult};
use std::thread;
use std::time::Duration;

fn select<'a>(prompt: &str, selections: &[&'a str]) -> Option<usize> {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(selections)
        .interact_opt()
        .unwrap()
}

fn get_string(prompt: &str) -> String {
    Input::<String>::new()
        .with_prompt(prompt)
        .interact()
        .unwrap()
}

fn get_usize(prompt: &str) -> usize {
    Input::<usize>::new()
        .with_prompt(prompt)
        .interact()
        .unwrap()
}

fn wait() {
    thread::sleep(Duration::from_millis(300));
}

fn main() {
    println!("Welcome to the sylladex");
    'person: loop {
        wait();
        let person = select("Select User", &["John", "Rose", "Dave"]);
        match person {
            Some(0) => john_root(),
            Some(1) => rose_root(),
            Some(2) => dave_root(),
            _ => break 'person,
        }
    }
    println!("Exiting...");
}

fn john_root() {
    let size = get_usize("Size of john's sylladex?");
    let mut john = john::Modus::new(size);
    let mut modus = "Queue";
    loop {
        wait();
        println!();
        println!("Fetch Modus: {}", modus);
        println!("{}", john);
        match select("Command", &["stack", "queue", "take", "insert"]) {
            Some(0) => modus = "Stack",
            Some(1) => modus = "Queue",
            Some(2) => {
                let result = match modus {
                    "Stack" => john.stack_take(),
                    "Queue" => john.queue_take(),
                    _ => panic!(),
                };
                match result {
                    FetchResult::Empty => println!("nothing to take"),
                    FetchResult::Success(item) => println!("Removed {} from sylladex", item),
                    FetchResult::SuccessBut(_, _) => panic!("this shouldn't happen"),
                };
            }
            Some(3) => {
                let new_item = get_string("Item to insert");
                let result = match modus {
                    "Stack" => john.stack_put(&new_item),
                    "Queue" => john.queue_put(&new_item),
                    _ => panic!(),
                };
                match result {
                    InsertResult::Success => println!("Inserted Successfully"),
                    InsertResult::SuccessBut(items) => {
                        println!("Inserted Successfully");
                        for item in items.iter() {
                            println!("The \"{}\" fell out of your sylladex", item);
                        }
                    }
                    InsertResult::CollisionDetected => {}
                }
            }
            _ => return,
        }
    }
}

fn rose_root() {
    let first_item = get_string("Root Item");
    let mut rose = rose::Modus::new(&first_item);
    loop {
        wait();
        println!();
        println!("Fetch Modus: Tree");
        println!("{}", rose);
        match select("Command", &["insert", "take"]) {
            Some(0) => {
                let new_item = get_string("Item to insert");
                rose.add(&new_item);
            }
            Some(1) => {
                let takeables = rose.takeables();
                let mut takenable = String::new();
                if let Some(e) = select("Available Items:", &takeables) {
                    takenable = String::from(takeables[e]);
                }
                match rose.take(&takenable) {
                    FetchResult::Success(s) => println!("Successfully removed {} from tree", s),
                    _ => println!("Failed to remove"),
                }
            }
            _ => return,
        }
    }
}

fn dave_root() {
    let mut dave = dave::Modus::new(dave::hash_default);
    loop {
        wait();
        println!();
        println!("Fetch Modus: Tree");
        println!("{}", dave);
        match select("Command", &["insert", "take"]) {
            Some(0) => {
                let new_item = get_string("Item to insert");
                match dave.add(&new_item) {
                    InsertResult::SuccessBut(items) => {
                        println!("Inserted into slot {}", (dave.hash_function)(&new_item));
                        for item in items.iter() {
                            println!("The \"{}\" fell out of your sylladex", item);
                        }
                    }
                    InsertResult::Success => {
                        println!("Inserted into slot {}", (dave.hash_function)(&new_item))
                    }
                    _ => {}
                }
            }
            Some(1) => {
                let key = get_string("Item to get");
                match dave.get(&key) {
                    FetchResult::Success(s) => println!("Successfully removed {} from hashmap", s),
                    _ => println!("Failed to remove"),
                }
            }
            _ => return,
        }
    }
}
