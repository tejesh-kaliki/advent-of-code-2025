pub fn run() {
    let input = include_str!("../../inputs/day03.txt");
    let parsed_input = input;
    println!("Part 1: {}", part1(parsed_input));
    println!("Part 2: {}", part2(parsed_input));
}

fn find_line_score(line: &str, n: usize) -> i64 {
    let nums: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    let mut value: i64 = 0;

    let mut next_start_index = 0;
    let len = nums.len();
    for i in 0..n {
        let max_num = nums[next_start_index..len - n + 1 + i]
            .iter()
            .max()
            .expect("Must have max num");
        next_start_index = nums[next_start_index..]
            .iter()
            .position(|x| *x == *max_num)
            .expect("Element must be present")
            + next_start_index
            + 1;
        value = value * 10 + *max_num as i64;
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
