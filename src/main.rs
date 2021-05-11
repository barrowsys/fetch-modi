/*
 * --------------------
 * THIS FILE IS LICENSED UNDER THE FOLLOWING TERMS
 *
 * this code may not be used for any purpose. be gay, do crime
 *
 * THE FOLLOWING MESSAGE IS NOT A LICENSE
 *
 * <barrow@tilde.team> wrote this file.
 * by reading this text, you are reading "TRANS RIGHTS".
 * this file and the content within it is the gay agenda.
 * if we meet some day, and you think this stuff is worth it,
 * you can buy me a beer, tea, or something stronger.
 * -Ezra Barrow
 * --------------------
 */

use console::{style, Key, Term};
use dialoguer::{theme::ColorfulTheme, Confirmation, Input, Select};
use fetch_modi::{dave, john, karkat, rose, FetchResult};
use radix_fmt::radix;

fn select<'a>(prompt: &str, selections: &[&'a str]) -> Option<usize> {
    let term = Term::stdout();
    term.write_line(prompt).ok();
    let rv = Select::with_theme(&ColorfulTheme::default())
        .items(selections)
        .default(0)
        .clear(true)
        .interact_opt()
        .unwrap();
    if let Some(v) = rv {
        term.clear_last_lines(1).ok();
        writeln(prompt, selections[v]);
    }
    rv
}

fn selectd<'a>(prompt: &str, selections: &[&'a str], default: usize) -> Option<usize> {
    let term = Term::stdout();
    term.write_line(prompt).ok();
    let rv = Select::with_theme(&ColorfulTheme::default())
        .items(selections)
        .default(default)
        .clear(true)
        .interact_opt()
        .unwrap();
    if let Some(v) = rv {
        term.clear_last_lines(1).ok();
        writeln(prompt, selections[v]);
    }
    rv
}

fn confirm(prompt: &str) -> bool {
    Confirmation::new().with_text(prompt).interact().unwrap()
}

fn writer<T: std::fmt::Display>(data: T) {
    let line = format!("{}", style(data).red());
    Term::stdout().write_line(&line).ok();
}

fn writec<T: std::fmt::Display>(data: T) {
    let line = format!("{}", style(data).cyan());
    Term::stdout().write_line(&line).ok();
}

fn writeln(tag: &str, data: &str) {
    let line = format!("{}: {}", tag, style(data).green());
    Term::stdout().write_line(&line).ok();
}

fn get_string(prompt: &str) -> String {
    let r = Input::<String>::new()
        .with_prompt(prompt)
        .interact()
        .unwrap();
    Term::stdout().clear_last_lines(1).ok();
    writeln(prompt, &r);
    r
}

fn get_usize(prompt: &str) -> usize {
    let r = Input::<usize>::new()
        .with_prompt(prompt)
        .interact()
        .unwrap();
    Term::stdout().clear_last_lines(1).ok();
    writeln(prompt, &format!("{}", r));
    r
}

fn main() {
    let term = Term::stdout();
    println!("--- Log ---");
    let mut last_user = 0;
    'main: loop {
        println!();
        println!();
        println!("Welcome to the sylladex!");
        println!("You can press 'q' in a menu to return to its parent.");
        let ar = &["John", "Rose", "Dave", "Karkat"];
        let person = match selectd("Select User", ar, last_user) {
            None => break 'main,
            Some(p) => {
                last_user = p;
                term.clear_last_lines(5).ok();
                println!("Selected Person: {}", ar[p]);
                p
            }
        };
        match person {
            0 => john_root(),
            1 => rose_root(),
            2 => dave_root(),
            3 => karkat_root(),
            _ => (),
        }
    }
    println!("Exiting...");
}

