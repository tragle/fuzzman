extern crate reqwest;

fn main() {
    println!("Starting fuzzman!");
    let client = reqwest::Client::new();
    let _res = client.post("http://localhost:3000")
        .form(&[("q", "hello")])
        .send();
}
