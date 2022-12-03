extern crate core;

use std::fs::File;
use std::io::Read;
use crate::Choice::{Paper, Rock, Scissors};

#[derive(PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}
impl Choice {
    fn value(&self) -> i32 {
        return match self {
            Rock => {1}
            Paper => {2}
            Scissors => {3}
        }
    }
}

#[derive(PartialEq, Eq)]
enum Outcome {
    Win,
    Loose,
    Draw,
}

impl Outcome {
    fn value(&self) -> i32 {
        return match self {
            Outcome::Win => {6}
            Outcome::Draw => {3}
            Outcome::Loose => {0}
        }
    }
}

struct ChoiceOutcome {
    choice:Choice,
    outcome:Outcome
}

fn solve_part_1(input_string: &str) -> i32 {
    return input_string.split("\n")
        .map(|split| split.split(" ")
            .map(|m| match m {
                "A" | "X" => { Rock }
                "B" | "Y" => { Paper }
                "C" | "Z" => { Scissors }
                _ => { panic! {"Illegal char"} }
            }).collect()).map(|m: Vec<Choice>| {
        &m[1].value() +  check(&m[0], &m[1]).value()
    }).sum();
}

fn check(a: &Choice, b: &Choice) -> Outcome {
    if a == b { return Outcome::Draw; }
    if a == &Rock && b == &Paper { return Outcome::Win; }
    if a == &Scissors && b == &Rock { return Outcome::Win; }
    if a == &Paper && b == &Scissors { return Outcome::Win; }
    return Outcome::Loose;
}

fn solve_part_2(input_string: &str) -> i32 {
    return input_string.split("\n")
        .map(|split| {
            let split: Vec<_> = split.split(" ").collect();
              ChoiceOutcome {
                  choice:  match split[0] {
                      "A" => { Rock }
                      "B" => { Paper }
                      "C" => { Scissors }
                      _ => {
                          panic! {"Illegal char"}
                      }
                  },
                  outcome:  match split[1] {
                      "X" => { Outcome::Loose }
                      "Y" => { Outcome::Draw }
                      "Z" => { Outcome::Win }
                      _ => {
                          panic! {"Illegal char"}
                      }
                  },
              }
        }).map(|co: ChoiceOutcome| {
        match co.choice {
            Rock => { match co.outcome {
                Outcome::Win => {Paper}
                Outcome::Loose => {Scissors}
                Outcome::Draw => {Rock}
            } }
            Paper => { match co.outcome {
                Outcome::Win => {Scissors}
                Outcome::Loose => {Rock}
                Outcome::Draw => {Paper}
            }}
            Scissors => { match co.outcome {
                Outcome::Win => {Rock}
                Outcome::Loose => {Paper}
                Outcome::Draw => {Scissors}
            }}
        }.value() + co.outcome.value()
    }).sum();
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

    const TEST_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1() {
        assert_eq!(solve_part_1(TEST_INPUT), 15);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part_2(TEST_INPUT), 12);
    }
}
