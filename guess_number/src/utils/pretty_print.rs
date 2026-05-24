use std::thread;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn with_loading(msg: &str, stop_print: &Arc<Mutex<bool>>) -> thread::JoinHandle<()> {
    let msg = msg.to_string();
    let stop_clone = Arc::clone(stop_print);
    return thread::spawn(move || {
        loop {
            {
                let stop_flag = stop_clone.lock().unwrap();
                if *stop_flag {
                    break;
                }
            }

            print_loading(&msg, &stop_clone);
        }
    });
}

fn print_loading(msg: &str, stop_print: &Arc<Mutex<bool>>) {
    let chars = ["—", "\\", "|", "/"];
    for c in chars {
        {
            let stop_print = stop_print.lock().unwrap();
            if *stop_print {
                break;
            }
        }
        print!("\r{}\x1b[33m{}\x1b[0m", msg, c);
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(200));
    }
}
