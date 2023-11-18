use std::fs;

fn part1(input_path: &str) -> String {
    let content = fs::read_to_string(input_path).expect("Expected input.txt");
    let lines = content.lines();

    let result = "".to_string();
    result
}

fn part2(input_path: &str) -> String {
    let content = fs::read_to_string(input_path).expect("Expected input.txt");
    let lines = content.lines();

    let result = "".to_string();
    result
}

fn main() {
    println!("Part1: {}", part1("input.txt"));
    println!("Part2: {}", part2("input.txt"));
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn part1_test() {
        assert_eq!("".to_string(), part1("example.txt"));
    }

    #[test]
    fn part2_test() {
        assert_eq!("".to_string(), part2("example.txt"));
    }
}
