use std::{fs, time::Instant};

pub fn run() {
    let (input, t_input) = time!(fs::read_to_string("inputs/day07.txt").unwrap());
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

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
    start: (usize, usize),
}

fn parse_input(input: &str) -> Grid {
    let lines = input
        .trim()
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut cells = Vec::with_capacity(lines.len());
    let mut start = (0, 0);
    for (y, line) in lines.iter().enumerate() {
        let mut row = Vec::with_capacity(line.len());
        for (x, &cell) in line.iter().enumerate() {
            row.push(cell == b'^');
            if cell == b'S' {
                start = (x, y);
            }
        }
        cells.push(row);
    }

    Grid {
        width: cells[0].len(),
        height: cells.len(),
        cells,
        start,
    }
}

fn part1(grid: &Grid) -> i64 {
    let mut current_row_state: Vec<bool> = vec![false; grid.width];
    current_row_state[grid.start.0] = true;

    let mut count = 0;
    for y in grid.start.1 + 1..grid.height {
        let row = &grid.cells[y];
        let mut new_row_state = vec![false; grid.width];
        for (x, (&state, &cell)) in current_row_state.iter().zip(row).enumerate() {
            if state && cell {
                count += 1;
                if x > 0 {
                    new_row_state[x - 1] = true;
                }
                if x < grid.width - 1 {
                    new_row_state[x + 1] = true;
                }
            } else if state {
                new_row_state[x] = true;
            }
        }

        current_row_state = new_row_state;
    }

    count
}

fn part2(grid: &Grid) -> i64 {
    let mut current_row_state: Vec<i64> = vec![0; grid.width];
    current_row_state[grid.start.0] = 1;

    for y in grid.start.1 + 1..grid.height {
        let row = &grid.cells[y];
        let mut new_row_state = vec![0; grid.width];
        for (x, (&state, &cell)) in current_row_state.iter().zip(row).enumerate() {
            if state > 0 && cell {
                if x > 0 {
                    new_row_state[x - 1] += state;
                }
                if x < grid.width - 1 {
                    new_row_state[x + 1] += state;
                }
            } else if state > 0 {
                new_row_state[x] += state;
            }
        }

        current_row_state = new_row_state;
    }

    current_row_state.iter().sum()
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
        assert_eq!(part1(&grid), 21);
        assert_eq!(part2(&grid), 40);
    }
}
