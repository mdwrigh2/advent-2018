extern crate clap;

use clap::{App, Arg};
use std::error::Error;
use std::fs;

mod day_1;

fn main() -> Result<(), Box<dyn Error>> {
    // advent --day=1 <input>
    let matches = App::new("advent")
        .about("A CLI for advent of code")
        .version("0.1")
        .author("Michael Wright")
        .arg(Arg::with_name("day")
             .short("d")
             .long("day")
             .value_name("DAY")
             .help("Sets which days problem to solve")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("input")
             .help("The input file")
             .required(true)
             .index(1))
        .get_matches();

    let input = fs::read_to_string(matches.value_of("input").unwrap())?;

    let day: u32 = matches.value_of("day").unwrap().parse()?;

    let ans = match day {
        1 => day_1::solve(input),
        _ => panic!("Day is not implemented!"),
    };
    println!("{}", ans);
    Ok(())
}
