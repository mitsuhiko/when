mod cli;
mod location;
mod parser;

fn main() {
    match cli::execute() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    }
}
