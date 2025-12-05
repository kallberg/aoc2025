use std::ops::RangeInclusive;

fn parse_id_ranges(input: &str) -> Vec<RangeInclusive<u64>> {
    let mut ranges = vec![];
    for line in input.lines() {
        let Some((left, right)) = line.split_once('-') else {
            continue;
        };
        let Ok(start) = left.parse::<u64>() else {
            continue;
        };
        let Ok(end) = right.parse::<u64>() else {
            continue;
        };
        ranges.push(start..=end);
    }
    ranges
}

fn parse_ids(input: &str) -> Vec<u64> {
    input.lines().flat_map(|line| line.parse()).collect()
}

fn split_input(input: &str) -> (&str, &str) {
    input.split_once("\n\n").expect("empty line")
}

fn combine_range(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> Option<RangeInclusive<u64>> {
    let a_start = *a.start();
    let a_end = *a.end();
    let b_start = *b.start();
    let b_end = *b.end();
    let max_start = a_start.max(b_start);
    let max_end = a_end.min(b_end);
    if max_start <= max_end || max_start == max_end + 1 {
        let new_start = a_start.min(b_start);
        let new_end = a_end.max(b_end);
        Some(new_start..=new_end)
    } else {
        None
    }
}

fn combine_ranges(ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    if ranges.is_empty() {
        return Vec::new();
    }
    let mut sorted_ranges = ranges;
    sorted_ranges.sort_unstable_by_key(|r| *r.start());
    let mut current = sorted_ranges.remove(0);
    let mut combined_ranges = vec![];
    for next in sorted_ranges {
        if let Some(combined) = combine_range(&current, &next) {
            current = combined;
        } else {
            combined_ranges.push(current);
            current = next
        }
    }
    combined_ranges.push(current);
    combined_ranges
}

pub fn part_1(input: &str) -> String {
    let (input_1, input_2) = split_input(input);
    let ranges = parse_id_ranges(input_1);
    let ids = parse_ids(input_2);

    let mut counter = 0;

    'id_search: for id in ids {
        for range in ranges.clone() {
            if range.contains(&id) {
                counter += 1;
                continue 'id_search;
            }
        }
    }

    counter.to_string()
}

pub fn part_2(input: &str) -> String {
    let (input_1, _) = split_input(input);
    let ranges = combine_ranges(parse_id_ranges(input_1));

    let mut id_counter = 0;

    for range in ranges {
        let size = *range.end() - *range.start() + 1;
        id_counter += size;
    }

    id_counter.to_string()
}
