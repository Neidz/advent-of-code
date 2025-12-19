use md5::{Digest, Md5};

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut num = 0;

    loop {
        let hash_input = format!("{INPUT}{num}");
        let hash = get_hash(&hash_input);
        if hash.starts_with("00000") {
            break;
        }

        num += 1;
    }

    println!("Part1: lowest input is: {num}");
}

fn part2() {
    let mut num = 0;

    loop {
        let hash_input = format!("{INPUT}{num}");
        let hash = get_hash(&hash_input);
        if hash.starts_with("000000") {
            break;
        }

        num += 1;
    }

    println!("Part1: lowest input is: {num}");
}

fn get_hash(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let bytes = hasher.finalize().to_vec();
    hex::encode(bytes)
}

const INPUT: &str = "bgvyzdsv";
