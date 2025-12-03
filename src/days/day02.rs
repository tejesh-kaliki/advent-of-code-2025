pub fn run() {
    let input = include_str!("../../inputs/day02.txt");
    let parsed_input = parse_input(input);
    println!("Part 1: {}", part1(&parsed_input));
    println!("Part 2: {}", part2(&parsed_input));
}

fn parse_range(range_str: &str) -> (i64, i64) {
    let (s, e) = range_str.split_once("-").expect("Must be a valid range");
    (
        s.parse().expect("Must be valid integer"),
        e.parse().expect("Must be valid integer"),
    )
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    let mut ranges: Vec<(i64, i64)> = input.trim().split(",").map(parse_range).collect();
    ranges.sort_unstable();
    ranges
}

fn is_repeated_str(s: &str, count: usize) -> bool {
    let n = s.len();
    let chunk_len = n / count;

    if !n.is_multiple_of(count) {
        return false;
    }

    let chunk = &s.as_bytes()[..chunk_len];
    s.as_bytes()[chunk_len..]
        .chunks(chunk_len)
        .all(|c| c == chunk)
}

fn part1(ranges: &[(i64, i64)]) -> i64 {
    let mut count = 0;
    for &(start, end) in ranges.iter() {
        for i in start..=end {
            if is_repeated_str(&i.to_string(), 2) {
                count += i;
            }
        }
    }

    count
}

fn part2(ranges: &[(i64, i64)]) -> i64 {
    let mut count = 0;
    for &(start, end) in ranges {
        for i in start..=end {
            let i_str = i.to_string();
            let digits = i_str.len();
            for j in 2..=digits {
                if is_repeated_str(&i_str, j) {
                    count += i;
                    break;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let parsed_input = parse_input(input);
        assert_eq!(part1(&parsed_input), 1227775554);
        assert_eq!(part2(&parsed_input), 4174379265);
    }
}
