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

    'size: for size in 1..=half {
        if input.len() % size != 0 {
            continue;
        }
        let mut pattern = None;
        for needle in chars.chunks(size) {
            let Some(pattern) = pattern else {
                pattern = Some(needle);
                continue;
            };
            if needle != pattern {
                continue 'size;
            }
        }
        return true;
    }
    false
}

pub fn part_2(input: &str) -> String {
    let mut sum = 0;
    for (start, end) in parse_ranges(input) {
        for entry in start..=end {
            let entry_str = entry.to_string();
            if invalid_part_2(&entry_str) {
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

#[cfg(test)]
mod tests {
    use crate::d2::invalid_part_2;

    #[test]
    fn test_invalid_part_2() {
        assert!(invalid_part_2("11"));
        assert!(invalid_part_2("22"));
        assert!(invalid_part_2("99"));
        assert!(invalid_part_2("1010"));
        assert!(invalid_part_2("1188511885"));
        assert!(invalid_part_2("222222"));
        assert!(invalid_part_2("446446"));
        assert!(invalid_part_2("38593859"));
    }

    #[test]
    fn test_valid_part_2() {
        for value in 1698522..=1698528 {
            assert!(!invalid_part_2(&value.to_string()));
        }
    }
}
