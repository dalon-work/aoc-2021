

fn main() {
    let depths : Vec<i32> = include_str!("../inputs/day1.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut count = 0;
    for w in depths.windows(2) {
        count += (w[1] > w[0]) as i32;
    }
    println!("{}",count);
}
