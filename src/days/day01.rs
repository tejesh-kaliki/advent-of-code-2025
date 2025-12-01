pub fn run() {
    let input = include_str!("../../inputs/day01.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse_line(line: &str) -> Option<(i32, i32)> {
    let (sign, rest) = match line.as_bytes().first()? {
        b'L' => (-1, &line[1..]),
        b'R' => (1, &line[1..]),
        _ => return None,
    };

    Some((sign, rest.parse::<i32>().ok()?))
}

fn part1(input: &str) -> i32 {
    let mut counter = 50;
    let mut res = 0;

    for (sign, v) in input.lines().filter_map(parse_line) {
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

fn part2(input: &str) -> i32 {
    let mut counter = 50;
    let mut res = 0;

    for (sign, v) in input.lines().filter_map(parse_line) {
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
        assert_eq!(part1(input), 3)
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
        assert_eq!(part2(input), 7);
        let input = "L68
L30
R48
L5
R60
L55";
        assert_eq!(part2(input), 4);
    }
}
