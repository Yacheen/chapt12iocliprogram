use std::fs;
use std::error::Error;
pub struct Config {
    pub query: String,
    pub file_name: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config,  &'static str>  {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query: String = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config {query, file_name})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>  {
    let contents = fs::read_to_string(config.file_name)?;

    Ok(())
}
pub fn search() {
    
}



//tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive"], search(query, contents)); 
    }
}