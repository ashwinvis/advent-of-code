use std::env;
mod day1;
mod day2;
mod day3;


fn help() {
    println!("usage: aoc2020 <integer>

Show solution of day <integer>.");
}


fn main() {
    // Argument parsing
    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/arg/matching.html?highlight=argument#argument-parsing
    let args: Vec<String> = env::args().collect();

    println!("Hello, Advent of Code 2020!");

    match args.len() {
        // one argument passed
        2 => {
          println!("Solution for Day {}", args[1]);
          match args[1].parse() {
              Ok(1) => day1::solve(),
              Ok(2) => day2::solve(),
              Ok(3) => day3::solve(),
              _ => println!("error: solution for that day does not exist."),
          }
        },
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}


