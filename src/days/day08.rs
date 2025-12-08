use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs,
    time::Instant,
};

pub fn run() {
    let (input, t_input) = time!(fs::read_to_string("inputs/day08.txt").unwrap());
    let (points, t_parse) = time!(parse_input(&input));
    let (part1, t_part1) = time!(part1(&points, 1000));
    let (part2, t_part2) = time!(part2(&points));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    println!("Input time: {:?}", t_input);
    println!("Parse time: {:?}", t_parse);
    println!("Part 1 time: {:?}", t_part1);
    println!("Part 2 time: {:?}", t_part2);
    println!("Total time: {:?}", t_part2 + t_part1 + t_parse + t_input);
}

fn parse_input(input: &str) -> Vec<[i64; 3]> {
    input
        .trim_end()
        .lines()
        .map(|line| line.splitn(3, ',').map(|s| s.parse().unwrap()))
        .map(|mut iter| {
            [
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ]
        })
        .collect::<Vec<_>>()
}

fn part1(points: &[[i64; 3]], n: usize) -> i64 {
    let mut pairs_heap: BinaryHeap<(Reverse<i128>, usize, usize)> = BinaryHeap::new();

    for (i, p1) in points.iter().enumerate() {
        if i == 0 {
            continue;
        }
        for (j, p2) in points.iter().take(i).enumerate() {
            let (dx, dy, dz) = (
                (p1[0] - p2[0]) as i128,
                (p1[1] - p2[1]) as i128,
                (p1[2] - p2[2]) as i128,
            );
            let dist = dx * dx + dy * dy + dz * dz;
            pairs_heap.push((Reverse(dist), i, j));
        }
    }

    let mut groups: Vec<HashSet<usize>> = Vec::new();
    for _ in 0..n {
        let (_, i, j) = pairs_heap.pop().unwrap();

        let i_idx = groups.iter().position(|group| group.contains(&i));
        let j_idx = groups.iter().position(|group| group.contains(&j));

        if let (Some(i_idx), Some(j_idx)) = (i_idx, j_idx) {
            if i_idx != j_idx {
                let cloned = groups[j_idx].clone();
                groups[i_idx].extend(cloned);
                groups.remove(j_idx);
            }
        } else if let Some(i_idx) = i_idx {
            groups[i_idx].insert(j);
        } else if let Some(j_idx) = j_idx {
            groups[j_idx].insert(i);
        } else {
            let mut group = HashSet::new();
            group.insert(i);
            group.insert(j);
            groups.push(group);
        }
    }

    let mut counts = groups
        .iter()
        .map(|group| group.len() as i64)
        .collect::<Vec<_>>();
    counts.sort_by_key(|x| Reverse(*x));
    counts.iter().take(3).product()
}

fn part2(points: &[[i64; 3]]) -> i64 {
    let mut pairs_heap: BinaryHeap<(Reverse<i128>, usize, usize)> = BinaryHeap::new();

    for (i, p1) in points.iter().enumerate() {
        if i == 0 {
            continue;
        }
        for (j, p2) in points.iter().take(i).enumerate() {
            let (dx, dy, dz) = (
                (p1[0] - p2[0]) as i128,
                (p1[1] - p2[1]) as i128,
                (p1[2] - p2[2]) as i128,
            );
            let dist = dx * dx + dy * dy + dz * dz;
            pairs_heap.push((Reverse(dist), i, j));
        }
    }

    let mut groups: Vec<HashSet<usize>> = Vec::new();
    loop {
        let (_, i, j) = pairs_heap.pop().unwrap();

        let i_idx = groups.iter().position(|group| group.contains(&i));
        let j_idx = groups.iter().position(|group| group.contains(&j));

        if let (Some(i_idx), Some(j_idx)) = (i_idx, j_idx) {
            if i_idx != j_idx {
                let cloned = groups[j_idx].clone();
                groups[i_idx].extend(cloned);
                groups.remove(j_idx);
            }
        } else if let Some(i_idx) = i_idx {
            groups[i_idx].insert(j);
        } else if let Some(j_idx) = j_idx {
            groups[j_idx].insert(i);
        } else {
            let mut group = HashSet::new();
            group.insert(i);
            group.insert(j);
            groups.push(group);
        }

        if groups.len() == 1 && groups[0].len() == points.len() {
            return points[i][0] * points[j][0];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let points = parse_input(input);
        assert_eq!(part1(&points, 10), 40);
        assert_eq!(part2(&points), 25272);
    }
}
