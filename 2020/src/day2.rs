use aoc2020::read_data;

// https://doc.rust-lang.org/stable/rust-by-example/custom_types/structs.html
#[derive(Debug)]
struct Password {
    count_min: usize,
    count_max: usize,
    alpha: char,
    word: String,
}


// https://doc.rust-lang.org/stable/rust-by-example/std/str.html
fn parse_line(line: &String) -> Password {
    let mut items: Vec<String> = Vec::new();

    for item in line.replace('-', " ").replace(':', " ").split_whitespace() {
        items.push(String::from(item));
    }

    let pass = Password {
        count_min: items[0].parse().unwrap(),
        count_max: items[1].parse().unwrap(),
        alpha: items[2].parse().unwrap(),
        word: items[3].parse().unwrap(),
    };
    pass
}

fn is_valid_part1(pass: &Password) -> bool {
    let count = pass.word.as_str().matches(pass.alpha).count();
    let valid = pass.count_min <= count && count <= pass.count_max;
    if valid {
      println!("{} occurs {:?} times in {:?}", pass.alpha, count, pass)
    }
    valid
}

#[test]
fn test() {
    let input = String::from("\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc");
    for line in input.lines() {
        let pass=parse_line(&String::from(line));
        is_valid_part1(&pass);
    }
}

// How to index a String in Rust
// https://stackoverflow.com/a/24542502
fn get_char(string: &String, idx: usize) -> char {
    string.to_string().chars().nth(idx - 1).unwrap()
}

#[test]
fn test_get_char() {
    assert_eq!(get_char(&String::from("Apple"), 1), 'A');
}

fn is_valid_part2(pass: &Password) -> bool {
    let valid = 
      (get_char(&pass.word, pass.count_min) == pass.alpha) ^
      (get_char(&pass.word, pass.count_max) == pass.alpha)
    ;
    if valid {
      println!("{:?} is valid", pass)
    }
    valid
}

#[test]
fn test2() {
    let input = String::from("\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc");
    for line in input.lines() {
        let pass=parse_line(&String::from(line));
        is_valid_part2(&pass);
    }
}

pub fn solve() {
    let passwords: Vec<Password> = read_data("input/day2").iter().map(parse_line).collect();

    let valids: Vec<bool> = passwords.iter().map(is_valid_part1).collect();
    let num_valid: i32 = valids.iter().map(|&b| b as i32).sum();
    println!("Number of valid passwords (part 1) = {}", num_valid);

    let valids2: Vec<bool> = passwords.iter().map(is_valid_part2).collect();
    let num_valid2: i32 = valids2.iter().map(|&b| b as i32).sum();
    println!("Number of valid passwords (part 2) = {}", num_valid2);
}
