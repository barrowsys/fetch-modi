use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirmation, Input, Select};
use fetch_modi::{dave, john, rose, FetchResult, InsertResult};
use std::thread;
use std::time::Duration;

fn select<'a>(prompt: &str, selections: &[&'a str]) -> Option<usize> {
    let term = Term::stdout();
    term.write_line(prompt);
    let rv = Select::with_theme(&ColorfulTheme::default())
        .items(selections)
        .default(0)
        .clear(true)
        .interact_opt()
        .unwrap();
    if rv.is_some() {
        term.clear_last_lines(1);
        writeln(prompt, selections[rv.unwrap()]);
    }
    rv
}

fn selectd<'a>(prompt: &str, selections: &[&'a str], default: usize) -> Option<usize> {
    let term = Term::stdout();
    term.write_line(prompt);
    let rv = Select::with_theme(&ColorfulTheme::default())
        .items(selections)
        .default(default)
        .clear(true)
        .interact_opt()
        .unwrap();
    if rv.is_some() {
        term.clear_last_lines(1);
        writeln(prompt, selections[rv.unwrap()]);
    }
    rv
}

fn confirm(prompt: &str) -> bool {
    Confirmation::new().with_text(prompt).interact().unwrap()
}

fn writer<T: std::fmt::Display>(data: T) {
    let line = format!("{}", style(data).red());
    Term::stdout().write_line(&line);
}

fn writec<T: std::fmt::Display>(data: T) {
    let line = format!("{}", style(data).cyan());
    Term::stdout().write_line(&line);
}

fn writeln(tag: &str, data: &str) {
    let line = format!("{}: {}", tag, style(data).green());
    Term::stdout().write_line(&line);
}

fn get_string(prompt: &str) -> String {
    let r = Input::<String>::new()
        .with_prompt(prompt)
        .interact()
        .unwrap();
    Term::stdout().clear_last_lines(1);
    writeln(prompt, &r);
    r
}

fn get_usize(prompt: &str) -> usize {
    let r = Input::<usize>::new()
        .with_prompt(prompt)
        .interact()
        .unwrap();
    Term::stdout().clear_last_lines(1);
    writeln(prompt, &format!("{}", r));
    r
}

fn wait() {
    thread::sleep(Duration::from_millis(300));
}

// Memory
// Scrabble hash function
// Inverted CV hash function
fn main() {
    println!("Welcome to the sylladex!");
    println!("You can press 'q' in a menu to return to its parent.");
    'main: loop {
        wait();
        let person = select("Select User", &["John", "Rose", "Dave"]);
        match person {
            Some(0) => john_root(),
            Some(1) => rose_root(),
            Some(2) => dave_root(),
            _ => break 'main,
        }
    }
    println!("Exiting...");
}

