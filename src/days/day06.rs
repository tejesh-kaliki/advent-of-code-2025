use std::{fs, time::Instant};

pub fn run() {
    let (input, t_input) = time!(fs::read_to_string("inputs/day06.txt").unwrap());
    let ((ops, data_lines), t_parse) = time!(parse_input(&input));
    let (part1, t_part1) = time!(part1(&ops, &data_lines));
    let (part2, t_part2) = time!(part2(&ops, &data_lines));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    println!("Input time: {:?}", t_input);
    println!("Parse time: {:?}", t_parse);
    println!("Part 1 time: {:?}", t_part1);
    println!("Part 2 time: {:?}", t_part2);
    println!("Total time: {:?}", t_part2 + t_part1 + t_parse + t_input);
}

fn parse_input(input: &str) -> (&[u8], Vec<&str>) {
    let mut lines = input.trim_end().lines().collect::<Vec<_>>();
    let ops = lines.pop().unwrap();
    (ops.as_bytes(), lines)
}

fn part1(ops_line: &[u8], data_lines: &[&str]) -> i64 {
    let ops: Vec<u8> = ops_line
        .iter()
        .copied()
        .filter(|&b| b == b'+' || b == b'*')
        .collect();

    let mut acc: Vec<i64> = ops
        .iter()
        .map(|&op| if op == b'+' { 0 } else { 1 })
        .collect();

    for line in data_lines {
        for (i, val) in line.split_whitespace().enumerate() {
            if ops[i] == b'+' {
                acc[i] += val.parse::<i64>().unwrap();
            } else {
                acc[i] *= val.parse::<i64>().unwrap();
            }
        }
    }
    acc.iter().sum()
}

fn part2(ops: &[u8], data_lines: &[&str]) -> i64 {
    let last_char = data_lines.iter().map(|line| line.len()).max().unwrap();
    let num_lines: Vec<&[u8]> = data_lines.iter().map(|l| l.as_bytes()).collect();

    // Get only the operators from ops (excluding spaces)
    let mut ops_iter = ops
        .iter()
        .filter_map(|&op| match op {
            b'+' | b'*' => Some(op),
            _ => None,
        })
        .rev();

    let mut total = 0;
    let mut acc: Option<i64> = None;
    let mut next_op = ops_iter.next();
    for i in (0..last_char).rev() {
        let mut num = 0;
        let mut has_char = false;
        // Get characters at position i for each line, and join them as a number.
        for c in num_lines.iter().filter_map(|line| match line.get(i) {
            Some(c) if c.is_ascii_digit() => Some(c),
            _ => None,
        }) {
            num = num * 10 + (c - b'0') as i64;
            has_char = true;
        }

        if has_char {
            // If the column is not empty, then add/multiply the number to accumulator
            acc = if next_op == Some(b'+') {
                Some(acc.unwrap_or(0) + num)
            } else {
                Some(acc.unwrap_or(1) * num)
            };
        } else {
            // If column is empty, then add accumulator to total and reset accumulator.
            next_op = ops_iter.next();
            if acc.is_some() {
                total += acc.unwrap();
                acc = None;
            }
        }
    }

    if acc.is_some() {
        total += acc.unwrap();
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
        let (ops, data_lines) = parse_input(input);
        assert_eq!(part1(&ops, &data_lines), 4277556);
        assert_eq!(part2(&ops, &data_lines), 3263827);
    }
}