fn john_root() {
    let size = get_usize("Size of john's sylladex?");
    let term = Term::stdout();
    let mut last_fm = 0;
    'person: loop {
        let modi = ["Queue", "Stack", "QueueStack", "Array", "Array QueueStack"];
        let modus = match selectd("New Fetch Modus", &modi, last_fm) {
            None => break 'person,
            Some(m) => {
                last_fm = m;
                m
            }
        };
        let mut qs = john::QueueStackModus::new(size);
        let mut ar = john::QueueStackModus::new_array(size);
        let mut arqs = john::ArrayQSModus::new(size);
        let mut last_c = 0;
        match modus {
            0 => 'modus: loop {
                term.write_line("").ok();
                term.write_line("").ok();
                writeln("Fetch Modus", modi[modus]);
                println!("{}", qs);
                let command = match selectd("Select Operation", &["Insert", "Take"], last_c) {
                    None => {
                        if confirm("Exit Modus? This will clear your sylldex!") {
                            break 'modus;
                        } else {
                            continue 'modus;
                        }
                    }
                    Some(n) => {
                        last_c = n;
                        n
                    }
                };
                term.clear_last_lines(5).ok();
                match command {
                    0 => writec(qs.queue_put(&get_string("Item"))),
                    1 => println!("{}", qs.queue_take()),
                    _ => {}
                }
            },
            1 => 'modus: loop {
                term.write_line("").ok();
                term.write_line("").ok();
                writeln("Fetch Modus", modi[modus]);
                println!("{}", qs);
                let command = match selectd("Select Operation", &["Insert", "Take"], last_c) {
                    None => {
                        if confirm("Exit Modus? This will clear your sylldex!") {
                            break 'modus;
                        } else {
                            continue 'modus;
                        }
                    }
                    Some(n) => {
                        last_c = n;
                        n
                    }
                };
                term.clear_last_lines(5).ok();
                match command {
                    0 => writec(qs.stack_put(&get_string("Item"))),
                    1 => println!("{}", qs.stack_take()),
                    _ => {}
                }
            },
            2 => 'modus: loop {
                term.write_line("").ok();
                term.write_line("").ok();
                writeln("Fetch Modus", modi[modus]);
                println!("{}", qs);
                let command = match selectd(
                    "Select Operation",
                    &["Insert", "Take Queue", "Take Stack"],
                    last_c,
                ) {
                    None => {
                        if confirm("Exit Modus? This will clear your sylldex!") {
                            break 'modus;
                        } else {
                            continue 'modus;
                        }
                    }
                    Some(n) => {
                        last_c = n;
                        n
                    }
                };
                term.clear_last_lines(5).ok();
                match command {
                    0 => writec(qs.stack_put(&get_string("Item"))),
                    1 => println!("{}", qs.queue_take()),
                    2 => println!("{}", qs.stack_take()),
                    _ => {}
                }
            },
            3 => 'modus: loop {
                term.write_line("").ok();
                term.write_line("").ok();
                writeln("Fetch Modus", modi[modus]);
                println!("{}", ar);
                let command = match selectd("Select Operation", &["Insert", "Take"], last_c) {
                    None => {
                        if confirm("Exit Modus? This will clear your sylldex!") {
                            break 'modus;
                        } else {
                            continue 'modus;
                        }
                    }
                    Some(n) => {
                        last_c = n;
                        n
                    }
                };
                term.clear_last_lines(5).ok();
                match command {
                    0 => writec(ar.array_put(get_usize("Index"), &get_string("Item"))),
                    1 => println!("{}", ar.array_take(get_usize("Index"))),
                    _ => {}
                }
            },
            4 => 'modus: loop {
                term.write_line("").ok();
                term.write_line("").ok();
                writeln("Fetch Modus", modi[modus]);
                println!("{}", arqs);
                let command = match selectd("Select Operation", &["Insert", "Take"], last_c) {
                    None => {
                        if confirm("Exit Modus? This will clear your sylldex!") {
                            break 'modus;
                        } else {
                            continue 'modus;
                        }
                    }
                    Some(n) => {
                        last_c = n;
                        n
                    }
                };
                term.clear_last_lines(5 + size).ok();
                match command {
                    0 => writec(arqs.put(
                        get_usize("Index"),
                        select("Mode", &["Queue", "Stack"]) == Some(0),
                        &get_string("Item"),
                    )),
                    1 => println!(
                        "{}",
                        arqs.get(
                            get_usize("Index"),
                            select("Mode", &["Queue", "Stack"]) == Some(0)
                        )
                    ),
                    _ => {}
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
        let mut last_c = 0;
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
            term.clear_last_lines(1).ok();
            last_command = command;
            match command {
                0 => 'modus: loop {
                    if auto_balance && rose.autobalance() {
                        writer("AUTOBALANCING");
                    }
                    term.write_line("").ok();
                    term.write_line("").ok();
                    writeln("Fetch Modus", "Tree");
                    println!("{}", rose);
                    let command =
                        match selectd("Select Operation", &["Insert", "Take", "Balance"], last_c) {
                            Some(m) => {
                                last_c = m;
                                m
                            }
                            None => break 'modus,
                        };
                    term.clear_last_lines(5 + rose.size()).ok();
                    match command {
                        0 => writec(rose.add(&get_string("Item"))),
                        1 => {
                            if take_root {
                                println!("{}", rose.take_root());
                                continue 'rootitem;
                            } else {
                                let ts = rose.takeables();
                                let s = select("Leaf", &ts);
                                if let Some(s) = s {
                                    let v = String::from(ts[s]);
                                    println!("{}", rose.take(&v))
                                }
                            }
                        }
                        2 => rose.balance(),
                        _ => (),
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
    let mut hash_function: &dyn Fn(&str) -> usize = &|s| dave::c2v1(s);
    let mut detect_collisions = false;
    'root: loop {
        let term = Term::stdout();
        let mut last_command = 0;
        let mut last_command_m = 0;
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
            term.clear_last_lines(1).ok();
            last_command = command;
            match command {
                0 => 'modus: loop {
                    term.write_line("").ok();
                    term.write_line("").ok();
                    writeln("Fetch Modus", "Hashmap");
                    println!("{}", dave);
                    let command =
                        match selectd("Select Operation", &["Insert", "Take"], last_command_m) {
                            Some(n) => {
                                last_command_m = n;
                                n
                            }
                            None => break 'modus,
                        };
                    term.clear_last_lines(16).ok();
                    match command {
                        0 => println!("{}", dave.add(&get_string("Item"))),
                        1 => println!("{}", dave.get(&get_string("Item"))),
                        _ => (),
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
                                term.clear_last_lines(1).ok();
                                hash_function = &|s| dave::c2v1(s);
                                continue 'root;
                            }
                            1 => {
                                term.clear_last_lines(1).ok();
                                hash_function = &|s| dave::c1v2(s);
                                continue 'root;
                            }
                            2 => {
                                term.clear_last_lines(1).ok();
                                hash_function = &|s| dave::scrabble(s);
                                continue 'root;
                            }
                            // "Scrabble" => hash_function = dave::scrabble,
                            _ => {
                                term.clear_last_lines(2).ok();
                                continue 'settings;
                            }
                        }
                    } else {
                        term.clear_last_lines(2).ok();
                        continue 'settings;
                    }
                }
                2 => detect_collisions = !detect_collisions,
                _ => {}
            }
        }
    }
}

