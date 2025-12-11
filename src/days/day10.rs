use std::{
    collections::{HashSet, VecDeque},
    fs, i32,
    time::Instant,
};

pub fn run() {
    let (input, t_input) = time!(fs::read_to_string("inputs/day10.txt").unwrap());
    let (parsed, t_parse) = time!(parse_input(&input));
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

fn parse_input(input: &str) -> Vec<MachineConfiguration> {
    input.lines().map(|line| line.into()).collect()
}

struct MachineConfiguration {
    required: u16,
    button_schematics: Vec<Vec<usize>>,
    button_schematics_bitmap: Vec<u16>,
    joltage: Vec<u16>,
}

impl From<&str> for MachineConfiguration {
    fn from(value: &str) -> Self {
        let mut parts = value.trim().split_whitespace();
        let required_str = parts.next().unwrap();
        let mut required: u16 = 0;
        for (i, &c) in required_str[1..required_str.len() - 1]
            .as_bytes()
            .iter()
            .enumerate()
        {
            if c == b'#' {
                required |= 1 << i;
            }
        }

        let remaining = parts.collect::<Vec<_>>();
        let button_schematics = remaining[..remaining.len() - 1]
            .iter()
            .map(|&b| {
                b[1..b.len() - 1]
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<usize>>>();

        let button_schematics_bitmap: Vec<u16> = button_schematics
            .iter()
            .map(|s| {
                let mut bitmap = 0;
                for button in s {
                    bitmap |= 1 << button;
                }
                bitmap
            })
            .collect();

        let last = remaining[remaining.len() - 1];
        let joltage = last[1..last.len() - 1]
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        MachineConfiguration {
            required,
            button_schematics,
            button_schematics_bitmap,
            joltage,
        }
    }
}

impl MachineConfiguration {
    fn least_clicks_for_required(
        &self,
        current_state: u16,
        current_path: u16,
        required_less_than: i32,
    ) -> i32 {
        if required_less_than <= 0 {
            return i32::MAX;
        }
        if current_state == self.required {
            return 0;
        }

        let mut min_steps = i32::MAX;
        let mut bit_index = 1;
        for schematic in &self.button_schematics_bitmap {
            if current_path & bit_index != 0 {
                bit_index <<= 1;
                continue;
            }
            let steps = self.least_clicks_for_required(
                current_state ^ schematic,
                current_path | bit_index,
                min_steps.min(required_less_than - 1),
            );
            min_steps = min_steps.min(steps.saturating_add(1));
            bit_index <<= 1;
        }

        min_steps
    }

    fn least_clicks_for_joltage_requirement(&self) -> i32 {
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();

        let button_sums: Vec<u32> = self
            .button_schematics
            .iter()
            .map(|s| s.len() as u32)
            .collect();
        let target_sum: u32 = self.joltage.iter().map(|&x| x as u32).sum();

        let start = vec![0; self.joltage.len()];
        visited.insert(start.clone());
        q.push_back((0, target_sum, start.clone()));

        while let Some((steps, remaining_sum, state)) = q.pop_front() {
            for (schematic, &b_sum) in self.button_schematics.iter().zip(&button_sums) {
                if remaining_sum < b_sum {
                    continue;
                }
                let mut next = state.clone();
                let mut is_exceeding = false;
                for &i in schematic {
                    next[i] += 1;
                    if next[i] > self.joltage[i] {
                        is_exceeding = true;
                        break;
                    }
                }
                if !is_exceeding {
                    if next == self.joltage {
                        return steps + 1;
                    }
                    if visited.insert(next.clone()) {
                        q.push_back((steps + 1, remaining_sum - b_sum, next));
                    }
                }
            }
        }

        0
    }
}

fn part1(config: &[MachineConfiguration]) -> i32 {
    config
        .iter()
        .map(|c| c.least_clicks_for_required(0, 0, i32::MAX))
        .sum()
}

fn part2(config: &[MachineConfiguration]) -> i32 {
    config
        .iter()
        .map(|c| {
            let res = c.least_clicks_for_joltage_requirement();
            println!("{}", res);
            res
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let parsed = parse_input(input);
        assert_eq!(part1(&parsed), 7);
        assert_eq!(part2(&parsed), 33);
    }
}
