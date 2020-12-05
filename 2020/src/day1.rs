use aoc2020::read_data;

// NOTE: & is required for the borrow checker
fn sum_to_2020(a: &i32, b: &i32) -> bool {
    a + b == 2020
}
fn trip_to_2020(a: &i32, b: &i32, c: &i32) -> bool {
    a + b + c == 2020
}

// Parse string to integer
// https://doc.rust-lang.org/stable/rust-by-example/conversion/string.html?highlight=string#parsing-a-string
fn str_to_int(string: &String) -> i32 {
    string.parse::<i32>().unwrap()
}


pub fn solve() { 
    // Initialize a Vector and read the input
    let xs: Vec<i32> = read_data("./input/day1").iter().map(str_to_int).collect();

    for (i, x) in xs.iter().enumerate() {
      for y in &xs[i..] {
        if sum_to_2020(&x, &y) {println!("{}", x * y)};

        for z in &xs[(i+1)..] {
          if trip_to_2020(&x, &y, &z) {println!("{}", x * y * z)};
        }
      }
    }
}
