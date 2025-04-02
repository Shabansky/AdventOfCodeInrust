fn main() {}

fn has_num_of_vowels(text: &String, threshold: u32) -> bool {
    let vowels = [b'a', b'e', b'i', b'o', b'u'];

    let mut num_vowels = 0;

    for v in text.as_bytes().iter() {
        if vowels.contains(v) {
            num_vowels += 1;
        }
    }

    num_vowels >= threshold
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

fn has_reoccuring_letters(text: &String, occurences_threshold: u32) -> bool {
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
fn has_forbidden_sequences(text: &String) -> bool {
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
