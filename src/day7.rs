fn part1(crabs: &[i64], pos: i64) -> i64 {
    let mut fuel = 0;
    for c in crabs {
        fuel += (c - pos).abs();
    }
    return fuel;
}

fn part2(crabs: &[i64], pos: i64) -> i64 {
    let mut fuel = 0;
    for c in crabs {
        let n = (c - pos).abs();
        fuel += n * (n + 1) / 2
    }
    return fuel;
}

fn run(crabs: &[i64], cost: fn(&[i64], i64) -> i64) -> i64 {
    let mut m = i64::MAX;
    for pos in crabs[0]..=crabs[crabs.len() - 1] {
        m = std::cmp::min(m, cost(crabs, pos));
    }
    return m;
}

fn main() {
    let mut crabs: Vec<i64> = include_str!("../inputs/day7.txt")
        .trim()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    crabs.sort();
    println!("{}", run(&crabs, part1));
    println!("{}", run(&crabs, part2));
}
