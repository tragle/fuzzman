extern crate reqwest;
extern crate rand;

use std::thread;
use std::sync::mpsc;
use rand::Rng;
use rand::distributions::Alphanumeric;

fn main() {
    println!("Starting fuzzman!");
    let (tx, rx) = mpsc::channel();
    let mut count = 0;
    thread::spawn(move || {
        for _ in rx {
            count += 1;
            print!("\r{}", count);
        }
    });
    for _ in 0..4 {
        let child_tx = tx.clone();
        thread::spawn(move || {
            let client = reqwest::Client::new();
            let mut rng = rand::thread_rng();
            for _i in 0..2500 {
                let str: String = rng
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .collect();
                let _res = client.post("http://localhost:3000")
                    .form(&[("q", str)])
                    .send();
                child_tx.send("done").unwrap();
            }
        }).join().unwrap();
    }
}
