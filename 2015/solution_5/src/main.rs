use std::fs;

mod helpers;
mod line_checker;
mod rules;

use rules::has_pairs::HasPairsRule;
use rules::repeat_with_gap::RepeatWithGapRule;

fn main() {
    let mut line_checker = line_checker::LineChecker::new();
    line_checker.add_rule(HasPairsRule::new(1));
    line_checker.add_rule(RepeatWithGapRule::new(1));

    let text = match fs::read_to_string("input.txt") {
        Ok(text) => text,
        Err(e) => panic!("Error reading file: {e}"),
    };

    let mut num_good_strings_modified = 0;
    for line in text.lines() {
        if line_checker.check(&String::from(line)) {
            num_good_strings_modified += 1;
        }
    }

    println!("Num of good strings via struct: {num_good_strings_modified}");
}

#[test]
fn test_is_good_string() {
    let mut line_checker = line_checker::LineChecker::new();
    line_checker.add_rule(RepeatWithGapRule::new(1));
    line_checker.add_rule(HasPairsRule::new(1));

    let text = String::from("qjhvhtzxzqqjkmpb");
    assert_eq!(true, line_checker.check(&text));

    let text = String::from("xxyxx");
    assert_eq!(true, line_checker.check(&text));

    let text = String::from("uurcxstgmygtbstg");
    assert_eq!(false, line_checker.check(&text));

    let text = String::from("ieodomkazucvgmuy");
    assert_eq!(false, line_checker.check(&text));
}
