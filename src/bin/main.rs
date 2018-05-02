extern crate brainrot;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please provide one argument, that is the bf source file.");
        return;
    }

    let filename = &args[1];

    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => {
            println!("Could not open file {}", filename);
            return;
        }
    };

    let mut code = Vec::new();
    match file.read_to_end(&mut code) {
        Ok(_) => (),
        Err(_) => {
            println!("Failed to read file {}", filename);
            return;
        }
    }

    match brainrot::run(code, &mut std::io::stdin(), &mut std::io::stdout()) {
        Ok(_) => (),
        Err(bferror) => {
            println!("Invalid code: {}", bferror.description());
            return;
        }
    }
}
