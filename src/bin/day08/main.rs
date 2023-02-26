use std::collections::HashMap;
use std::str;

use sscanf::scanf;

type Word = i32;

struct Regs {
    registers: HashMap<String, Word>,
}

impl Regs {
    fn new() -> Regs {
        Regs {
            registers: HashMap::new(),
        }
    }

    fn get(&self, name: &str) -> Word {
        *self.registers.get(name).unwrap_or(&0)
    }

    fn set(&mut self, name: &str, value: Word) {
        self.registers.insert(name.to_string(), value);
    }

    fn largest_value(&self) -> Word {
        *self.registers.values().max().unwrap_or(&0)
    }
}

fn compare(left: Word, comparison: &str, right: Word) -> bool {
    match comparison {
        ">" => left > right,
        "<" => left < right,
        ">=" => left >= right,
        "<=" => left <= right,
        "==" => left == right,
        "!=" => left != right,
        _ => {
            panic!("unknown comparison {comparison}");
        }
    }
}

fn compute(current: Word, operation: &str, amount: Word) -> Word {
    match operation {
        "inc" => current + amount,
        "dec" => current - amount,
        _ => {
            panic!("unknown operation {operation}");
        }
    }
}

fn part1(input: &str) -> Word {
    let mut registers = Regs::new();
    for line in input.split_terminator('\n') {
        if let Some((target_name, operation, amount, source, comparison, compare_with)) =
            scanf!(line, "{str} {str} {Word} if {str} {str} {Word}").ok()
        {
            let source_val = registers.get(source);
            if compare(source_val, comparison, compare_with) {
                let newval = compute(registers.get(target_name), operation, amount);
                registers.set(target_name, newval);
            }
        } else {
            panic!("failed to parse {line}");
        }
    }
    registers.largest_value()
}

#[test]
fn test_part1() {
    let input = concat!(
        "b inc 5 if a > 1\n",
        "a inc 1 if b < 5\n",
        "c dec -10 if a >= 1\n",
        "c inc -20 if c == 10\n",
    );
    assert_eq!(part1(input), 1);
}

fn main() {
    let input = str::from_utf8(include_bytes!("input.txt")).expect("input should be valid UTF-8");
    println!("Day 08 part 1: {}", part1(input));
}
