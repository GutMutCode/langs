use std::env;
// use std::error::Error;
// use std::fs;
use std::process;

use minigrep::Config;

// fn parse_config(args: &[String]) -> (&str, &str) {
// fn parse_config(args: &[String]) -> Config {
//     // let query = &args[1];
//     // let file_path = &args[2];
//     let query = args[1].clone();
//     let file_path = args[2].clone();
//
//     Config { query, file_path }
// }

fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // let query = &args[1];
    // let file_path = &args[2];
    // let (query, file_path) = parse_config(&args);
    // let config = parse_config(&args);
    // let config = Config::new(&args);
    // let config = Config::build(&args).unwrap_or_else(|err| {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    //
    // println!("With text:\n{contents}");
    // run(config);
    // if let Err(e) = run(config) {
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// struct Config {
//     query: String,
//     file_path: String,
// }
//
// impl Config {
//     // fn new(args: &[String]) -> Config {
//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             // panic!("not enough arguments");
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone();
//         let file_path = args[2].clone();
//         Ok(Config { query, file_path })
//     }
// }
//
// // fn run(config: Config) {
// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     // let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
//     let contents = fs::read_to_string(config.file_path)?;
//
//     println!("With text:\n{contents}");
//     Ok(())
// }
