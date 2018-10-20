extern crate reqwest;
extern crate rand;

use rand::Rng;
use rand::distributions::Alphanumeric;

fn main() {
    println!("Starting fuzzman!");
    let client = reqwest::Client::new();
    let str: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(100)
        .collect();
    let _res = client.post("http://localhost:3000")
        .form(&[("q", str)])
        .send();
}
