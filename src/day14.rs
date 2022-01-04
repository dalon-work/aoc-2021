use std::collections::HashMap;

type Pair = (char,char);
type Letter = char;

fn parse(s: &str) -> (HashMap<Pair,usize>, HashMap<Letter,usize>, Vec<(Pair,char)>) {
    let mut lines = s.lines();
    let c: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();

    let mut pair_map = HashMap::<Pair,usize>::new();
    let mut letter_map = HashMap::<Letter,usize>::new();

    let inst: Vec<(Pair,char)> = lines.map(|l| {
        let mut s = l.split(" -> ").collect::<Vec<&str>>();
        let pair = s[0].chars().collect::<Vec<char>>();
        let c = s[1].chars().next().unwrap();
        ((pair[0],pair[1]),c)
    })
    .collect();

    for c in &c {
        *letter_map.entry(*c).or_insert(0) += 1;
    }

    for p in c.windows(2) {
        let p = p.to_owned();
        *pair_map.entry((p[0],p[1])).or_insert(0) += 1;
    }

    return (pair_map, letter_map, inst);
}

fn part1(pair_map: HashMap<Pair, usize>, mut letter_map: HashMap<Letter, usize>, inst: Vec<(Pair, char)>) -> usize {

    let mut old_pairs = pair_map.clone();

    for _ in (0..40) {
        let mut new_pairs = HashMap::new();
        for (pair,new_letter) in &inst {
            let old_count = *old_pairs.entry( *pair ).or_insert(0);
            *new_pairs.entry( (pair.0, *new_letter) ).or_insert(0) += old_count;
            *new_pairs.entry( (*new_letter, pair.1) ).or_insert(0) += old_count;
            *letter_map.entry( *new_letter ).or_insert(0) += old_count;
        }
        old_pairs = new_pairs;
    }

    let mut max = usize::MIN;
    let mut min = usize::MAX;

    for (k,v) in letter_map {
        max = v.max(max);
        min = v.min(min);
    }

    return max - min;

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14() {
        let s = include_str!("../inputs/day14.example.txt");
        let (pair_map, letter_map, inst) = parse(s);
        assert_eq!(1588, part1(pair_map, letter_map, inst));

    }
}

fn main() {
    let s = include_str!("../inputs/day14.txt");
    let (pair_map, letter_map, inst) = parse(s);
    println!("{}",part1(pair_map, letter_map, inst));
}
