extern crate pnet;

use std::{result, env};
fn run_app() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            return Ok(());
        },
        _ => {
            return Err("error in arguments".to_string());
        }
    }
}



fn main() {
    ::std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
