use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day05.txt").expect("Must read the file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn parse_range(range_str: &str) -> (i64, i64) {
    let (s, e) = range_str.split_once("-").expect("Must be a valid range");
    (
        s.parse().expect("Must be valid integer"),
        e.parse().expect("Must be valid integer"),
    )
}

fn part1(input: &str) -> i16 {
    let (ranges_str, items_str) = input.trim().split_once("\n\n").expect("Must split input");

    let ranges: Vec<(i64, i64)> = ranges_str.lines().map(parse_range).collect();

    let mut count = 0;
    for item in items_str
        .lines()
        .map(|item| item.parse::<i64>().expect("Must be integer"))
    {
        for &(start, end) in &ranges {
            if item >= start && item <= end {
                count += 1;
                break;
            }
        }
    }

    count
}

fn part2(input: &str) -> i64 {
    let (ranges_str, _) = input.trim().split_once("\n\n").expect("Must split input");

    let mut ranges: Vec<(i64, i64)> = ranges_str.lines().map(parse_range).collect();
    ranges.sort_by_key(|(s, _)| *s);

    let mut new_ranges = Vec::new();
    for (start, end) in ranges {
        match new_ranges.pop() {
            None => new_ranges.push((start, end)),
            Some((last_start, last_end)) if last_end >= start => {
                new_ranges.push((last_start, last_end.max(end)))
            }
            Some(last_range) => new_ranges.extend_from_slice(&[last_range, (start, end)]),
        }
    }

    let mut count = 0;
    for (start, end) in new_ranges {
        count += end - start + 1;
    }

    count
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
        assert_eq!(part1(input), 3);
        assert_eq!(part2(input), 14);
    }
}
