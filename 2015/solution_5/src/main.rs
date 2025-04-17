use std::fs;

mod helpers;

struct VowelRule {
    threshold: u32,
    num_vowels: u32,
}

impl VowelRule {
    fn new(threshold: u32) -> Self {
        Self {
            threshold: threshold,
            num_vowels: 0,
        }
    }
}

impl helpers::Rule for VowelRule {
    fn process_char(&mut self, _: &String, _: usize, char: helpers::Byte) {
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
    let mut line_checker = LineChecker::new();
    line_checker.add_rule(VowelRule::new(3));

    let text = String::from("test");
    assert_eq!(false, line_checker.check(&text));
    let text = String::from("aaabcd");
    assert_eq!(true, line_checker.check(&text));
    let text = String::from("      ");
    assert_eq!(false, line_checker.check(&text));

    let mut line_checker = LineChecker::new();
    line_checker.add_rule(VowelRule::new(0));
    assert_eq!(true, line_checker.check(&text));

    let text = String::from("(@S@DM(92da!!#X");
    let mut line_checker = LineChecker::new();
    line_checker.add_rule(VowelRule::new(1));
    assert_eq!(true, line_checker.check(&text));
}

struct ReccuringLettersRule {
    num_occurences: u32,
    threshold: u32,
    current_char: u8,
}

impl ReccuringLettersRule {
    fn new(threshold: u32) -> Self {
        Self {
            threshold: threshold,
            num_occurences: 1,
            current_char: 0,
        }
    }
}

impl helpers::Rule for ReccuringLettersRule {
    fn process_char(&mut self, _: &String, index: usize, char: helpers::Byte) {
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
    let mut line_checker = LineChecker::new();
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

struct ForbiddenCharsRule {
    is_forbidden: bool,
}

impl ForbiddenCharsRule {
    fn new() -> Self {
        ForbiddenCharsRule {
            is_forbidden: false,
        }
    }
}

impl helpers::Rule for ForbiddenCharsRule {
    /*
    Checks if any of the following sequences are present in the string:
    ab, cd, pq, xy
    */
    fn process_char(&mut self, line: &String, index: usize, char: helpers::Byte) {
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
    let mut line_checker = LineChecker::new();
    line_checker.add_rule(ForbiddenCharsRule::new());

    let text = String::from("abcdefg");
    assert_eq!(false, line_checker.check(&text));
    let text = String::from("1111111");
    assert_eq!(true, line_checker.check(&text));
    let text = String::from("axyb");
    assert_eq!(false, line_checker.check(&text));
}

struct RepeatWithGapRule {
    gap: usize,
    occurred: bool,
}

impl RepeatWithGapRule {
    fn new(gap: usize) -> Self {
        RepeatWithGapRule {
            gap: gap,
            occurred: false,
        }
    }
}

impl helpers::Rule for RepeatWithGapRule {
    fn process_char(&mut self, line: &String, index: usize, char: helpers::Byte) {
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
    let mut line_checker = LineChecker::new();
    line_checker.add_rule(RepeatWithGapRule::new(1));

    let text = String::from("");
    assert_eq!(false, line_checker.check(&text));

    let text = String::from("xyx");
    assert_eq!(true, line_checker.check(&text));

    let text = String::from("abcdefeghi");
    assert_eq!(true, line_checker.check(&text));

    let text = String::from("aaa");
    assert_eq!(true, line_checker.check(&text));

    let mut line_checker = LineChecker::new();
    line_checker.add_rule(RepeatWithGapRule::new(3));

    let text = String::from("xaaax");
    assert_eq!(true, line_checker.check(&text));
}

struct HasPairsRule {
    occurences: usize,
    occurred: bool,
}

impl HasPairsRule {
    fn new(occurences: usize) -> Self {
        HasPairsRule {
            occurences: occurences,
            occurred: false,
        }
    }
}

impl helpers::Rule for HasPairsRule {
    fn process_char(&mut self, line: &String, index: usize, _: helpers::Byte) {
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
    let mut line_checker = LineChecker::new();
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

struct RuleRow {
    rule: Box<dyn helpers::Rule>,
    passed: bool,
}

struct LineChecker {
    line: String,
    rules: Vec<RuleRow>,
}

impl LineChecker {
    //TODO: Why is the + 'static' needed here?
    fn add_rule<T: helpers::Rule + 'static>(&mut self, rule: T) {
        self.rules.push(RuleRow {
            rule: Box::new(rule),
            passed: false,
        });
    }

    fn new() -> Self {
        Self {
            line: String::from(""),
            rules: vec![],
        }
    }

    fn check(&mut self, text: &String) -> bool {
        self.line = text.to_string();
        self.reset_rules();
        for (index, char) in text.as_bytes().iter().enumerate() {
            self.run_rules_on_byte(index, *char);
        }

        let is_good_string = self.is_good_string();

        is_good_string
    }

    fn run_rules_on_byte(&mut self, index: usize, char: helpers::Byte) {
        for rule_row in &mut self.rules {
            let rule = &mut rule_row.rule;
            rule.process_char(&self.line, index, char);
            if rule.passes() {
                rule_row.passed = true;
            } else {
                rule_row.passed = false;
            }
        }
    }

    fn is_good_string(&mut self) -> bool {
        let mut is_good = true;

        for rule_row in self.rules.iter() {
            if rule_row.passed == false {
                is_good = false;
                break;
            }
        }

        is_good
    }

    fn reset_rules(&mut self) {
        for rule_row in self.rules.iter_mut() {
            rule_row.passed = false;
            rule_row.rule.reset();
        }
    }
}

fn main() {
    let mut line_checker = LineChecker::new();
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
    let mut line_checker = LineChecker::new();
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
