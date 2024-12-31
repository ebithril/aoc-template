use easybench::bench;
use rayon::prelude::*;
use std::fs;

fn part1(input: &str) -> String {
    let lines = input.par_lines();

    let result = "".to_string();
    result
}

fn part2(input: &str) -> String {
    let lines = input.par_lines();

    let result = "".to_string();
    result
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part1: {}", part1(input));
    println!("Part1 bench:{}", bench(|| part1(input)));
    println!("Part2: {}", part2(input));
    println!("Part2 bench:{}", bench(|| part2(input)));
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn part1_test() {
        let input = include_str!("example.txt");
        assert_eq!("".to_string(), part1(input));
    }

    #[test]
    fn part2_test() {
        let input = include_str!("example.txt");
        assert_eq!("".to_string(), part2(input));
    }
}
