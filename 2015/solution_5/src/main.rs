use std::fs;

type Byte = u8;

trait Rule {
    fn process_char(&mut self, line: &String, index: usize, char: &Byte);
    fn is_true(&self) -> bool;
    fn reset(&mut self);
}

struct VowelRule {
    threshold: u32,
    num_vowels: u32,
}

impl Rule for VowelRule {
    fn process_char(&mut self, _: &String, _: usize, char: &Byte) {
        let vowels = [b'a', b'e', b'i', b'o', b'u'];

        if vowels.contains(char) {
            self.num_vowels += 1;
        }
    }

    fn is_true(&self) -> bool {
        self.num_vowels >= self.threshold
    }

    fn reset(&mut self) {
        self.num_vowels = 0;
    }
}

impl VowelRule {
    fn new(threshold: u32) -> Self {
        Self {
            threshold: threshold,
            num_vowels: 0,
        }
    }
}

struct ReccuringLettersRule {
    num_occurences: u32,
    threshold: u32,
    current_char: u8,
}

impl Rule for ReccuringLettersRule {
    fn process_char(&mut self, _: &String, index: usize, char: &Byte) {
        let char = *char;

        if index == 0 {
            self.current_char = char;
            return;
        }

        if char != self.current_char {
            self.num_occurences = 1;
            self.current_char = char;
            return;
        }

        self.num_occurences += 1;
    }

    fn is_true(&self) -> bool {
        self.num_occurences >= self.threshold
    }

    fn reset(&mut self) {
        self.num_occurences = 0;
        self.current_char = 0;
    }
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

struct ForbiddenCharsRule {
    is_forbidden: bool,
}

impl Rule for ForbiddenCharsRule {
    fn process_char(&mut self, line: &String, index: usize, char: &Byte) {
        let suspicious_chars = [b'b', b'd', b'q', b'y'];
        let text_as_bytes = line.as_bytes();

        //Skip first element as there's nothing to compare it against
        if index == 0 {
            return;
        }

        if !suspicious_chars.contains(char) {
            return;
        }

        self.is_forbidden = if text_as_bytes[index - 1] == char - 1 {
            true
        } else {
            false
        };
    }

    fn is_true(&self) -> bool {
        !self.is_forbidden
    }

    fn reset(&mut self) {
        self.is_forbidden = false;
    }
}

/*
Checks if any of the following sequences is present in the string:
ab, cd, pq, xy
*/
fn has_forbidden_sequences(text: &str) -> bool {
    let suspicious_chars = [b'b', b'd', b'q', b'y'];
    let text_as_bytes = text.as_bytes();

    for (i, v) in text_as_bytes.iter().enumerate() {
        //Skip first element as there's nothing to compare it against
        if i == 0 {
            continue;
        }

        if !suspicious_chars.contains(v) {
            continue;
        }

        if text_as_bytes[i - 1] == v - 1 {
            return true;
        }
    }
    false
}

impl ForbiddenCharsRule {
    fn new() -> Self {
        ForbiddenCharsRule {
            is_forbidden: false,
        }
    }
}

struct RuleRow {
    rule: Box<dyn Rule>,
    passed: bool,
}

struct LineChecker {
    line: String,
    rules: Vec<RuleRow>,
}

impl LineChecker {
    //TODO: Why is the + 'static' needed here?
    fn add_rule<T: Rule + 'static>(&mut self, rule: T) {
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

    fn run_on(&mut self, text: &String) {
        self.line = text.to_string();
        for (index, char) in text.as_bytes().iter().enumerate() {
            self.run_rules_on_byte(index, &char);
        }
    }

