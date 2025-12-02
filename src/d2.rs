use std::ops::RangeInclusive;

fn parse_ranges(input: &str) -> Vec<RangeInclusive<u64>> {
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

        ranges.push(left..=right);
    }

    ranges
}

fn digit_len(value: u64) -> u32 {
    value.checked_ilog10().unwrap_or(0) + 1
}

fn slice_u64(value: u64, index: u32, length: u32) -> u64 {
    let digits = digit_len(value);

    if index >= digits {
        return 0;
    }

    let drop_right = digits.saturating_sub(index + length);
    let mut sliced = value / 10u64.pow(drop_right);

    if length < digits {
        sliced %= 10u64.pow(length);
    }

    sliced
}

fn slice_middle(value: u64) -> (u64, u64) {
    let digits = digit_len(value);
    let half = digits / 2;

    let left = slice_u64(value, 0, half);
    let right = slice_u64(value, half, half);
    (left, right)
}

fn invalid_part_2(input: u64) -> bool {
    if input == 0 {
        return false;
    }

    let digits = input.ilog10() + 1;
    let half = digits / 2;

    'size: for size in 1..=half {
        if digits % size != 0 {
            continue;
        }
        let pattern = slice_u64(input, 0, size);

        for index in (size..digits).step_by(size as usize) {
            let needle = slice_u64(input, index, size);
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
    for range in parse_ranges(input) {
        for entry in range {
            if invalid_part_2(entry) {
                sum += entry;
            }
        }
    }

    sum.to_string()
}

pub fn part_1(input: &str) -> String {
    let mut sum = 0;
    for range in parse_ranges(input) {
        for entry in range {
            let digits = digit_len(entry);
            if digits % 2 == 0 {
                let (left, right) = slice_middle(entry);
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
    use crate::d2::{invalid_part_2, slice_middle};

    #[test]
    fn test_slice_middle() {
        assert_eq!(slice_middle(11), (1, 1));
        assert_eq!(slice_middle(1111), (11, 11));
        assert_eq!(slice_middle(110011), (110, 11));
        assert_eq!(slice_middle(11011011), (1101, 1011));
    }

    #[test]
    fn test_invalid_part_2() {
        assert!(invalid_part_2(11));
        assert!(invalid_part_2(22));
        assert!(invalid_part_2(99));
        assert!(invalid_part_2(1010));
        assert!(invalid_part_2(1188511885));
        assert!(invalid_part_2(222222));
        assert!(invalid_part_2(446446));
        assert!(invalid_part_2(38593859));
    }

    #[test]
    fn test_valid_part_2() {
        for value in 1698522..=1698528 {
            assert!(!invalid_part_2(value));
        }
    }
}
