use md5;

const HASH_SECRET: &str = "ckczppom";
const NUM_CAP: i32 = 10_000_000;
fn main() {
    let mut i = 1;
    while i < NUM_CAP {
        let plaintext = String::from(HASH_SECRET) + &i.to_string();
        // Returns e.g. dc 94 5c 58 4c be 97 b6 97 7e 88 3d 9d a5 ae 18
        let hash = md5::compute(plaintext);
        let segs = &hash[0..3];

        if segs[0] == 0 && segs[1] == 0 && segs[2] < 16 {
            if segs[2] == 0 {
                println!("Six zeros: {i}");
            } else if segs[2] < 16 {
                println!("Five zeros: {i}");
            }
        }
        i += 1;
    }
}
