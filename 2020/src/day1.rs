use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// NOTE: & is required for the borrow checker
fn sum_to_2020(a: &i32, b: &i32) -> bool {
    a + b == 2020
}
fn trip_to_2020(a: &i32, b: &i32, c: &i32) -> bool {
    a + b + c == 2020
}

// Parse string to integer
// https://doc.rust-lang.org/stable/rust-by-example/conversion/string.html?highlight=string#parsing-a-string
fn str_to_int(string: &str) -> i32 {
    string.parse::<i32>().unwrap()
}


pub fn solve() { 
    // Initialize a resizable Vector to read the input
    let mut xs: Vec<i32> = [].to_vec();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day1") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                xs.push(str_to_int(&ip));
            }
        }
    }

    for (i, x) in xs.iter().enumerate() {
      for y in &xs[i..] {
        if sum_to_2020(&x, &y) {println!("{}", x * y)};

        for z in &xs[(i+1)..] {
          if trip_to_2020(&x, &y, &z) {println!("{}", x * y * z)};
        }
      }
    }
}
