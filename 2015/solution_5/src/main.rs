use std::fs;

fn main() {
    let text = match fs::read_to_string("input.txt") {
        Ok(text) => text,
        Err(e) => panic!("Error reading file: {e}"),
    };

    let mut num_good_strings = 0;
    for line in text.lines() {
        if string_is_nice(line) {
            num_good_strings += 1;
        }
    }

    println!("Num of good strings: {num_good_strings}");
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

#[test]
fn test_has_forbidden_sequences() {
    let text = String::from("abcdefg");
    assert_eq!(true, has_forbidden_sequences(&text));
    let text = String::from("1111111");
    assert_eq!(false, has_forbidden_sequences(&text));
    let text = String::from("axyb");
    assert_eq!(true, has_forbidden_sequences(&text));
}
