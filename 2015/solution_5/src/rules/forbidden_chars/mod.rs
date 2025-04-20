use crate::helpers::{Byte, Rule};
pub struct ForbiddenCharsRule {
    is_forbidden: bool,
}

impl ForbiddenCharsRule {
    pub fn new() -> Self {
        ForbiddenCharsRule {
            is_forbidden: false,
        }
    }
}

impl Rule for ForbiddenCharsRule {
    /*
    Checks if any of the following sequences are present in the string:
    ab, cd, pq, xy
    */
    fn process_char(&mut self, line: &String, index: usize, char: Byte) {
        //Skip first element as there's nothing to compare it against
        if index == 0 {
            return;
        }

        if self.is_forbidden == true {
            return;
        }

        let suspicious_chars = [b'b', b'd', b'q', b'y'];

        if !suspicious_chars.contains(&char) {
            self.is_forbidden = false;
            return;
        }

        let text_as_bytes = &line[..index].as_bytes();
        self.is_forbidden = if text_as_bytes[index - 1] == char - 1 {
            true
        } else {
            false
        };
    }

    fn passes(&self) -> bool {
        !self.is_forbidden
    }

    fn reset(&mut self) {
        self.is_forbidden = false;
    }
}

#[test]
fn test_has_forbidden_sequences() {
    use crate::line_checker;

    let mut line_checker = line_checker::LineChecker::new();
    line_checker.add_rule(ForbiddenCharsRule::new());

    let text = String::from("abcdefg");
    assert_eq!(false, line_checker.check(&text));
    let text = String::from("1111111");
    assert_eq!(true, line_checker.check(&text));
    let text = String::from("axyb");
    assert_eq!(false, line_checker.check(&text));
}
