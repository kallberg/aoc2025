fn main() {
    println!("{}", aoc2025::d11::part_1(aoc2025::input::D11));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_passes_example() {
        assert_eq!(aoc2025::d11::part_1(aoc2025::input::D11E), "5")
    }
}
