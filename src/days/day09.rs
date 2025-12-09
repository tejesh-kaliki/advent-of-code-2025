use std::{fs, time::Instant};

pub fn run() {
    let (input, t_input) = time!(fs::read_to_string("inputs/day09.txt").unwrap());
    let (points, t_parse) = time!(parse_input(&input));
    let ((part1, part2), t_part1_and_2) = time!(solve_both_parts(&points));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    println!("Input time: {:?}", t_input);
    println!("Parse time: {:?}", t_parse);
    println!("Part 1 & 2 time: {:?}", t_part1_and_2);
    println!("Total time: {:?}", t_part1_and_2 + t_parse + t_input);
}

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    input
        .trim_end()
        .lines()
        .map(|line| line.split(',').map(|s| s.parse().unwrap()))
        .map(|mut line| (line.next().unwrap(), line.next().unwrap()))
        .collect()
}

fn is_inside_bounds(points: &[(i32, i32)], i: usize, j: usize) -> bool {
    let (x1, y1) = points[i];
    let (x2, y2) = points[j];
    let (min_x, max_x) = (x1.min(x2), x1.max(x2));
    let (min_y, max_y) = (y1.min(y2), y1.max(y2));

    for window in points[j..=j + points.len() / 2].windows(2) {
        if let Some(value) = is_line_in_bound(min_x, max_x, min_y, max_y, window) {
            return value;
        }
    }

    false
}

fn is_line_in_bound(
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    window: &[(i32, i32)],
) -> Option<bool> {
    let (px1, py1) = window[0];
    let (px2, py2) = window[1];

    // Horizontal line
    if px1 == px2 {
        let (py1, py2) = if py1 > py2 { (py2, py1) } else { (py1, py2) };
        if px1 > min_x
            && px1 < max_x
            && ((py1 <= min_y && py2 > min_y) || (py1 < max_y && py2 >= max_y))
        {
            return Some(true);
        }
    }
    // Vertical line
    else if py1 == py2 {
        let (px1, px2) = if px1 > px2 { (px2, px1) } else { (px1, px2) };
        if py1 > min_y
            && py1 < max_y
            && ((px1 <= min_x && px2 > min_x) || (px1 < max_x && px2 >= max_x))
        {
            return Some(true);
        }
    }
    None
}

fn solve_both_parts(points: &[(i32, i32)]) -> (i64, i64) {
    let mut max_area_with_rg_tiles = 0;
    let mut overall_max_area = 0;

    let doubled_points = points.iter().cloned().collect::<Vec<_>>().repeat(2);
    for i in 1..points.len() {
        let (x1, y1) = points[i];
        for j in 0..i {
            let (x2, y2) = points[j];
            let area = ((x2 - x1).abs() as i64 + 1) * ((y2 - y1).abs() as i64 + 1);
            overall_max_area = overall_max_area.max(area);

            if !is_inside_bounds(&doubled_points, i, j) {
                max_area_with_rg_tiles = max_area_with_rg_tiles.max(area);
            }
        }
    }

    (overall_max_area, max_area_with_rg_tiles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let points = parse_input(input);
        assert_eq!(solve_both_parts(&points), (50, 24));
    }

    #[test]
    fn test_valid_for_part2() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let points = parse_input(input);
        let points = points.clone().repeat(2);
        assert_eq!(is_inside_bounds(&points, 2, 0), true);
    }
}
