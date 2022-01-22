extern crate walkdir;
extern crate zip;


mod core;
mod ui;


use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args)
}
