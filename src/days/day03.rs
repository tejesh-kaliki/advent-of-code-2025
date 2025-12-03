pub fn run() {
    let input = include_str!("../../inputs/day03.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn find_line_score(line: &str, n: usize) -> i64 {
    let nums = line.as_bytes();
    let mut value: i64 = 0;

    let mut start = 0;
    let len = nums.len();
    for i in 0..n {
        let end = len - n + i;
        let (idx, &max_digit) = nums[start..=end]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|(_, d)| *d)
            .unwrap();

        start = idx + start + 1;
        value = value * 10 + (max_digit - b'0') as i64;
    }
    value
}

fn part1(input: &str) -> i64 {
    input.trim().lines().map(|x| find_line_score(x, 2)).sum()
}

fn part2(input: &str) -> i64 {
    input.trim().lines().map(|x| find_line_score(x, 12)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(part1(input), 357);
        assert_eq!(part2(input), 3121910778619);
    }
}
