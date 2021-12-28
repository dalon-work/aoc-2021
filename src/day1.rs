fn main() {
    let depths: Vec<i32> = include_str!("../inputs/day1.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let part1: i32 = depths.windows(2).map(|w| (w[1] > w[0]) as i32).sum();
    println!("{}", part1);
    let part2: i32 = depths.windows(4).map(|w| (w[3] > w[0]) as i32).sum();
    println!("{}", part2);
}
