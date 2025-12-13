use std::{fs, time::Instant};

pub fn run() {
    let (input, t_input) = time!(fs::read_to_string("inputs/day12.txt").unwrap());
    let (parsed, t_parse) = time!(parse_input(&input));
    let (part1, t_part1) = time!(part1(&parsed));

    println!("Part 1: {}", part1);

    println!("Input time: {:?}", t_input);
    println!("Parse time: {:?}", t_parse);
    println!("Part 1 time: {:?}", t_part1);
    println!("Total time: {:?}", t_part1 + t_parse + t_input);
}
#[derive(Debug)]
struct Input {
    patterns: Vec<Pattern>,
    rows: Vec<RowData>,
}

#[derive(Debug)]
struct Pattern {
    grid: [[bool; 3]; 3],
}

#[derive(Debug)]
struct RowData {
    width: usize,
    height: usize,
    values: Vec<i32>, // length = patterns.len()
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines().peekable();

    // ---- parse patterns ----
    let mut patterns = Vec::new();

    while let Some(line) = lines.peek() {
        if line.contains('x') {
            break;
        }

        // skip "N:"
        lines.next();

        let mut grid = [[false; 3]; 3];
        for y in 0..3 {
            let row = lines.next().unwrap().as_bytes();
            for x in 0..3 {
                grid[y][x] = row[x] == b'#';
            }
        }

        patterns.push(Pattern { grid });

        while let Some(line) = lines.peek() {
            if line.is_empty() {
                lines.next();
            } else {
                break;
            }
        }
    }

    // ---- parse rows ----
    let mut rows = Vec::new();

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        let (dim, rest) = line.split_once(':').unwrap();
        let (w, h) = dim.split_once('x').unwrap();

        rows.push(RowData {
            width: w.parse().unwrap(),
            height: h.parse().unwrap(),
            values: rest
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect(),
        });
    }

    Input { patterns, rows }
}

fn part1(input: &Input) -> i32 {
    let shape_sizes: Vec<i32> = input
        .patterns
        .iter()
        .map(|pattern| {
            pattern
                .grid
                .iter()
                .map(|row| row.iter().map(|&value| value as i32).sum::<i32>())
                .sum()
        })
        .collect::<Vec<_>>();

    input
        .rows
        .iter()
        .map(|row| {
            let total_shapes: i32 = row
                .values
                .iter()
                .zip(&shape_sizes)
                .map(|(num, size)| num * size)
                .sum();
            let grid_size = (row.width * row.height) as i32;
            (grid_size > total_shapes) as i32
        })
        .sum()
}
