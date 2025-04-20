use crate::helpers::{Byte, Rule};
pub struct RepeatWithGapRule {
    gap: usize,
    occurred: bool,
}

impl RepeatWithGapRule {
    pub fn new(gap: usize) -> Self {
        RepeatWithGapRule {
            gap: gap,
            occurred: false,
        }
    }
}

impl Rule for RepeatWithGapRule {
    fn process_char(&mut self, line: &String, index: usize, char: Byte) {
        if self.occurred == true {
            return;
        }

        //Checked string needs to be at least gap + 2 (1 adjusted for index) chars big to be processed
        if index < self.gap + 1 {
            return;
        }

        let line = &line[..index].as_bytes();

        let start_char = line[index - (self.gap + 1)];

        if start_char == char {
            self.occurred = true;
        }
    }

    fn passes(&self) -> bool {
        self.occurred == true
    }

    fn reset(&mut self) {
        self.occurred = false;
    }
}

#[test]
fn test_repeat_with_gap_rule() {
    use crate::line_checker;

    let mut line_checker = line_checker::LineChecker::new();
    line_checker.add_rule(RepeatWithGapRule::new(1));

    let text = String::from("");
    assert_eq!(false, line_checker.check(&text));

    let text = String::from("xyx");
    assert_eq!(true, line_checker.check(&text));

    let text = String::from("abcdefeghi");
    assert_eq!(true, line_checker.check(&text));

    let text = String::from("aaa");
    assert_eq!(true, line_checker.check(&text));

    let mut line_checker = line_checker::LineChecker::new();
    line_checker.add_rule(RepeatWithGapRule::new(3));

    let text = String::from("xaaax");
    assert_eq!(true, line_checker.check(&text));
}
