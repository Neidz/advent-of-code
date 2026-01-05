fn main() {
    part1();
    part2();
}

fn part1() {
    let mut password = INPUT.to_owned();

    loop {
        password = next_password(&password);
        if is_valid(&password) {
            break;
        }
    }

    println!("Part 1: valid password is {password}");
}

fn part2() {}

fn next_password(pass: &str) -> String {
    let mut pass_chars = pass.chars().collect::<Vec<char>>();

    let mut i = pass_chars.len() - 1;

    loop {
        let (next, wrapping) = next_char(pass_chars[i]);

        pass_chars[i] = next;

        if !wrapping {
            break;
        }

        i -= 1;
    }

    pass_chars.iter().collect::<String>()
}

fn is_valid(pass: &str) -> bool {
    let pass_chars = pass.chars().collect::<Vec<char>>();

    let mut i = 0usize;

    loop {
        if i < pass.len() - 3 {
            return false;
        }

        let first = pass_chars[i];
        let second = pass_chars[i + 1];
        let third = pass_chars[i + 2];

        let (valid_second, _) = next_char(first);
        let (valid_third, _) = next_char(valid_second);

        if second == valid_second && third == valid_third {
            break;
        }

        i += 1;
    }

    if pass.contains('i') || pass.contains('l') || pass.contains('o') {
        return false;
    }

    let mut pairs = 0usize;
    let mut i = 0usize;

    loop {
        if i < pass.len() - 2 {
            return false;
        }

        let first = pass_chars[i];
        let second = pass_chars[i + 1];

        if first == second {
            pairs += 1;
        }

        if pairs == 2 {
            break;
        }

        i += 1;
    }

    true
}

type Wraps = bool;

fn next_char(c: char) -> (char, Wraps) {
    let num = c as u8;
    let next = num + 1;

    if next == 123 {
        return ('a', true);
    }

    (next as char, false)
}

const INPUT: &str = "vzbxkghb";
