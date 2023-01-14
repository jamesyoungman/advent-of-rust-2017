use std::str;

fn part1(s: &str) -> u32 {
    let mut v: Vec<char> = s.trim_end().chars().collect();
    match v.first() {
        Some(ch) => {
            v.push(*ch);
        }
        None => (),
    }
    v.windows(2)
        .filter_map(|w| {
            if w[0] == w[1] {
                match w[1].to_digit(10) {
                    Some(n) => Some(n),
                    None => {
                        panic!("{} is not a valid digit", w[1]);
                    }
                }
            } else {
                None
            }
        })
        .sum()
}

#[test]
fn test_day01_part1() {
    assert_eq!(part1(""), 0);
    assert_eq!(part1("\n"), 0);
    assert_eq!(part1("1122\n"), 3);
    assert_eq!(part1("1111\n"), 4);
    assert_eq!(part1("1234\n"), 0);
    assert_eq!(part1("91212129\n"), 9);
}

fn main() {
    let input = str::from_utf8(include_bytes!("input.txt")).expect("input should be value UTF-8");
    // 1155 is too low.
    println!("Day 01 part 1: {}", part1(input));
}
