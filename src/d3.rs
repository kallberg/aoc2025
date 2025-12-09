fn joltage_from_char(value: char) -> u64 {
    match value {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => 0,
    }
}

fn parse_max_with_min_rem(bank: &[u64], min_rem: usize) -> (u64, &[u64]) {
    let size = bank.len();
    let mut max = 0;
    let mut max_index = 0;
    for (index, value) in bank.iter().enumerate() {
        if size - index <= min_rem {
            break;
        }
        if *value > max {
            max = *value;
            max_index = index;
        }
    }

    (max, &bank[max_index + 1..])
}

fn bank_from_line(input: &str) -> Vec<u64> {
    input.chars().map(joltage_from_char).collect()
}

pub fn part_1(input: &str) -> String {
    let mut sum_joltage = 0;
    for line in input.lines() {
        let bank = bank_from_line(line);

        // for (index, char) in line.chars().enumerate() {
        //     let last = index + 1 == length;
        //     let joltage = joltage_from_char(char);

        //     if joltage > max_one && !last {
        //         max_one = joltage;
        //         max_two = 0;
        //     } else if joltage > max_two {
        //         max_two = joltage
        //     }
        // }

        let (max_one, bank_rem) = parse_max_with_min_rem(&bank, 1);
        let (max_two, _) = parse_max_with_min_rem(bank_rem, 0);

        let bank_joltage = max_one * 10 + max_two;
        sum_joltage += bank_joltage;
    }

    sum_joltage.to_string()
}

pub fn part_2(input: &str) -> String {
    let mut sum_joltage = 0;
    for line in input.lines() {
        let bank = bank_from_line(line);
        let mut bank_rem: &[u64] = &bank;
        let mut bank_joltage = 0;

        for position in (0..12).rev() {
            let (value, rem) = parse_max_with_min_rem(bank_rem, position);
            bank_rem = rem;

            let joltage_value = value * 10u64.pow(position as u32);
            bank_joltage += joltage_value;
        }

        sum_joltage += bank_joltage;
    }

    sum_joltage.to_string()
}

#[cfg(test)]
mod tests {
    use crate::d3::parse_max_with_min_rem;

    #[test]
    fn test_parse_max_with_min_rem() {
        assert_eq!(parse_max_with_min_rem(&[1, 2, 3, 4], 2), (2, &[3, 4][..]));
        assert_eq!(parse_max_with_min_rem(&[1, 2, 3, 4], 0), (4, &[][..]));
    }
}