fn john_root() {
    let size = get_usize("Size of john's sylladex?");
    let term = Term::stdout();
    'person: loop {
        let modi = ["Queue", "Stack", "QueueStack", "Array", "Array QueueStack"];
        let modus = match select("New Fetch Modus", &modi) {
            None => break 'person,
            Some(m) => m,
        };
        let mut qs = john::QueueStackModus::new(size);
        let mut ar = john::QueueStackModus::new_array(size);
        let mut arqs = john::ArrayQSModus::new(size);
        match modus {
            0 => 'modus: loop {
                term.write_line("");
                term.write_line("");
                writeln("Fetch Modus", modi[modus]);
                println!("{}", qs);
                let command = select("Select Operation", &["Insert", "Take"]);
                term.clear_last_lines(5);
                match command {
                    Some(0) => writec(qs.queue_put(&get_string("Item"))),
                    Some(1) => println!("{}", qs.queue_take()),
                    _ => {
                        if confirm("Exit Modus? This will clear your sylldex!") {
                            break 'modus;
                        }
                    }
                }
            },
            1 => 'modus: loop {
                term.write_line("");
                term.write_line("");
                writeln("Fetch Modus", modi[modus]);
                println!("{}", qs);
                let command = select("Select Operation", &["Insert", "Take"]);
                term.clear_last_lines(5);
                match command {
                    Some(0) => writec(qs.stack_put(&get_string("Item"))),
                    Some(1) => println!("{}", qs.stack_take()),
                    _ => {
                        if confirm("Exit Modus? This will clear your sylldex!") {
                            break 'modus;
                        }
                    }
                }
            },
            2 => 'modus: loop {
                term.write_line("");
                term.write_line("");
                writeln("Fetch Modus", modi[modus]);
                println!("{}", qs);
                let command = select("Select Operation", &["Insert", "Take Queue", "Take Stack"]);
                term.clear_last_lines(5);
                match command {
                    Some(0) => writec(qs.stack_put(&get_string("Item"))),
                    Some(1) => println!("{}", qs.queue_take()),
                    Some(2) => println!("{}", qs.stack_take()),
                    _ => {
                        if confirm("Exit Modus? This will clear your sylldex!") {
                            break 'modus;
                        }
                    }
                }
            },
            3 => 'modus: loop {
                term.write_line("");
                term.write_line("");
                writeln("Fetch Modus", modi[modus]);
                println!("{}", ar);
                let command = select("Select Operation", &["Insert", "Take"]);
                term.clear_last_lines(5);
                match command {
                    Some(0) => writec(ar.array_put(get_usize("Index"), &get_string("Item"))),
                    Some(1) => println!("{}", ar.array_take(get_usize("Index"))),
                    _ => {
                        if confirm("Exit Modus? This will clear your sylldex!") {
                            break 'modus;
                        }
                    }
                }
            },
            4 => 'modus: loop {
                term.write_line("");
                term.write_line("");
                writeln("Fetch Modus", modi[modus]);
                println!("{}", arqs);
                let command = select("Select Operation", &["Insert", "Take"]);
                term.clear_last_lines(5 + size);
                match command {
                    Some(0) => writec(arqs.put(
                        get_usize("Index"),
                        select("Mode", &["Queue", "Stack"]) == Some(0),
                        &get_string("Item"),
                    )),
                    Some(1) => println!(
                        "{}",
                        arqs.get(
                            get_usize("Index"),
                            select("Mode", &["Queue", "Stack"]) == Some(0)
                        )
                    ),
                    _ => {
                        if confirm("Exit Modus? This will clear your sylldex!") {
                            break 'modus;
                        }
                    }
                }
            },
            _ => break 'person,
        };
    }
}

fn rose_root() {
    'rootitem: loop {
        let first_item = get_string("Root Item");
        let term = Term::stdout();
        let mut auto_balance = false;
        let mut take_root = false;
        let mut rose = rose::Modus::new(&first_item);
        let mut last_command = 0;
        'person: loop {
            let settings = [
                "Use Modus",
                if auto_balance {
                    "Auto Balance: Enabled"
                } else {
                    "Auto Balance: Disabled"
                },
                if take_root {
                    "Take Mode: Root"
                } else {
                    "Take Mode: Leaf"
                },
            ];
            let command = match selectd("Settings", &settings, last_command) {
                None => {
                    if confirm("Exit Modus? This will clear your sylldex!") {
                        break 'rootitem;
                    } else {
                        continue 'person;
                    }
                }
                Some(m) => m,
            };
            term.clear_last_lines(1);
            last_command = command;
            match command {
                0 => 'modus: loop {
                    if auto_balance && rose.autobalance() {
                        writer("AUTOBALANCING");
                    }
                    term.write_line("");
                    term.write_line("");
                    writeln("Fetch Modus", "Tree");
                    println!("{}", rose);
                    let command = select(
                        "Select Operation",
                        if auto_balance {
                            &["Insert", "Take"]
                        } else {
                            &["Insert", "Take", "Balance"]
                        },
                    );
                    term.clear_last_lines(5 + rose.size());
                    match command {
                        Some(0) => writec(rose.add(&get_string("Item"))),
                        Some(1) => match take_root {
                            false => {
                                let ts = rose.takeables();
                                let s = select("Leaf", &ts);
                                if s.is_some() {
                                    let v = String::from(ts[s.unwrap()]);
                                    println!("{}", rose.take(&v))
                                }
                            }
                            true => {
                                println!("{}", rose.take_root());
                                continue 'rootitem;
                            }
                        },
                        Some(2) => rose.balance(),
                        _ => break 'modus,
                    };
                },
                1 => auto_balance = !auto_balance,
                2 => take_root = !take_root,
                _ => (),
            };
        }
    }
}

