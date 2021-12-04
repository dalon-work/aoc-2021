fn main() {
    let mut x = 0;
    let mut y = 0;

    for line in include_str!("../inputs/day2.txt").lines() {
        let mut it = line.split_whitespace();
        let dir : &str = it.next().unwrap();
        let c : isize = it.next().unwrap().parse().unwrap();
        match dir {
            "forward" => x += c,
            "down"    => y += c,
            "up"      => y -= c,
            _ => panic!("Bad direction!")
        }
    }
    println!("(x,y) m: ({},{}) {}",x,y, x*y);
}