fn karkat_root() {
    'rootitem: loop {
        let mut karkat_m = karkat::Modus::default();
        let term = Term::stdout();
        let mut last_command = 0;
        'person: loop {
            term.write_line("").ok();
            term.write_line("").ok();
            writeln("Fetch Modus", "Encryption");
            println!("{}", karkat_m);
            let command = match selectd(
                "Select Operation",
                &["Encrypt Item", "Decrypt Vault"],
                last_command,
            ) {
                None => {
                    term.clear_last_lines(5).ok();
                    if confirm("Exit Modus? This will clear your sylldex!") {
                        break 'rootitem;
                    } else {
                        continue 'person;
                    }
                }
                Some(m) => {
                    last_command = m;
                    m
                }
            };
            term.clear_last_lines(5).ok();
            match command {
                0 => writec(karkat_m.add(&get_string("Item"))),
                1 => {
                    if karkat_m.size() == 0 {
                        writer("No vaults!");
                        continue 'person;
                    }
                    let vault = get_usize("Vault");
                    if vault >= karkat_m.size() {
                        writer(format!(
                            "Cannot find vault {}, please input a number in the range 0-{}",
                            vault,
                            karkat_m.size() - 1
                        ));
                        continue 'person;
                    }
                    let mut current_roller = 0;
                    let mut roller_positions: [u16; 4] = [0, 0, 0, 0];
                    'decrypt: loop {
                        let t = karkat_m.test(vault, karkat::Modus::from_arr(roller_positions));
                        term.write_line("").ok();
                        term.write_line("Decryption Tool").ok();
                        term.write_line("").ok();
                        term.write_line("  ∧  ∧  ∧  ∧  ").ok();
                        for i in 0..4 {
                            let number = format!("{:#}", radix(roller_positions[i], 16));
                            let number = if t[i] {
                                style(number).green()
                            } else {
                                style(number)
                            };
                            if i == current_roller {
                                term.write_str(&format!(" >{}", number)).ok();
                            } else {
                                term.write_str(&format!("  {}", number)).ok();
                            }
                        }
                        term.write_line("").ok();
                        term.write_line("  ∨  ∨  ∨  ∨  ").ok();
                        let key = term.read_key().unwrap();
                        term.clear_last_lines(6).ok();
                        let val = karkat::Modus::from_arr(roller_positions);
                        match key {
                            Key::ArrowUp => {
                                roller_positions[current_roller] =
                                    (roller_positions[current_roller].wrapping_add(1)) % 16
                            }
                            Key::ArrowDown => {
                                roller_positions[current_roller] =
                                    (roller_positions[current_roller].wrapping_sub(1)) % 16
                            }
                            Key::ArrowLeft => current_roller = current_roller.wrapping_sub(1) % 4,
                            Key::ArrowRight => current_roller = current_roller.wrapping_add(1) % 4,
                            Key::Escape => break 'decrypt,
                            Key::Enter => {
                                if let FetchResult::Success(item) = karkat_m.take(vault, val) {
                                    println!("{}", FetchResult::Success(item));
                                    break 'decrypt;
                                }
                            }
                            Key::Char(c) => {
                                if c == 'q' {
                                    break 'decrypt;
                                }
                            }
                            _ => (),
                        };
                    }
                }
                _ => (),
            }
        }
    }
}
