use crate::helpers;
struct RuleRow {
    rule: Box<dyn helpers::Rule>,
    passed: bool,
}

pub struct LineChecker {
    line: String,
    rules: Vec<RuleRow>,
}

impl LineChecker {
    //TODO: Why is the + 'static' needed here?
    pub fn add_rule<T: helpers::Rule + 'static>(&mut self, rule: T) {
        self.rules.push(RuleRow {
            rule: Box::new(rule),
            passed: false,
        });
    }

    pub fn new() -> Self {
        Self {
            line: String::from(""),
            rules: vec![],
        }
    }

    pub fn check(&mut self, text: &String) -> bool {
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
