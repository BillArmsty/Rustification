use std::env;
// use std::fs;
//use std::process;
//use minigrep::Config;

fn main() {
    // Read any command line arguments passed to and then collect the values into a vector.
    let args: Vec<String> = env::args().collect();

    // Creating variables to hold the query argument and file path argument
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    // Call the run function and handle any errors that occur

    if
        let Err(e) = minigrep::run(minigrep::Config {
            query: args[0].clone(),
            file_path: args[0].clone(),
        })
    {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

// pub struct Config {
//     query: String,
//     file_path: String,
// }
