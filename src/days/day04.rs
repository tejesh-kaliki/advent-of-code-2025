use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day04.txt").expect("Must read the file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
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

fn can_be_removed(grid: &[Vec<bool>], rows: usize, cols: usize, x: usize, y: usize) -> bool {
    let mut surrounding_count = 0;
    let x = x as isize;
    let y = y as isize;

    for (dx, dy) in NEIGHBOURS {
        let px = x + dx;
        let py = y + dy;
        if px >= 0 && px < cols as isize && py >= 0 && py < rows as isize {
            surrounding_count += grid[py as usize][px as usize] as i32;
        }
    }

    surrounding_count < 4
}

fn part1(input: &str) -> i64 {
    let grid: Vec<Vec<bool>> = input
        .trim()
        .lines()
        .map(|l| l.as_bytes().iter().map(|c| *c == b'@').collect())
        .collect();

    let (rows, cols) = (grid.len(), grid[0].len());

    let mut count = 0;
    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] && can_be_removed(&grid, rows, cols, x, y) {
                count += 1;
            }
        }
    }

    count
}

fn part2(input: &str) -> i64 {
    let mut grid: Vec<Vec<bool>> = input
        .trim()
        .lines()
        .map(|l| l.as_bytes().iter().map(|c| *c == b'@').collect())
        .collect();

    let (rows, cols) = (grid.len(), grid[0].len());

    let mut total_count = 0;
    loop {
        let mut can_remove = Vec::new();
        for y in 0..rows {
            for x in 0..cols {
                if grid[y][x] && can_be_removed(&grid, rows, cols, x, y) {
                    can_remove.push((x, y));
                    total_count += 1;
                }
            }
        }

        if can_remove.is_empty() {
            break;
        }

        for (x, y) in can_remove {
            grid[y][x] = false;
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
        assert_eq!(part1(input), 13);
        assert_eq!(part2(input), 43);
    }
}
