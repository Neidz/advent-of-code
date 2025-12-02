fn main() {
    part1();
    part2();
}

fn part1() {
    let ranges = get_ranges(INPUT);
    let mut sum = 0;

    for (min, max) in ranges {
        for i in min..=max {
            if is_repeating_sequence(i, Some(2)) {
                sum += i;
            }
        }
    }

    println!("Part1: sum of invalid ids is {sum}");
}

fn part2() {
    let ranges = get_ranges(INPUT);
    let mut sum = 0;

    for (min, max) in ranges {
        for i in min..=max {
            if is_repeating_sequence(i, None) {
                sum += i;
            }
        }
    }

    println!("Part2: sum of invalid ids is {sum}");
}

fn is_repeating_sequence(num: usize, max_sequences: Option<usize>) -> bool {
    let str_num = num.to_string();

    let mut chunk_size = 1;

    loop {
        if let Some(max) = max_sequences
            && str_num.len() / chunk_size > max
        {
            chunk_size += 1;
            continue;
        }

        if str_num.len() < chunk_size * 2 {
            return false;
        }

        let chunks = str_num
            .as_bytes()
            .chunks(chunk_size)
            .map(|c| String::from_utf8_lossy(c).into_owned())
            .collect::<Vec<String>>();

        let first_seq = &chunks[0];

        if chunks.iter().all(|seq| seq == first_seq) {
            return true;
        }

        chunk_size += 1;
    }
}

fn get_ranges(s: &str) -> Vec<(usize, usize)> {
    s.split(',')
        .map(|pair| pair.split_once('-').unwrap())
        .map(|(min, max)| (min.parse::<usize>().unwrap(), max.parse::<usize>().unwrap()))
        .collect()
}

const _TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

const INPUT: &str = "9595822750-9596086139,1957-2424,88663-137581,48152-65638,12354817-12385558,435647-489419,518494-609540,2459-3699,646671-688518,195-245,295420-352048,346-514,8686839668-8686892985,51798991-51835611,8766267-8977105,2-17,967351-995831,6184891-6331321,6161577722-6161678622,912862710-913019953,6550936-6625232,4767634976-4767662856,2122995-2257010,1194-1754,779-1160,22-38,4961-6948,39-53,102-120,169741-245433,92902394-92956787,531-721,64-101,15596-20965,774184-943987,8395-11781,30178-47948,94338815-94398813";
