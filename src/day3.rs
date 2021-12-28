fn part1(nbits: usize, lines: &[&str]) -> usize {
    let mut count = vec![0; nbits];

    for l in lines {
        for (i, c) in l.chars().enumerate() {
            match c {
                '1' => count[i] += 1,
                '0' => count[i] -= 1,
                _ => unreachable!(),
            }
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for i in 0..nbits {
        if count[i] > 0 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
}

fn part2(nbits: usize, lines: &[&str], comp: fn(usize, usize) -> bool) -> usize {
    let mut cur = lines.to_vec();
    let mut bucket0 = Vec::<&str>::new();
    let mut bucket1 = Vec::<&str>::new();

    for i in 0..nbits {
        bucket0.clear();
        bucket1.clear();

        for l in &cur {
            if l.as_bytes()[i] == '1' as u8 {
                bucket1.push(l);
            } else {
                bucket0.push(l);
            }
        }

        if comp(bucket0.len(), bucket1.len()) {
            std::mem::swap(&mut bucket0, &mut cur);
        } else {
            std::mem::swap(&mut bucket1, &mut cur);
        }

        if cur.len() == 1 {
            break;
        }
    }
    assert!(cur.len() == 1);
    return usize::from_str_radix(cur[0], 2).unwrap();
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines: Vec<&str> = include_str!("../inputs/day3.example.txt").lines().collect();
        assert_eq!(part1(5, &lines), 198);
    }

    #[test]
    fn test_part2() {
        let lines: Vec<&str> = include_str!("../inputs/day3.example.txt").lines().collect();
        assert_eq!(part2(5, &lines, |l0, l1| l0 > l1), 23);
        assert_eq!(part2(5, &lines, |l0, l1| l0 <= l1), 10);
    }
}

fn main() {
    let lines: Vec<&str> = include_str!("../inputs/day3.txt").lines().collect();
    println!("Part 1: {}", part1(12, &lines));
    println!(
        "Part 2: {}",
        part2(12, &lines, |l0, l1| l0 > l1) * part2(12, &lines, |l0, l1| l0 <= l1)
    );
}
