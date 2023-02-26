use std::cmp::max;
use std::collections::HashMap;
use std::str;

use sscanf::scanf;

type Word = i32;

struct Regs {
    registers: HashMap<String, Word>,
    largest_ever: Word,
}

impl Regs {
    fn new() -> Regs {
        Regs {
            registers: HashMap::new(),
            largest_ever: 0,
        }
    }

    fn get(&self, name: &str) -> Word {
        *self.registers.get(name).unwrap_or(&0)
    }

    fn set(&mut self, name: &str, value: Word) {
        self.largest_ever = max(self.largest_ever, value);
        self.registers.insert(name.to_string(), value);
    }

    fn largest_current_value(&self) -> Word {
        *self.registers.values().max().unwrap_or(&0)
    }

    fn largest_ever_value(&self) -> Word {
        self.largest_ever
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

fn parts(input: &str) -> (Word, Word) {
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
    (
        registers.largest_current_value(),
        registers.largest_ever_value(),
    )
}

#[test]
fn test_part2() {
    let input = concat!(
        "b inc 5 if a > 1\n",
        "a inc 1 if b < 5\n",
        "c dec -10 if a >= 1\n",
        "c inc -20 if c == 10\n",
    );
    assert_eq!(parts(input), (1, 10));
}

fn main() {
    let input = str::from_utf8(include_bytes!("input.txt")).expect("input should be valid UTF-8");
    let solution = parts(input);
    println!("Day 08 part 1: {}", solution.0);
    println!("Day 08 part 2: {}", solution.1);
}
