fn main() {
    let dirs: Vec<(&str, i32)> = include_str!("../inputs/day2.txt")
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();
            (it.next().unwrap(), it.next().unwrap().parse().unwrap())
        })
        .collect();

    let mut x = 0;
    let mut y = 0;
    for d in &dirs {
        match d.0 {
            "forward" => x += d.1,
            "down" => y += d.1,
            "up" => y -= d.1,
            _ => unreachable!(),
        }
    }
    println!("Part 1 {}", x * y);

    x = 0;
    y = 0;
    let mut a = 0;

    for d in &dirs {
        match d.0 {
            "forward" => {
                x += d.1;
                y += a * d.1;
            }
            "down" => a += d.1,
            "up" => a -= d.1,
            _ => unreachable!(),
        }
    }
    println!("Part 2 {}", x * y);
}
