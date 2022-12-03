use std::fs::File;
use std::io::Read;

fn solve_part_1(input_string: &str) -> i32 {
    return input_string.lines().map(|line| {
        let (first, last) = line.split_at(line.len() / 2);
        let mut first_chars: Vec<_> = first.chars().collect();
        first_chars.retain(|x| last.chars().collect::<Vec<_>>().contains(x));
        priority(*first_chars.first().unwrap())
    }).sum();
}

fn solve_part_2(input_string: &str) -> i32 {
    return input_string.lines().collect::<Vec<_>>().chunks(3).map(|arr| {
        priority(*arr.iter().map(|v| {
            let x: Vec<_> = v.chars().collect();
            x
        }).reduce(|mut accum, item| {
            accum.retain(|x| item.contains(x));
            accum
        }).unwrap().first().unwrap())
    }).sum();
}

fn priority(c: char) -> i32 {
    (match c.is_lowercase() {
        true => { c as u8 - b'a' + 1 }
        false => { c as u8 - b'A' + 27 }
    }) as i32
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

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1() {
        assert_eq!(solve_part_1(TEST_INPUT), 157);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part_2(TEST_INPUT), 70);
    }
}
