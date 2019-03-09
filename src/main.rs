use std::io;

use rsh;
use rsh::Args;

fn main() {
    println!("Hello, rsh!");

    loop {
        if let Err(e) = rsh::prompt() {
            eprintln!("{}", e);
        };

        let mut input = String::new();

        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Got an error when trying to read from stdin: {}", e);
            continue;
        };

        let args = Args::new(&input);

        if let Err(e) = args.deal() {
            eprintln!("{}", e);
        };
    }
}
