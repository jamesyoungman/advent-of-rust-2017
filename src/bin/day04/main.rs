use std::collections::HashSet;
use std::str;

fn contains_duplicate_word(s: &str) -> bool {
    let mut words: HashSet<&str> = HashSet::new();
    for word in s.split_ascii_whitespace() {
        if !words.insert(word) {
            return true;
        }
    }
    false
}

#[test]
fn test_contains_duplicate_word() {
    assert!(!contains_duplicate_word("aa bb cc dd ee"));
    assert!(contains_duplicate_word("aa bb cc dd aa"));
    assert!(!contains_duplicate_word("aa bb cc dd aaa"));
}

fn count_part1_valid_passphrases(s: &str) -> usize {
    s.split_terminator('\n')
        .filter(|s| !contains_duplicate_word(s))
        .count()
}

#[test]
fn test_count_part1_valid_passphrases() {
    assert_eq!(
        count_part1_valid_passphrases(concat!(
            "aa bb cc dd ee\n",
            "aa bb cc dd aa\n",
            "aa bb cc dd aaa\n"
        )),
        2
    );
}

fn contains_duplicate_anagram(s: &str) -> bool {
    let mut words: HashSet<String> = HashSet::new();
    for word in s.split_ascii_whitespace() {
        let mut v: Vec<char> = word.chars().collect();
        v.sort();
        let canonical: String = v.iter().collect();

        if !words.insert(canonical) {
            return true;
        }
    }
    false
}

fn count_part2_valid_passphrases(s: &str) -> usize {
    s.split_terminator('\n')
        .filter(|s| !contains_duplicate_anagram(s))
        .count()
}

fn main() {
    let input = str::from_utf8(include_bytes!("input.txt")).expect("input should be value UTF-8");
    println!("Day 04 part 1: {}", count_part1_valid_passphrases(input));
    println!("Day 04 part 2: {}", count_part2_valid_passphrases(input));
}
