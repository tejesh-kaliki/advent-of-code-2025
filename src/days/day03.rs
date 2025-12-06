use std::{fs, time::Instant};

pub fn run() {
    let (input, t_input) = time!(fs::read_to_string("inputs/day03.txt").unwrap());
    let (parsed, t_parse) = time!(input);
    let (part1, t_part1) = time!(part1(&parsed));
    let (part2, t_part2) = time!(part2(&parsed));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    println!("Input time: {:?}", t_input);
    println!("Parse time: {:?}", t_parse);
    println!("Part 1 time: {:?}", t_part1);
    println!("Part 2 time: {:?}", t_part2);
    println!("Total time: {:?}", t_part2 + t_part1 + t_input + t_parse);
}

/// Line score is the maximum n-digit number possible by using the digits from line in order.
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
