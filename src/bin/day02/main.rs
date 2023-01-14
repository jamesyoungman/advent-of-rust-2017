use std::cmp::{max, min};
use std::str;

fn rowdiff(s: &str) -> u32 {
    let (dmin, dmax) = s
        .split_ascii_whitespace()
        .map(|s: &str| s.parse().expect("decimal numbers only please"))
        .fold((u32::MAX, u32::MIN), |(dmin, dmax), n| {
            (min(dmin, n), max(dmax, n))
        });
    dmax - dmin
}

fn part1(s: &str) -> u32 {
    s.split_terminator('\n').map(rowdiff).sum()
}

#[test]
fn test_day02_rowdiff() {
    assert_eq!(rowdiff("5 1 9 5"), 8);
    assert_eq!(rowdiff("7 5 3"), 4);
    assert_eq!(rowdiff("2 4 6 8"), 6);

    assert_eq!(rowdiff("200       600"), 400);
}

#[test]
fn test_day02_part1() {
    assert_eq!(part1("5 1 9 5\n7 5 3\n2 4 6 8\n"), 18);
}

fn maybe_rowdiv(s: &str) -> Option<u32> {
    let mut numbers: Vec<u32> = s
        .split_ascii_whitespace()
        .map(|s: &str| s.parse().expect("decimal numbers only please"))
        .collect();
    numbers.sort();
    for (i, greater) in numbers.iter().enumerate() {
        for lesser in numbers.iter().take(i) {
            if greater % lesser == 0 {
                return Some(greater / lesser);
            }
        }
    }
    None
}

fn rowdiv(s: &str) -> u32 {
    maybe_rowdiv(s).expect("every row must have a divisible pair")
}

#[test]
fn test_day02_rowdiv() {
    assert_eq!(rowdiv("5 9 2 8"), 4);
    assert_eq!(rowdiv("9 4 7 3"), 3);
    assert_eq!(rowdiv("3 8 6 5"), 2);
}

fn part2(s: &str) -> u32 {
    s.split_terminator('\n').map(rowdiv).sum()
}

#[test]
fn test_day02_part2() {
    assert_eq!(part2(concat!("5 9 2 8\n", "9 4 7 3\n", "3 8 6 5\n")), 9);
}

fn main() {
    let input = str::from_utf8(include_bytes!("input.txt")).expect("input should be value UTF-8");
    println!("Day 01 part 1: {}", part1(input));
    println!("Day 01 part 2: {}", part2(input));
}
