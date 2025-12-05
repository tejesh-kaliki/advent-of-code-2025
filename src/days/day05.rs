use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day05.txt").expect("Must read the file");
    let (ranges, items) = parse_input(&input);
    println!("Part 1: {}", part1(&ranges, &items));
    println!("Part 2: {}", part2(&ranges));
}

fn parse_range(range_str: &str) -> (i64, i64) {
    let (s, e) = range_str.split_once("-").expect("Must be a valid range");
    (
        s.parse().expect("Must be valid integer"),
        e.parse().expect("Must be valid integer"),
    )
}

fn parse_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let (ranges_str, items_str) = input.trim().split_once("\n\n").expect("Must split input");

    let mut given_ranges: Vec<(i64, i64)> = ranges_str.lines().map(parse_range).collect();
    given_ranges.sort_by_key(|&(start, _)| start);

    let mut ranges = Vec::new();
    for (start, end) in given_ranges {
        match ranges.pop() {
            None => ranges.push((start, end)),
            Some((last_start, last_end)) if last_end >= start => {
                ranges.push((last_start, last_end.max(end)))
            }
            Some(last_range) => ranges.extend_from_slice(&[last_range, (start, end)]),
        }
    }

    let items = items_str
        .lines()
        .map(|item| item.parse().expect("Must be integer"))
        .collect();

    (ranges, items)
}

fn part1(ranges: &[(i64, i64)], items: &[i64]) -> i16 {
    let mut count = 0;
    for &item in items {
        for &(start, end) in ranges {
            if (start..=end).contains(&item) {
                count += 1;
                break;
            }
        }
    }

    count
}

fn part2(ranges: &[(i64, i64)]) -> i64 {
    ranges.iter().map(|&(s, e)| e - s + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let (ranges, items) = parse_input(input);
        assert_eq!(part1(&ranges, &items), 3);
        assert_eq!(part2(&ranges), 14);
    }
}
