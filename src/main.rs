use std::io::{self, Write};

fn main() {
    loop {
        print!("Continue (yes/no)? ");
        let _ = io::stdout().flush();

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Could not read from stdin :(");

        match buffer.trim() {
            "yes" => break,
            "no" => std::process::exit(1),
            _ => {
                println!("Please type 'yes' or 'no'.");
            },
        };
    }

    return;
}
