mod day1;

use std::env;
use anyhow::Result; 

fn main() -> Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cat inputs/<day>.txt | cargo run <day_number>");
        return Ok(());
    }
    let day_number: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid day number");
            return Ok(());
        }
    };

    match day_number {
        1 => day1::run(),
        _ => {
            println!("Day not implemented");
            Ok(())
        },
    }
}
