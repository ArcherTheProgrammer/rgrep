use clap::Parser;
use std::{
    fs,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[clap(version = "1.0.1", author = "rgrep")]
struct Args {
    #[clap(default_value = "./")]
    file: String,

    #[clap(default_value = "none")]
    search: String,
}

fn main() {
    let args = Args::parse();
    let f = fs::File::open(args.file);

    match f {
        Ok(file) => {
            let reader = BufReader::new(file);
            for i in reader.lines() {
                match i {
                    Ok(value) => {
                        let single = value.split_whitespace();
                        for word in single {
                            if args.search == word {
                                println!("{}", &value);
                            }
                        }
                    }
                    Err(_e) => println!("errors occured"),
                }
            }
        }

        Err(_e) => {
            println!("Something went wrong");
        }
    }
}
