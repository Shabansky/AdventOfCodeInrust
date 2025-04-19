use crate::helpers::{Byte, Rule};

pub struct ReccuringLettersRule {
    num_occurences: u32,
    threshold: u32,
    current_char: u8,
}

impl ReccuringLettersRule {
    pub fn new(threshold: u32) -> Self {
        Self {
            threshold: threshold,
            num_occurences: 1,
            current_char: 0,
        }
    }
}

impl Rule for ReccuringLettersRule {
    fn process_char(&mut self, _: &String, index: usize, char: Byte) {
        if index == 0 {
            self.current_char = char;
            return;
        }

        if self.passes() {
            return;
        }

        if char != self.current_char {
            self.num_occurences = 1;
            self.current_char = char;
            return;
        }

        self.num_occurences += 1;
    }

    fn passes(&self) -> bool {
        self.num_occurences >= self.threshold
    }

    fn reset(&mut self) {
        self.num_occurences = 1;
        self.current_char = 0;
    }
}

#[test]
fn test_has_reoccuring_letters() {
    use crate::line_checker;
    let mut line_checker = line_checker::LineChecker::new();
    line_checker.add_rule(ReccuringLettersRule::new(2));

    let text = String::from("abcdefg");
    assert_eq!(false, line_checker.check(&text));

    let text = String::from("somethingxx");
    assert_eq!(true, line_checker.check(&text));
    let text = String::from("xxsomething");
    assert_eq!(true, line_checker.check(&text));
    let text = String::from("sometxxhing");
    assert_eq!(true, line_checker.check(&text));
    let text = String::from("xX");
    assert_eq!(false, line_checker.check(&text));
}
