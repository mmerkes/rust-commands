extern crate clap;

use clap::{App, Arg, ArgMatches};
use std::fs;
use std::path::Path;

fn main() {
    let args = App::new("rcat")
        .version("1.0")
        .author("Matt Merkes")
        .about("concatenate files and print on the standard output")
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("number all output lines"),
        )
        .arg(
            Arg::with_name("FILE")
                .help("Sets the file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    std::process::exit(match cat(args) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}

fn cat(args: ArgMatches) -> Result<(), String> {
    let file_path = args.value_of("FILE").unwrap();

    if !Path::new(file_path).exists() {
        return Err(format!("{}: No such file or directory", file_path));
    }

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    if args.is_present("number") {
        for (i, line) in contents.split('\n').enumerate() {
            if i > 0 {
                print!("\n");
            }
            print!("{:>6}\t{}", i + 1, line);
        }
    } else {
        print!("{}", contents);
    }

    Ok(())
}
