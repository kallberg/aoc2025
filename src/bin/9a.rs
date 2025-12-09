fn main() {
    println!("{}", aoc2025::d9::part_1(aoc2025::input::D9));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_passes_example() {
        assert_eq!(aoc2025::d9::part_1(aoc2025::input::D9E), "50")
    }
}
