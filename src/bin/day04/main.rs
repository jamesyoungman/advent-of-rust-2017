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

fn count_valid_passphrases(s: &str) -> usize {
    s.split_terminator('\n')
        .filter(|s| !contains_duplicate_word(s))
        .count()
}

#[test]
fn test_count_valid_passphrases() {
    assert_eq!(
        count_valid_passphrases(concat!(
            "aa bb cc dd ee\n",
            "aa bb cc dd aa\n",
            "aa bb cc dd aaa\n"
        )),
        2
    );
}

fn main() {
    let input = str::from_utf8(include_bytes!("input.txt")).expect("input should be value UTF-8");
    println!("Day 04 part 1: {}", count_valid_passphrases(input));
}
