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
