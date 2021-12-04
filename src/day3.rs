
fn part1(nbits : usize, lines : &[&str]) -> usize {
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
    
    let mut gamma = 0usize;
    for i in 0..nbits {
        if count[i] > 0 {
            gamma ^= (1<<(nbits-i-1));
        } else if count[i] == 0 {
            panic!("This shouldn't happen!");
        }
    }

    let mask : usize = (1 << nbits)-1;
    gamma*(!gamma & mask)
}


mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let lines : Vec<&str> = include_str!("../inputs/day3.example.txt").lines().collect();
        assert_eq!(part1(5, &lines), 198);
    }
}




fn main() {
    let lines : Vec<&str> = include_str!("../inputs/day3.txt").lines().collect();
    println!("Part 1: {}", part1(12,&lines));
}
