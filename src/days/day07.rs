use std::{fs, time::Instant};

pub fn run() {
    let (input, t_input) = time!(fs::read_to_string("inputs/day07.txt").unwrap());
    let (parsed, t_parse) = time!(parse_input(&input));
    let ((part1, part2), t_part1_and_2) = time!(solve_both_parts(&parsed));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    println!("Input time: {:?}", t_input);
    println!("Parse time: {:?}", t_parse);
    println!("Part 1 & 2 time: {:?}", t_part1_and_2);
    println!("Total time: {:?}", t_part1_and_2 + t_parse + t_input);
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<u8>,
    start: usize,
}

fn parse_input(input: &str) -> Grid {
    let lines = input
        .trim()
        .lines()
        .skip_while(|line| !line.contains('S')) // It should be in first row, but just to be sure.
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let height = lines.len();
    let width = lines[0].len();

    let mut cells = Vec::with_capacity(width * height);
    let start = lines[0].iter().position(|&cell| cell == b'S').unwrap();
    for line in lines {
        cells.extend(line);
    }

    Grid {
        width,
        height,
        cells,
        start,
    }
}

fn solve_both_parts(grid: &Grid) -> (i64, i64) {
    let mut current_row_state: Vec<i64> = vec![0; grid.width];
    let mut next_row_state = vec![0; grid.width];
    current_row_state[grid.start] = 1;

    let mut split_count = 0;
    let w = grid.width;
    for y in 1..grid.height {
        next_row_state.fill(0);
        for x in 0..w {
            let state = current_row_state[x];
            if state == 0 {
                continue;
            }

            if grid.cells[y * w + x] == b'^' {
                split_count += 1;
                if x > 0 {
                    next_row_state[x - 1] += state;
                }
                if x + 1 < w {
                    next_row_state[x + 1] += state;
                }
            } else {
                next_row_state[x] += state;
            }
        }

        std::mem::swap(&mut current_row_state, &mut next_row_state);
    }

    (split_count, current_row_state.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            .trim();
        let grid = parse_input(input);
        assert_eq!(solve_both_parts(&grid), (21, 40));
    }
}
