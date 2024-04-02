use std::env;
use std::process;

mod data_processing;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a CSV file as an argument.");
        process::exit(1);
    }

    let filename = &args[1];
    match data_processing::ingest_data(filename) {
        Ok(data) => {
            let processed_data = data_processing::process_data(&data);
            println!("{:?}", processed_data);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            process::exit(1);
        }
    }
}