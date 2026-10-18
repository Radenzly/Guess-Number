#![allow(unused_imports)] //Can be override with allow, warn, deny, forbid
#![allow(dead_code)]
use rand::Rng;
use std::io;
use std::io::{Write, stdout};
use std::process;

struct Gamestate {
    total_win: i32,
    retry_limit: i32,
}

impl Gamestate {
    fn exit() {
        process::exit(0);
    }
    fn add_total(&mut self, amount: i32) {
        self.total_win += amount;
    }
}

fn main() {
    //let num: Vec<i32> = (1..=100).collect();
    let mut rng = rand::rng();
    let rand = rng.random_range(1..=100);
    let mut input = String::new();

    let mut state = Gamestate {
        total_win: 0,
        retry_limit: 6,
    };

    loop {
        input.clear();

        print!("Enter Guess Number(1-100): ");
        stdout().flush().expect("Error at flushing");

        io::stdin().read_line(&mut input).expect("Error at stdin");

        let sanitize = input.trim();

        if sanitize == "exit" {
            break;
        }

        let parse_str = match sanitize.parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Error at parse_str");
                return;
            }
        };

        if rand == parse_str {
            println!("you win! Guess({})", rand);
            state.add_total(1);
            break;
        }

        if parse_str > rand {
            println!("Note: Too High!");
        }

        if parse_str < rand {
            println!("Note: Too Low!");
        }
    }
}
