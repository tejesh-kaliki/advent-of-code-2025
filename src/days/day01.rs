use std::{fs, time::Instant};

pub fn run() {
    let (input, t_input) = time!(fs::read_to_string("inputs/day01.txt").unwrap());
    let (parsed, t_parse) = time!(parse_input(&input));
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

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    input.lines().filter_map(parse_line).collect()
}

fn parse_line(line: &str) -> Option<(i32, i32)> {
    let (sign, rest) = match line.as_bytes().first()? {
        b'L' => (-1, &line[1..]),
        b'R' => (1, &line[1..]),
        _ => return None,
    };

    Some((sign, rest.parse::<i32>().ok()?))
}

fn part1(input: &[(i32, i32)]) -> i32 {
    let mut counter = 50;
    let mut res = 0;

    for (sign, v) in input {
        counter += sign * v;
        if counter % 100 == 0 {
            res += 1;
        }
    }

    res
}

fn count_multiples_in_range(range: i32, start: i32, end: i32) -> i32 {
    // Ensure the divisor is valid (non-zero)
    if range == 0 {
        return 0;
    }

    // start is excluded, end is included => (start, end]
    let (lo, hi) = if start < end {
        (start + 1, end)
    } else {
        // Handle inverted ranges gracefully
        (end, start - 1)
    };

    // Use Euclidean division, which aligns with the floor-based mathematical formula
    hi.div_euclid(range) - (lo - 1).div_euclid(range)
}

fn part2(input: &[(i32, i32)]) -> i32 {
    let mut counter = 50;
    let mut res = 0;

    for (sign, v) in input {
        let start = counter;
        let end = counter + sign * v;
        res += count_multiples_in_range(100, start, end);
        counter = end;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part1(&parse_input(input)), 3)
    }
    #[test]
    fn test_part2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L182";
        assert_eq!(part2(&parse_input(input)), 7);
        let input = "L68
L30
R48
L5
R60
L55";
        assert_eq!(part2(&parse_input(input)), 4);
    }
}
