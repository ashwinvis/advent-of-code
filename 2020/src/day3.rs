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
    let nb_trees = ride_toboggan(input, 0, 3, 1);
    assert_eq!(nb_trees, 7);
    show_nb_trees(input);
}


fn ride_toboggan(input: &str, start: usize, right: usize, down: usize) -> i64 {
    let mut nb_trees = 0;

    for (n, line) in input.lines().enumerate() {
        if (n % down) == 0 {
            // NOTE: Hack! Only handles 1 and 2
            let loc = right * n / down + start;
            if line.chars().cycle().nth(loc).unwrap() == '#' {
                // println!("Line {} at loc {}: {}", n, loc, line);
                nb_trees += 1 ;
            }
        }
    }
    nb_trees
}

fn show_nb_trees(input: &str) {
    let mut prod_trees = 1;
    for (right, down) in vec![(1,1), (3, 1), (5, 1), (7, 1), (1, 2)] {
      let nb_trees = ride_toboggan(
          input,
          0,
          right,
          down,
      );
      println!(
          "Number of trees encountered while riding toboggan, right {}, down {} = {}",
          right,
          down,
          nb_trees
      );
      prod_trees *= nb_trees;
    }
    println!("Product of nb_trees = {}", prod_trees);
}

pub fn solve() {
    show_nb_trees(fs::read_to_string("input/day3").unwrap().as_str());
}

