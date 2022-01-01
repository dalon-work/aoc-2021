use std::collections::HashSet;

type Paper = HashSet<(usize, usize)>;
type Fold  = Vec::<(char,usize)>;

fn part1(dots: &Paper, dir: char, fold: usize)-> Paper {
    let mut new_dots: Paper = HashSet::new();
    if dir == 'x' {
        for d in dots {
            if d.0 > fold {
                new_dots.insert( (fold-(d.0-fold), d.1));
            } else {
                new_dots.insert( *d );
            }
        }
    } else {
        for d in dots {
            if d.1 > fold {
                new_dots.insert( (d.0, fold-(d.1-fold)) );
            } else {
                new_dots.insert( *d );
            }
        }
    }
    return new_dots;
}

fn dot_max(dots: &Paper) -> (usize, usize) {
    return dots.iter().fold( (0,0), |d, m| (std::cmp::max(d.0,m.0), std::cmp::max(d.1,m.1)) );
}

fn print_dots(dots: &Paper) {
    let max = dot_max(dots);
    let mut paper = vec![' '; (max.0+1)*(max.1+1)];

    for d in dots {
        paper[ d.1*(max.0+1) + d.0 ] = '#'
    }

    for j in 0..=(max.1) {
        assert_eq!(((j*max.0)..((j+1)*max.0+1)).len(), max.0+1);
        let row = paper[(j*(max.0+1))..((j+1)*(max.0+1))].iter().collect::<String>();
        println!("{}",row);
    }

}

fn parse(s: &str) -> (Paper, Fold) {
    let mut dots = Paper::new();
    let mut lines = s.lines();
    loop {
        let l = lines.next().unwrap();
        if l.is_empty() {
            break;
        }
        let mut s = l.split(",");
        dots.insert(( s.next().unwrap().parse().unwrap(), s.next().unwrap().parse().unwrap()));
    }
    let mut fold = Fold::new();

    for l in lines {
        let mut s = l.split("=");
        fold.push( (s.next().unwrap().chars().last().unwrap(),
                    s.next().unwrap().parse().unwrap()) );
    }

    return (dots, fold);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13() {
        let (mut dots, fold) = parse(include_str!("../inputs/day13.example.txt"));
        print_dots(&dots);
        for f in fold {
            dots = part1(&dots, f.0, f.1);
            print_dots(&dots);
        }
    }
}


fn main() {

    let (mut dots, fold) = parse(include_str!("../inputs/day13.txt"));

    println!("{:?}", part1(&dots, fold[0].0, fold[0].1).len());

    for f in fold {
        dots = part1(&dots, f.0, f.1);
    }
    print_dots(&dots);
}
