use std::collections::HashSet;

fn main() {
    let mut lines = include_str!("../inputs/day8.txt").lines();
    let mut count = 0;
    for l in lines {
        let mut l = l.split(" | ");
        let input = l.next().unwrap();
        let output =  l.next().unwrap();
        for word in output.split_whitespace() {
            let hs: HashSet<char> = word.chars().collect();
            match hs.len() {
                2 | 4 | 3 | 7 => count += 1,
                _ => {}
            }
        }
    }
    println!("Part 1 {}", count);
}
