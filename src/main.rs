use std::env;
use std::process;
use cli::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem  parsing arguments: {}", err);
        eprintln!("The required arguments was not provided:\n <pattern> \n <path> \n USAGE: \n cli <pattern> <path>");
        process::exit(1);
    });

    if let Err(e) = cli::run(config) {
        eprintln!("Application Error:{}", e);
        process::exit(1);
    };
}
