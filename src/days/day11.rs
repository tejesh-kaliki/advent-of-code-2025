use std::{
    collections::{HashMap, VecDeque},
    fs,
    time::Instant,
};

pub fn run() {
    let (input, t_input) = time!(fs::read_to_string("inputs/day11.txt").unwrap());
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

struct Input<'a> {
    devices: HashMap<&'a str, Vec<&'a str>>,
    topo_order: Vec<&'a str>,
}

fn parse_input(input: &str) -> Input<'_> {
    let mut devices = HashMap::with_capacity(input.lines().count());
    for line in input.lines() {
        let mut parts = line.split(": ");
        let name = parts.next().unwrap();
        let edges: Vec<&str> = parts.next().unwrap().split_whitespace().collect();
        devices.insert(name, edges);
    }

    // Compute indegrees for topo sort
    let mut indeg = HashMap::<&str, usize>::new();
    for (&u, outs) in devices.iter() {
        indeg.entry(u).or_insert(0);
        for &v in outs {
            *indeg.entry(v).or_insert(0) += 1;
        }
    }

    // Topo sort (Kahn)
    let mut q = VecDeque::new();
    for (&node, &d) in indeg.iter() {
        if d == 0 {
            q.push_back(node);
        }
    }

    let mut topo = Vec::new();
    while let Some(u) = q.pop_front() {
        topo.push(u);
        if let Some(neighs) = devices.get(u) {
            for &v in neighs {
                let e = indeg.get_mut(v).unwrap();
                *e -= 1;
                if *e == 0 {
                    q.push_back(v);
                }
            }
        }
    }

    Input {
        devices,
        topo_order: topo,
    }
}

fn part1(input: &Input) -> i64 {
    count_paths_from(&"you", input)["out"]
}

// Count number of distinct paths from `start` to all reachable nodes in a DAG.
fn count_paths_from<'a>(start: &'a str, input: &'a Input) -> HashMap<&'a str, i64> {
    let mut ways = HashMap::<&str, i64>::new();
    ways.insert(start, 1);

    for &u in &input.topo_order {
        let wu = ways.get(u).copied().unwrap_or(0);
        if wu == 0 {
            continue;
        }
        if let Some(neighs) = input.devices.get(u) {
            for &v in neighs {
                *ways.entry(v).or_insert(0) += wu;
            }
        }
    }

    ways
}

fn part2(input: &Input) -> i64 {
    // 1. Count paths from svr
    let from_svr = count_paths_from("svr", input);
    let ways_to_fft = from_svr.get("fft").copied().unwrap_or(0);
    let ways_to_dac = from_svr.get("dac").copied().unwrap_or(0);

    // 2. Count paths starting at fft
    let from_fft = count_paths_from("fft", input);
    let ways_fft_to_dac = from_fft.get("dac").copied().unwrap_or(0);
    let ways_fft_to_out = from_fft.get("out").copied().unwrap_or(0);

    // 3. Count paths starting at dac
    let from_dac = count_paths_from("dac", input);
    let ways_dac_to_fft = from_dac.get("fft").copied().unwrap_or(0); // should be 0 by assumption
    let ways_dac_to_out = from_dac.get("out").copied().unwrap_or(0);

    ways_to_fft * ways_fft_to_dac * ways_dac_to_out
        + ways_to_dac * ways_dac_to_fft * ways_fft_to_out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let parsed = parse_input(input);
        assert_eq!(part1(&parsed), 5);

        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let parsed = parse_input(input);
        assert_eq!(part2(&parsed), 2);
    }
}
