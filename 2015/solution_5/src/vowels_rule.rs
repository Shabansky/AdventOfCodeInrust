use crate::helpers::{Byte, Rule};

pub struct VowelRule {
    threshold: u32,
    num_vowels: u32,
}

impl VowelRule {
    pub fn new(threshold: u32) -> Self {
        Self {
            threshold: threshold,
            num_vowels: 0,
        }
    }
}

impl Rule for VowelRule {
    fn process_char(&mut self, _: &String, _: usize, char: Byte) {
        let vowels = [b'a', b'e', b'i', b'o', b'u'];

        if vowels.contains(&char) {
            self.num_vowels += 1;
        }
    }

    fn passes(&self) -> bool {
        self.num_vowels >= self.threshold
    }

    fn reset(&mut self) {
        self.num_vowels = 0;
    }
}

#[test]
fn test_has_num_of_vowels() {
    use crate::line_checker;
    let mut line_checker = line_checker::LineChecker::new();
    line_checker.add_rule(VowelRule::new(3));

    let text = String::from("test");
    assert_eq!(false, line_checker.check(&text));
    let text = String::from("aaabcd");
    assert_eq!(true, line_checker.check(&text));
    let text = String::from("      ");
    assert_eq!(false, line_checker.check(&text));

    let mut line_checker = line_checker::LineChecker::new();
    line_checker.add_rule(VowelRule::new(0));
    assert_eq!(true, line_checker.check(&text));

    let text = String::from("(@S@DM(92da!!#X");
    let mut line_checker = line_checker::LineChecker::new();
    line_checker.add_rule(VowelRule::new(1));
    assert_eq!(true, line_checker.check(&text));
}
