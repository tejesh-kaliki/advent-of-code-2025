use std::{cmp::Reverse, collections::BinaryHeap, fs, time::Instant};

pub fn run() {
    let (input, t_input) = time!(fs::read_to_string("inputs/day08.txt").unwrap());
    let ((mut pairs_heap, points), t_parse) = time!(parse_input(&input));
    let ((part1, part2), t_part1_and_2) = time!(solve_both_parts(&mut pairs_heap, &points, 1000));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    println!("Input time: {:?}", t_input);
    println!("Parse time: {:?}", t_parse);
    println!("Part 1 & 2 time: {:?}", t_part1_and_2);
    println!("Total time: {:?}", t_part1_and_2 + t_parse + t_input);
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            let (smaller, larger) = if self.size[root_x] < self.size[root_y] {
                (root_x, root_y)
            } else {
                (root_y, root_x)
            };
            self.parent[smaller] = larger;
            self.size[larger] += self.size[smaller];
        }
    }
}

fn parse_input(input: &str) -> (BinaryHeap<(Reverse<i64>, usize, usize)>, Vec<[i64; 3]>) {
    let points: Vec<[i64; 3]> = input
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
        .collect::<Vec<_>>();

    let mut pairs_heap = BinaryHeap::with_capacity(points.len() * (points.len() - 1) / 2);

    for (i, &p1) in points.iter().enumerate() {
        if i == 0 {
            continue;
        }
        for (j, &p2) in points.iter().take(i).enumerate() {
            let dx = p1[0] - p2[0];
            let dy = p1[1] - p2[1];
            let dz = p1[2] - p2[2];
            let dist = dx * dx + dy * dy + dz * dz;
            pairs_heap.push((Reverse(dist), i, j));
        }
    }

    (pairs_heap, points)
}

fn solve_both_parts(
    pairs_heap: &mut BinaryHeap<(Reverse<i64>, usize, usize)>,
    points: &[[i64; 3]],
    n: usize,
) -> (i64, i64) {
    let mut dsu = DSU::new(points.len());
    let mut ids = vec![false; points.len()];
    let mut seen = 0;
    let mut iteration = 0;
    while let Some((_, i, j)) = pairs_heap.pop() {
        if !ids[i] {
            seen += 1;
            ids[i] = true;
        }
        if !ids[j] {
            seen += 1;
            ids[j] = true;
        }

        if seen == points.len() {
            let mut counts = dsu
                .parent
                .iter()
                .enumerate()
                .filter(|&(i, &p)| i == p)
                .map(|(i, _)| dsu.size[i] as i64)
                .collect::<Vec<_>>();

            counts.select_nth_unstable_by(2, |a, b| b.cmp(a));
            let part1_result = counts[..3].iter().product::<i64>();
            let part2_result = points[i][0] * points[j][0];
            return (part1_result, part2_result);
        }

        // For part 1
        if iteration < n {
            dsu.union(i, j);
            iteration += 1;
        }
    }

    unreachable!("heap exhausted before covering all points")
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
        let (mut pairs_heap, points) = parse_input(input);
        assert_eq!(solve_both_parts(&mut pairs_heap, &points, 10), (40, 25272));
    }
}
