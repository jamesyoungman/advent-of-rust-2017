use std::{collections::HashSet, str};

fn part1(s: &str) -> &str {
    let mut programs: HashSet<&str> = HashSet::new();
    let mut supported_programs: HashSet<&str> = HashSet::new();
    for line in s.split_terminator('\n') {
        let left = match line.split_once(" -> ") {
            Some((left, right)) => {
                for prog in right.split(", ") {
                    supported_programs.insert(prog);
                }
                left
            }
            None => line,
        };
        match left.split_once(" (") {
            Some((program, _)) => {
                programs.insert(program);
            }
            None => {
                panic!("malformed line {line}");
            }
        }
    }
    for p in programs {
        if !supported_programs.contains(p) {
            return p;
        }
    }
    panic!("there is no root");
}

#[cfg(test)]
fn example() -> &'static str {
    concat!(
        "pbga (66)\n",
        "xhth (57)\n",
        "ebii (61)\n",
        "havc (66)\n",
        "ktlj (57)\n",
        "fwft (72) -> ktlj, cntj, xhth\n",
        "qoyq (66)\n",
        "padx (45) -> pbga, havc, qoyq\n",
        "tknk (41) -> ugml, padx, fwft\n",
        "jptl (61)\n",
        "ugml (68) -> gyxo, ebii, jptl\n",
        "gyxo (61)\n",
        "cntj (57)\n",
    )
}

#[test]
fn test_part1() {
    assert_eq!(part1(example()), "tknk");
}

fn main() {
    let input = str::from_utf8(include_bytes!("input.txt")).expect("input should be valid UTF-8");
    println!("Day 07 part 1: {}", part1(input));
}
