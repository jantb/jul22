use std::fs::File;
use std::io::Read;
use std::ops::{Range, RangeInclusive};

fn solve_part_1(input_string: &str) -> i32 {
    return input_string.lines().filter(|line| {
        let split: Vec<_> = line.split(",").collect();

        let right_included_in_left = get_included_in(&split[1], &split[0]);
        let left_included_in_right = get_included_in(&split[0], &split[1]);

        left_included_in_right || right_included_in_left
    }).count() as i32;
}

fn get_included_in(this: &str, other: &str) -> bool {
    let range_this = get_range(this);
    let range_other = get_range(other);

    range_this.fold(true, |acc, i| {
        acc && range_other.contains(&i)
    })
}

fn overlaps(this: &str, other: &str) -> bool {
    let range_this = get_range(this);
    let range_other = get_range(other);

    range_this.fold(false, |acc, i| {
        acc || range_other.contains(&i)
    })
}

fn get_range(part: &str) -> RangeInclusive<i32> {
    let vec = part.split("-").collect::<Vec<_>>();

    let first = vec[0].parse::<i32>().unwrap();
    let last = vec[1].parse::<i32>().unwrap();

    return first..=last;
}

fn solve_part_2(input_string: &str) -> i32 {
    return input_string.lines().filter(|line| {
        let split: Vec<_> = line.split(",").collect();

        let right_included_in_left = overlaps(&split[1], &split[0]);
        let left_included_in_right = overlaps(&split[0], &split[1]);

        left_included_in_right || right_included_in_left
    }).count() as i32;
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

    const TEST_INPUT: &str = "44-67,43-43
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1() {
        assert_eq!(solve_part_1(TEST_INPUT), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part_2(TEST_INPUT), 4);
    }
}
