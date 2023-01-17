use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn main() {
        println!("{}", a());
}


fn a() -> String {
    let data = include_bytes!("../input.txt");
    let (b, m) = data.split_at(data.windows(2) // Splits data into groups of 2
        .position(|b| b == b"\n\n").unwrap() + 2);
    let test = data.windows(2).map(|b| b.to_ascii_lowercase()).collect::<Vec<_>>();
    String::from("Hello")
}
