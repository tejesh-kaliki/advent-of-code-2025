use std::{fs, i8, time::Instant};

pub fn run() {
    let (input, t_input) = time!(fs::read_to_string("inputs/day04.txt").unwrap());
    let (mut parsed, t_parse) = time!(parse_input(&input));
    let (part1, t_part1) = time!(part1(&mut parsed));
    let (part2, t_part2) = time!(part2(&mut parsed));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    println!("Input time: {:?}", t_input);
    println!("Parse time: {:?}", t_parse);
    println!("Part 1 time: {:?}", t_part1);
    println!("Part 2 time: {:?}", t_part2);
    println!("Total time: {:?}", t_part2 + t_part1 + t_input + t_parse);
}

fn parse_input(input: &str) -> Vec<Vec<i8>> {
    let grid: Vec<Vec<bool>> = input
        .trim()
        .lines()
        .map(|l| l.as_bytes().iter().map(|c| *c == b'@').collect())
        .collect();

    let mut neighbours = vec![vec![i8::MAX; grid[0].len()]; grid.len()];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] {
                neighbours[y][x] = count_neighbours(&grid, grid.len(), grid[0].len(), x, y);
            }
        }
    }

    neighbours
}

static NEIGHBOURS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn count_neighbours(grid: &[Vec<bool>], rows: usize, cols: usize, x: usize, y: usize) -> i8 {
    let mut surrounding_count = 0;
    let x = x as isize;
    let y = y as isize;

    for (dx, dy) in NEIGHBOURS {
        let px = x + dx;
        let py = y + dy;
        if px >= 0 && px < cols as isize && py >= 0 && py < rows as isize {
            surrounding_count += grid[py as usize][px as usize] as i8;
        }
    }

    surrounding_count
}

fn part1(grid: &[Vec<i8>]) -> i64 {
    let (rows, cols) = (grid.len(), grid[0].len());

    let mut count = 0;
    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] < 4 {
                count += 1;
            }
        }
    }

    count
}

fn part2(grid: &mut [Vec<i8>]) -> i64 {
    let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);

    let mut total_count = 0;
    loop {
        let mut can_remove = Vec::new();
        for y in 0..rows {
            for x in 0..cols {
                if grid[y as usize][x as usize] < 4 {
                    can_remove.push((x, y));
                    total_count += 1;
                    for (dx, dy) in NEIGHBOURS {
                        let px = x + dx;
                        let py = y + dy;
                        if px >= 0 && px < cols as isize && py >= 0 && py < rows as isize {
                            grid[py as usize][px as usize] -= 1;
                        }
                    }
                }
            }
        }

        if can_remove.is_empty() {
            break;
        }

        for (x, y) in can_remove {
            grid[y as usize][x as usize] = i8::MAX;
        }
    }

    total_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let mut parsed = parse_input(input);
        assert_eq!(part1(&parsed), 13);
        assert_eq!(part2(&mut parsed), 43);
    }
}
