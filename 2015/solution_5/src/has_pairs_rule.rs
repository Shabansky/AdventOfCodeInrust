use crate::helpers::{Byte, Rule};
pub struct HasPairsRule {
    occurences: usize,
    occurred: bool,
}

impl HasPairsRule {
    pub fn new(occurences: usize) -> Self {
        HasPairsRule {
            occurences: occurences,
            occurred: false,
        }
    }
}

impl Rule for HasPairsRule {
    fn process_char(&mut self, line: &String, index: usize, _: Byte) {
        if self.occurred == true {
            return;
        }

        //Length needs to be at least two pairs in size to make any sense
        let min_text_length: usize = 4;
        if index + 1 < min_text_length {
            return;
        }

        let line_to_check = &line[..index - 1];
        let pair_to_check = &line[index - 1..=index];

        if line_to_check.contains(&pair_to_check) {
            self.occurred = true;
        }
    }

    fn passes(&self) -> bool {
        self.occurred == true
    }

    fn reset(&mut self) {
        self.occurences = 0;
        self.occurred = false;
    }
}

#[test]
fn test_has_pairs_rule() {
    use crate::line_checker;

    let mut line_checker = line_checker::LineChecker::new();
    line_checker.add_rule(HasPairsRule::new(1));

    // Test case: No pairs
    let text = String::from("abcdefg");
    assert_eq!(false, line_checker.check(&text));

    // Test case: Empty string
    let text = String::from("");
    assert_eq!(false, line_checker.check(&text));

    // Test case: Overlapping pairs. Not allowed
    let text = String::from("aaa");
    assert_eq!(false, line_checker.check(&text));

    // Test case: Pairs one next to another
    let text = String::from("xyxy");
    assert_eq!(true, line_checker.check(&text));

    // Test case: Pairs separated
    let text = String::from("aabcdefgaa");
    assert_eq!(true, line_checker.check(&text));
}
