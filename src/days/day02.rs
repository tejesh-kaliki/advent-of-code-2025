pub fn run() {
    let input = include_str!("../../inputs/day02.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse_range(range_str: &str) -> (i64, i64) {
    let (s, e) = range_str.split_once("-").expect("Must be a valid range");
    (
        s.parse().expect("Must be valid integer"),
        e.parse().expect("Must be valid integer"),
    )
}

fn part1(input: &str) -> i64 {
    let mut ranges: Vec<(i64, i64)> = input.trim().split(",").map(parse_range).collect();
    ranges.sort();

    let mut count = 0;
    for (start, end) in ranges {
        for i in start..=end {
            let digits = i.to_string().len();
            if digits % 2 != 0 {
                continue;
            }

            let factor = (10 as i64).pow(digits as u32 / 2);
            if i % (factor + 1) == 0 {
                count += i
            }
        }
    }

    count
}

fn part2(input: &str) -> i64 {
    let mut ranges: Vec<(i64, i64)> = input.trim().split(",").map(parse_range).collect();
    ranges.sort();

    let mut count = 0;
    for (start, end) in ranges {
        for i in start..=end {
            let i_str = i.to_string();
            let digits = i_str.len();
            for j in 1..digits {
                if digits % j != 0 {
                    continue;
                }

                if i_str[0..j].repeat(digits / j) == i_str {
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
        assert_eq!(part1(input), 1227775554);
        assert_eq!(part2(input), 4174379265);
    }
}
