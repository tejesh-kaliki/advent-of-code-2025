use std::{cmp::Ordering, fs, time::Instant};

pub fn run() {
    let (input, t_input) = time!(fs::read_to_string("inputs/day05.txt").unwrap());
    let ((ranges, items), t_parse) = time!(parse_input(&input));
    let (part1, t_part1) = time!(part1(&ranges, &items));
    let (part2, t_part2) = time!(part2(&ranges));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    println!("Input time: {:?}", t_input);
    println!("Parse time: {:?}", t_parse);
    println!("Part 1 time: {:?}", t_part1);
    println!("Part 2 time: {:?}", t_part2);
    println!("Total time: {:?}", t_part2 + t_part1 + t_parse + t_input);
}

fn parse_range(range_str: &str) -> (i64, i64) {
    let (s, e) = range_str.split_once('-').expect("Must be a valid range");
    (
        s.parse().expect("Must be valid integer"),
        e.parse().expect("Must be valid integer"),
    )
}

fn parse_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let (ranges_str, items_str) = input.trim().split_once("\n\n").expect("Must split input");

    let mut given_ranges: Vec<(i64, i64)> = ranges_str.lines().map(parse_range).collect();
    given_ranges.sort_by_key(|&(start, _)| start);

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    for (start, end) in given_ranges {
        if let Some(last) = ranges.last_mut()
            && last.1 >= start - 1
        {
            last.1 = last.1.max(end);
            continue;
        }
        ranges.push((start, end));
    }

    let items = items_str
        .lines()
        .map(|item| item.parse().expect("Must be integer"))
        .collect();

    (ranges, items)
}

fn contains(ranges: &[(i64, i64)], x: i64) -> bool {
    ranges
        .binary_search_by(|&(s, e)| {
            if x < s {
                Ordering::Greater
            } else if x > e {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
        .is_ok()
}

fn part1(ranges: &[(i64, i64)], items: &[i64]) -> usize {
    items.iter().filter(|&&item| contains(ranges, item)).count()
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
