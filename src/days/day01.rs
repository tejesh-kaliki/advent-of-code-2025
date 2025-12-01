pub fn run() {
    let input = include_str!("../../inputs/day01.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut counter = 50;
    let mut res = 0;

    for line in input.lines() {
        let (sign, num_str) = match line.chars().next() {
            Some('L') => (-1, &line[1..]),
            Some('R') => (1, &line[1..]),
            _ => continue,
        };

        let value: i32 = num_str.parse().unwrap();
        counter += sign * value;

        if counter % 100 == 0 {
            res += 1;
        }
    }

    res
}

fn part2(input: &str) -> i32 {
    let mut counter = 50;
    let mut res = 0;

    for line in input.lines() {
        let (sign, num_str) = match line.chars().next() {
            Some('L') => (-1, &line[1..]),
            Some('R') => (1, &line[1..]),
            _ => continue,
        };

        let value: i32 = num_str.parse().unwrap();
        for _ in 0..value {
            counter += sign;
            if counter % 100 == 0 {
                res += 1;
            }
        }
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
        assert_eq!(part2(input), 5);
    }
}
