use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Check to make sure the user passed enough arguments
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // Create variables to hold the query argument and file path argument
        let query = args[1].clone();
        let file_path = args[2].clone();

        // Return a new instance of Config
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read the contents of the file specified by the second argument
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{}", contents);

    Ok(())
}

pub struct OtherConfig {
    pub query: String,
    pub file_path: String,
}

impl OtherConfig {
    pub fn build(args: &[String]) -> Result<OtherConfig, &'static str> {
        // Check to make sure the user passed enough arguments
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // Create variables to hold the query argument and file path argument
        let query = args[1].clone();
        let file_path = args[2].clone();

        // Return a new instance of Config
        Ok(OtherConfig { query, file_path })
    }
}