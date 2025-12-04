use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day04.txt").expect("Must read the file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn can_be_removed(grid: &[Vec<bool>], x: i32, y: i32) -> bool {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut surrounding_count = 0;
    // Check in 8 adjacent cells
    for dx in -1i32..=1 {
        for dy in -1i32..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let px = x + dx;
            let py = y + dy;
            if px >= 0 && px < cols as i32 && py >= 0 && py < rows as i32 {
                surrounding_count += grid[py as usize][px as usize] as i32;
            }
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
    for y in 0i32..rows as i32 {
        for x in 0i32..cols as i32 {
            if !grid[y as usize][x as usize] {
                continue;
            }

            if can_be_removed(&grid, x, y) {
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
        let mut can_remove: Vec<(usize, usize)> = Vec::new();
        for y in 0i32..rows as i32 {
            for x in 0i32..cols as i32 {
                if !grid[y as usize][x as usize] {
                    continue;
                }

                if can_be_removed(&grid, x, y) {
                    can_remove.push((x as usize, y as usize));
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
