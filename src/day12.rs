use std::collections::HashMap;

// visited only contains the small caves
fn part1_impl<'a>(caves: &HashMap<&str,Vec<&'a str>>, cur: &'a str, small: &mut Vec<&'a str>, mut twice: bool) -> usize {
    let cur_neighbors = &caves[cur];

    // This small cave has already been visited
    if small.contains(&cur) {
        if twice {
            return 0;
        } else {
            twice = true;
        }
    }

    // We found a successful path through
    if cur == "end" {
        return 1;
    }

    // If is is lowercase, add to small vector, before processing
    // neighbors, so they know it is part of the current path
    let is_small = cur.chars().next().unwrap().is_ascii_lowercase();

    if is_small {
        small.push(&cur);
    }

    // Node hasn't been processed yet. Loop through neighbors and add up paths
    let mut count = 0;
    for c in cur_neighbors {
        if *c == "start" { // Don't ever return to start
            continue;
        }
        count += part1_impl(caves, c, small, twice);
    }

    // now pop self from the small list
    if is_small {
        small.pop();
    }

    return count;
}

fn part1(caves: &HashMap<&str,Vec<&str>>) -> usize {
    let mut small = Vec::new();
    let ans = part1_impl(caves, "start", &mut small, false);
    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
    let mut caves: HashMap<&str,Vec<&str>> = HashMap::new();
    let lines = include_str!("../inputs/day12.example.txt").lines();
    for l in lines {
        let mut s = l.split("-");
        let (a, b) = (s.next().unwrap(), s.next().unwrap());
        caves.entry(a).or_insert(vec![]).push(b);
        caves.entry(b).or_insert(vec![]).push(a);
    }
    //assert_eq!(10, part1(&caves));


    }
}

fn main() {
    let mut caves: HashMap<&str,Vec<&str>> = HashMap::new();
    let lines = include_str!("../inputs/day12.txt").lines();
    for l in lines {
        let mut s = l.split("-");
        let (a, b) = (s.next().unwrap(), s.next().unwrap());
        caves.entry(a).or_insert(vec![]).push(b);
        caves.entry(b).or_insert(vec![]).push(a);
    }

    println!("{}", part1(&caves));
}