    fn run_rules_on_byte(&mut self, index: usize, char: &Byte) {
        for rule_row in &mut self.rules {
            let rule = &mut rule_row.rule;
            rule.process_char(&self.line, index, &char);
            if rule.is_true() {
                rule_row.passed = true;
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

        self.reset_rules();
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
    line_checker.add_rule(VowelRule::new(3));
    line_checker.add_rule(ReccuringLettersRule::new(2));
    line_checker.add_rule(ForbiddenCharsRule::new());

    let text = match fs::read_to_string("input.txt") {
        Ok(text) => text,
        Err(e) => panic!("Error reading file: {e}"),
    };

    let mut num_good_strings_original = 0;
    let mut num_good_strings_modified = 0;
    for line in text.lines() {
        //Struct implementation
        line_checker.run_on(&String::from(line));
        if line_checker.is_good_string() {
            num_good_strings_modified += 1;
        }

        //Original implementation
        if string_is_nice(&line) {
            num_good_strings_original += 1;
        }
    }

    println!("Num of good strings via struct: {num_good_strings_modified}");
    println!("Num of good strings original: {num_good_strings_original}");
}

fn string_is_nice(text: &str) -> bool {
    has_num_of_vowels(&text, 3)
        && has_reoccuring_letters(&text, 2)
        && !has_forbidden_sequences(&text)
}

#[test]
fn test_string_is_nice() {
    let text = String::from("ugknbfddgicrmopn");
    assert_eq!(true, string_is_nice(&text));

    let text = String::from("aaa");
    assert_eq!(true, string_is_nice(&text));

    let text = String::from("jchzalrnumimnmhp");
    assert_eq!(false, string_is_nice(&text));

    let text = String::from("haegwjzuvuyypxyu");
    assert_eq!(false, string_is_nice(&text));

    let text = String::from("dvszwmarrgswjxmb");
    assert_eq!(false, string_is_nice(&text));
}

fn has_num_of_vowels(text: &str, threshold: u32) -> bool {
    let vowels = [b'a', b'e', b'i', b'o', b'u'];

    let mut num_vowels = 0;

    for v in text.as_bytes().iter() {
        if vowels.contains(v) {
            num_vowels += 1;

            if num_vowels >= threshold {
                return true;
            }
        }
    }

    //Explicit return checker for case threshold = 0 nad vowels = 0
    num_vowels == threshold
}

#[test]
fn test_has_num_of_vowels() {
    let text = String::from("test");
    assert_eq!(false, has_num_of_vowels(&text, 3));
    let text = String::from("aaabcd");
    assert_eq!(true, has_num_of_vowels(&text, 3));
    let text = String::from("      ");
    assert_eq!(false, has_num_of_vowels(&text, 3));
    assert_eq!(true, has_num_of_vowels(&text, 0));
    let text = String::from("(@S@DM(92da!!#X");
    assert_eq!(true, has_num_of_vowels(&text, 1));
}

fn has_reoccuring_letters(text: &str, occurences_threshold: u32) -> bool {
    let text_as_bytes = text.as_bytes();
    let mut current_char = text_as_bytes[0];
    let mut occurences = 1;

    for (i, v) in text_as_bytes.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if *v != current_char {
            occurences = 1;
            current_char = *v;
            continue;
        }

        occurences += 1;

        if occurences == occurences_threshold {
            return true;
        }
    }
    false
}

#[test]
fn test_has_reoccuring_letters() {
    let text = String::from("abcdefg");
    assert_eq!(false, has_reoccuring_letters(&text, 2));
    let text = String::from("somethingxx");
    assert_eq!(true, has_reoccuring_letters(&text, 2));
    let text = String::from("xxsomething");
    assert_eq!(true, has_reoccuring_letters(&text, 2));
    let text = String::from("sometxxhing");
    assert_eq!(true, has_reoccuring_letters(&text, 2));
    let text = String::from("xX");
    assert_eq!(false, has_reoccuring_letters(&text, 2));
}

#[test]
fn test_has_forbidden_sequences() {
    let text = String::from("abcdefg");
    assert_eq!(true, has_forbidden_sequences(&text));
    let text = String::from("1111111");
    assert_eq!(false, has_forbidden_sequences(&text));
    let text = String::from("axyb");
    assert_eq!(true, has_forbidden_sequences(&text));
}
