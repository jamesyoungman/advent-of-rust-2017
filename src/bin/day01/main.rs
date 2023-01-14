use std::str;

fn part1(s: &str) -> u32 {
    let mut v: Vec<char> = s.trim_end().chars().collect();
    if let Some(ch) = v.first() {
        v.push(*ch);
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

fn part2(s: &str) -> u32 {
    let v: Vec<char> = s.trim_end().chars().collect();
    let len = v.len();
    let halflen = len / 2;
    (0..len)
        .filter_map(|i| {
            let next = (i + halflen) % len;
            if v[i] == v[next] {
                Some(
                    v[i].to_digit(10)
                        .expect("input should only contain decimal digits"),
                )
            } else {
                None
            }
        })
        .sum()
}

#[test]
fn test_day01_part2() {
    assert_eq!(part2(""), 0);
    assert_eq!(part2("\n"), 0);
    assert_eq!(part2("1212\n"), 6);
    assert_eq!(part2("1221\n"), 0);
    assert_eq!(part2("123425\n"), 4);
    assert_eq!(part2("123123\n"), 12);
    assert_eq!(part2("12131415\n"), 4);
}

fn main() {
    let input = str::from_utf8(include_bytes!("input.txt")).expect("input should be value UTF-8");
    println!("Day 01 part 1: {}", part1(input));
    println!("Day 01 part 2: {}", part2(input));
}
