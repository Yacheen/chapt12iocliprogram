use std::env;
use std::process;
use chapt12iocliprogram::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = chapt12iocliprogram::run(config) {
        println!("Problem getting file: {}", e);
        process::exit(1);
    };
    
}
