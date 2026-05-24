mod utils;

use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use rand::Rng;
use utils::pretty_print;

fn main() {
    let stop_print = Arc::new(Mutex::new(false));
    loop {
        let _ = io::stdout().flush();
        let t = pretty_print::with_loading("Generating a magic number", &stop_print);
        let target = get_rand_int();
        thread::sleep(Duration::from_millis(1000));
        *stop_print.lock().unwrap() = true;
        t.join().unwrap();
        *stop_print.lock().unwrap() = false;

        let mut guess = get_guess();
        while guess != target {
            print_tip(target, guess);
            guess = get_guess();
        }

        println!("Bingo!");
    }
}

fn get_rand_int() -> i8 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..=100);
}

fn get_guess() -> i8 {
    println!("");
    print!("guess a number: ");
    let _ = io::stdout().flush();
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("read number failed");

    return line.trim().parse().expect("parse number error");
}

fn print_tip(target: i8, guess: i8) {
    let mut tip = "smaller";
    if target > guess {
        tip = "bigger";
    }
    println!("Think {}", tip);
    let _ = io::stdout().flush();
}
