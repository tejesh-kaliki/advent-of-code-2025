use std::{fs, time::Instant};

pub fn run() {
    let (input, t_input) = time!(fs::read_to_string("inputs/day06.txt").unwrap());
    let (parsed, t_parse) = time!(input);
    let (part1, t_part1) = time!(part1(&parsed));
    let (part2, t_part2) = time!(part2(&parsed));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    println!("Input time: {:?}", t_input);
    println!("Parse time: {:?}", t_parse);
    println!("Part 1 time: {:?}", t_part1);
    println!("Part 2 time: {:?}", t_part2);
    println!("Total time: {:?}", t_part2 + t_part1 + t_parse + t_input);
}

fn part1(input: &str) -> i64 {
    let lines = input
        .trim()
        .lines()
        .map(|line| line.trim().split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<_>>();

    let last_line_index = lines.len() - 1;
    let mut total = 0;
    for i in 0..lines[0].len() {
        let nums: Vec<i64> = (0..last_line_index)
            .map(|j| lines[j][i].parse::<i64>().unwrap())
            .collect();

        let result = match lines[last_line_index][i] {
            "+" => nums.iter().fold(0, |acc, x| acc + *x),
            "*" => nums.iter().fold(1, |acc, x| acc * *x),
            _ => continue,
        };
        total += result;
    }

    total
}

fn part2(input: &str) -> i64 {
    let lines = input
        .trim_end()
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let (ops, num_lines) = lines.split_last().unwrap();
    let last_char = num_lines.iter().map(|line| line.len()).max().unwrap();

    let mut total = 0;
    let mut buffer = Vec::new();
    for i in (0..last_char).rev() {
        let num_chars = num_lines.iter().filter_map(|line| line.get(i));
        let mut num = 0;
        let mut has_char = false;
        for c in num_chars {
            match c {
                x if x.is_ascii_digit() => num = num * 10 + (x - b'0') as i64,
                _ => continue,
            }
            has_char = true;
        }
        if !has_char {
            continue;
        }

        let op = ops.get(i);
        match op {
            Some(b'+') => {
                buffer.push(num);
                total += buffer.iter().sum::<i64>();
                buffer.clear();
            }
            Some(b'*') => {
                buffer.push(num);
                total += buffer.iter().product::<i64>();
                buffer.clear();
            }
            _ => {
                buffer.push(num);
                continue;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let input = "
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "
            .trim();
        assert_eq!(part1(input), 4277556);
        assert_eq!(part2(input), 3263827);
    }
}
