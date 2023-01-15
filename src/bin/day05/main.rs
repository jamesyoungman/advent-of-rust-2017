use std::str;

use lib::error::Fail;

#[derive(Eq, PartialEq, Debug)]
struct Code {
    pc: isize,
    instructions: Vec<i32>,
}

impl Code {
    fn step(&mut self, part: i32) -> bool {
        match self.instructions.get_mut(self.pc as usize) {
            Some(offset) => {
                self.pc += *offset as isize;
                match part {
                    1 => {
                        *offset += 1;
                    }
                    2 => {
                        if *offset >= 3 {
                            *offset -= 1;
                        } else {
                            *offset += 1;
                        }
                    }
                    _ => unreachable!(),
                }
                true
            }
            None => false,
        }
    }
}

impl TryFrom<&str> for Code {
    type Error = Fail;
    fn try_from(s: &str) -> Result<Self, Fail> {
        let instructions: Vec<i32> = s
            .split_terminator('\n')
            .map(|s| {
                s.trim()
                    .parse()
                    .map_err(|e| Fail(format!("invalid digit '{s}: {e}")))
            })
            .collect::<Result<Vec<i32>, Fail>>()?;
        Ok(Code {
            pc: 0,
            instructions,
        })
    }
}

#[test]
fn test_parse() {
    let input = "0\n3\n0\n  1\n  -3\n";
    assert_eq!(
        Code::try_from(input),
        Ok(Code {
            pc: 0,
            instructions: vec![0, 3, 0, 1, -3]
        })
    )
}

fn part1(s: &str) -> usize {
    let mut code = Code::try_from(s).expect("input should be valid");
    for i in 0.. {
        if !code.step(1) {
            return i;
        }
    }
    unreachable!()
}

fn part2(s: &str) -> usize {
    let mut code = Code::try_from(s).expect("input should be valid");
    for i in 0.. {
        if !code.step(2) {
            return i;
        }
    }
    unreachable!()
}

fn main() {
    let input = str::from_utf8(include_bytes!("input.txt")).expect("input should be valid UTF-8");
    println!("Day 05 part 1: {}", part1(input));
    println!("Day 05 part 2: {}", part2(input));
}
