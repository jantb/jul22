use std::fs::File;
use std::io::Read;

fn solve_part_1(input_string: &str) -> i32 {
    return input_string
        .split("\n\n")
        .map(|split| split.lines().map(|line| line.parse::<i32>().unwrap()).sum())
        .max()
        .unwrap();
}

fn solve_part_2(input_string: &str) -> i32 {
    let split = input_string.split("\n\n");
    let mut map: Vec<i32> = split
        .map(|split| split.lines().map(|line| line.parse::<i32>().unwrap()).sum())
        .collect();
    map.sort();
    return map.iter().rev().take(3).sum();
}

fn main() {
    let mut input_string = String::new();

    File::open("src/input.txt")
        .unwrap()
        .read_to_string(&mut input_string)
        .unwrap();

    println!("Part 1 {}", solve_part_1(&input_string));
    println!("Part 2 {}", solve_part_2(&input_string));
}

#[cfg(test)]
mod tests {
    use crate::{solve_part_1, solve_part_2};

    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1() {
        assert_eq!(solve_part_1(TEST_INPUT), 24_000);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part_2(TEST_INPUT), 45_000);
    }
}
