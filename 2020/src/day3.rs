use std::fs;


#[test]
fn test_ride_toboggan() {
    let input = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    let nb_trees = ride_toboggan(input, 0);
    assert_eq!(nb_trees, 7);
}


fn ride_toboggan(input: &str, start: usize) -> i64 {
    let mut nb_trees = 0;

    for (n, line) in input.lines().enumerate() {
        if line.chars().cycle().nth(3*n + start).unwrap() == '#' {
            // println!("Spotted # at {} {}", 3*n+start, line);
            nb_trees += 1 ;
        }
    }
    nb_trees
}

pub fn solve() {
    let nb_trees = ride_toboggan(
      fs::read_to_string("input/day3").unwrap().as_str(),
      0
    );
    println!("Number of trees encountered while riding toboggan = {}", nb_trees);
}

