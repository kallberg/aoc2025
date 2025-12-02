fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    let mut ranges = vec![];
    for range_str in input.split(',') {
        let (left_str, right_str) = range_str
            .trim()
            .split_once('-')
            .unwrap_or_else(|| panic!("split by='-' range_str={range_str} failed"));

        let left = left_str
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("parse left_str={left_str} to u64 failed"));
        let right = right_str
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("parse right_str={right_str} to u64 failed"));

        ranges.push((left, right));
    }

    ranges
}

fn invalid_part_2(input: &str) -> bool {
    let chars = input.as_bytes();
    let half = input.len() / 2;
    let first_half = &chars[0..half];

    for size in 1..=half {
        if input.len() % size != 0 {
            continue;
        }
        'window: for window in first_half.windows(size) {
            if window.starts_with(b"0") {
                continue;
            }

            for chunk in chars.chunks(size) {
                if window != chunk {
                    continue 'window;
                }
            }
            return true;
        }
    }
    false
}

pub fn part_2(input: &str) -> String {
    let mut sum = 0;
    for (start, end) in parse_ranges(input) {
        // println!("checking {start}..={end}");
        for entry in start..=end {
            let entry_str = entry.to_string();
            if invalid_part_2(&entry_str) {
                // println!("{entry}");
                sum += entry;
            }
        }
    }

    sum.to_string()
}

pub fn part_1(input: &str) -> String {
    let mut sum = 0;
    for (start, end) in parse_ranges(input) {
        for entry in start..=end {
            let entry_str = entry.to_string();
            if entry_str.len() % 2 == 0 {
                let (left, right) = entry_str.split_at(entry_str.len() / 2);
                assert!(left.len() == right.len());
                if left == right {
                    sum += entry;
                }
            }
        }
    }

    sum.to_string()
}