fn dave_root() {
    let mut hash_function: &dyn Fn(&str) -> usize = &|s| dave::C2V1(s);
    let mut detect_collisions = false;
    'root: loop {
        let term = Term::stdout();
        let mut last_command = 0;
        let mut dave = dave::Modus::new(hash_function);
        'settings: loop {
            dave.detect_collisions = detect_collisions;
            let settings = [
                "Use Modus",
                "Set Hash Function",
                if dave.detect_collisions {
                    "Detect Collisions: On"
                } else {
                    "Detect Collisions: Off"
                },
            ];
            let command = match selectd("Settings", &settings, last_command) {
                None => {
                    if confirm("Exit Modus? This will clear your sylldex!") {
                        return;
                    } else {
                        continue 'settings;
                    }
                }
                Some(m) => m,
            };
            term.clear_last_lines(1);
            last_command = command;
            match command {
                0 => 'modus: loop {
                    term.write_line("");
                    term.write_line("");
                    writeln("Fetch Modus", "Hashmap");
                    println!("{}", dave);
                    let command = select("Select Operation", &["Insert", "Take"]);
                    term.clear_last_lines(16);
                    match command {
                        Some(0) => println!("{}", dave.add(&get_string("Item"))),
                        Some(1) => println!("{}", dave.get(&get_string("Item"))),
                        _ => break 'modus,
                    }
                },
                1 => {
                    let selection = match select("Hash Function", &["C2V1", "C1V2", "Scrabble"]) {
                        None => continue 'settings,
                        Some(m) => m,
                    };
                    if confirm("Change hash function? This will clear your sylldex!") {
                        match selection {
                            0 => {
                                term.clear_last_lines(1);
                                hash_function = &|s| dave::C2V1(s);
                                continue 'root;
                            }
                            1 => {
                                term.clear_last_lines(1);
                                hash_function = &|s| dave::C1V2(s);
                                continue 'root;
                            }
                            2 => {
                                term.clear_last_lines(1);
                                hash_function = &|s| dave::Scrabble(s);
                                continue 'root;
                            }
                            // "Scrabble" => hash_function = dave::scrabble,
                            _ => {
                                term.clear_last_lines(2);
                                continue 'settings;
                            }
                        }
                    } else {
                        term.clear_last_lines(2);
                        continue 'settings;
                    }
                }
                2 => detect_collisions = !detect_collisions,
                _ => {}
            }
        }
    }
}
// fn dave_root() {
//     let mut dave = dave::Modus::new(dave::hash_default);
//     loop {
//         wait();
//         println!();
//         println!("Fetch Modus: Hashmap");
//         println!("{}", dave);
//         match select("Command", &["insert", "take", "toggle collision detection"]) {
//             Some(0) => {
//                 let new_item = get_string("Item");
//                 match dave.add(&new_item) {
//                     InsertResult::SuccessBut(items) => {
//                         println!("Inserted into slot {}", (dave.hash_function)(&new_item));
//                         for item in items.iter() {
//                             println!("The \"{}\" fell out of your sylladex", item);
//                         }
//                     }
//                     InsertResult::Success => {
//                         println!("Inserted into slot {}", (dave.hash_function)(&new_item))
//                     }
//                     InsertResult::CollisionDetected => println!(
//                         "Collision detected in slot {}",
//                         (dave.hash_function)(&new_item)
//                     ),
//                 }
//             }
//             Some(1) => {
//                 let key = get_string("Item to get");
//                 match dave.get(&key) {
//                     FetchResult::Success(s) => println!("Successfully removed {} from hashmap", s),
//                     FetchResult::Empty => {
//                         println!("No item matching key {}", (dave.hash_function)(&key))
//                     }
//                     _ => println!("Failed to remove"),
//                 }
//             }
//             Some(2) => dave.toggle_collisions(),
//             _ => return,
//         }
//     }
// }
