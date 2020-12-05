// https://doc.rust-lang.org/stable/rust-by-example/custom_types/structs.html
#[derive(Debug)]
struct Password {
    count_min: usize,
    count_max: usize,
    alpha: char,
    word: String,
}

// https://doc.rust-lang.org/stable/rust-by-example/std/str.html
fn parse_line(line: String) -> bool {
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
    println!("{:?}", pass);
    let count = pass.word.as_str().matches(pass.alpha).count();
    println!("{} occurs {:?} times", pass.alpha, count);
    pass.count_min < count && count < pass.count_max
}

#[test]
fn test() {
    let input = String::from("1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc");
    for line in input.lines() {
        parse_line(String::from(line));
    }
}

pub fn solve() {
    // test();
}
