extern crate reqwest;
extern crate rand;
extern crate num_cpus;

use std::thread;
use std::sync::mpsc;
use std::env;
use rand::Rng;
use rand::distributions::Alphanumeric;

fn main() {
    println!("Starting fuzzman!");

    const WORD_SIZE: usize = 100;
    let threads: usize = num_cpus::get() - 1;

    let args: Vec<String> = env::args().collect();
    let default_arg = String::from("1000");
    let requests_arg = &args.get(1).unwrap_or(&default_arg);
    let requests = requests_arg.parse::<usize>().unwrap() / threads;

    let (tx, rx) = mpsc::channel();
    let mut count = 0;

    thread::spawn(move || {
        for _ in rx {
            count += 1;
            print!("\r{}", count);
        }
        println!("\n");
    });

    for _ in 0..threads {
        let child_tx = tx.clone();
        thread::spawn(move || {
            let client = reqwest::Client::new();
            let mut rng = rand::thread_rng();
            for _i in 0..requests {
                let str: String = rng
                    .sample_iter(&Alphanumeric)
                    .take(WORD_SIZE)
                    .collect();
                let _ = client.post("http://localhost:3000")
                    .form(&[("q", str)])
                    .send();
                child_tx.send(1).unwrap();
            }
        }).join().unwrap();
    }
    println!("\n");
}
