use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug,Copy, Clone, Eq, PartialEq)]
struct Unvisited {
    dist: usize,
    pos: (usize, usize)
}

impl Ord for Unvisited {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Unvisited {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(risk: &Vec<Vec<usize>>) -> usize {
    let (X,Y) = (risk[0].len(), risk.len());
    let mut visited = vec![vec![false;X]; Y];
    let (X,Y) = (X as i64, Y as i64);
    let pos = (0,0);
    let mut unvisited = BinaryHeap::new();
    unvisited.push(Unvisited{dist:0, pos:(0,0)});

    loop {
        let node = unvisited.pop().unwrap();

        let cur_dist = node.dist;
        let pos = node.pos;

        if visited[pos.0 as usize][pos.1 as usize] {
            continue;
        }

        if pos.0 == (X-1) as usize && pos.1 == (Y-1) as usize {
            return cur_dist;
        }

        for i in -1i64..=1 {
                if i == 0 {
                    continue;
                } 

                let nb = (pos.0 as i64 + i, pos.1 as i64);

                if nb.0 < 0 || nb.0 >= X {
                    continue;
                }

                let nb = (nb.0 as usize, nb.1 as usize);

                if !visited[nb.0][nb.1] {
                    let new_dist = cur_dist + risk[nb.0][nb.1];
                    unvisited.push(Unvisited{ dist: new_dist, pos: nb });
                }
        }

        for j in -1i64..=1 {
                if j == 0 {
                    continue;
                } 

                let nb = (pos.0 as i64, pos.1 as i64+j);

                if nb.1 < 0 || nb.1 >= Y {
                    continue;
                }

                let nb = (nb.0 as usize, nb.1 as usize);

                if !visited[nb.0][nb.1] {
                    let new_dist = cur_dist + risk[nb.0][nb.1];
                    unvisited.push(Unvisited{ dist: new_dist, pos: nb });
                }
        }

        visited[pos.0][pos.1] = true;
    }
}


fn main() {
    let risk: Vec<Vec<usize>> = include_str!("../inputs/day15.txt").lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>()).collect();
    println!("{}", part1(&risk));
}
