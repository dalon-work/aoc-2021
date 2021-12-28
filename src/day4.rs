type Board = [[i32; 5]; 5];

fn check_done(b: &Board, i: usize, j: usize) -> bool {
    let mut row_done = true;
    let mut col_done = true;
    for k in 0..5 {
        row_done = row_done && b[i][k] == -1;
        col_done = col_done && b[k][j] == -1;
    }
    return row_done || col_done;
}

fn sum(b: &Board) -> i32 {
    let mut s: i32 = 0;
    for i in 0..5 {
        for j in 0..5 {
            if b[i][j] != -1 {
                s += b[i][j];
            }
        }
    }
    return s;
}

fn part1(numbers: &[i32], boards: &[Board]) -> i32 {
    let mut boards = boards.to_vec();

    for n in numbers {
        for b in boards.iter_mut() {
            for i in 0..5 {
                for j in 0..5 {
                    if b[i][j] == *n {
                        b[i][j] = -1;
                        if check_done(&b, i, j) {
                            return n * sum(&b);
                        }
                    }
                }
            }
        }
    }
    return 0;
}

use std::collections::HashSet;

fn part2(numbers: &[i32], boards: &[Board]) -> i32 {
    let mut boards = boards.to_vec();
    let mut win = HashSet::new();
    let mut order = Vec::new();

    for n in numbers {
        if win.len() == boards.len() {
            break;
        }
        for (bidx, b) in boards.iter_mut().enumerate() {
            for i in 0..5 {
                for j in 0..5 {
                    if b[i][j] == *n {
                        b[i][j] = -1;
                        if check_done(&b, i, j) {
                            if !win.contains(&bidx) {
                                win.insert(bidx);
                                order.push((bidx, *n));
                            }
                        }
                    }
                }
            }
        }
    }

    let (last_board_idx, last_board_n) = order.last().unwrap();

    return last_board_n * sum(&boards[*last_board_idx]);
}

fn parse_board(lines: &mut std::str::Lines) -> Board {
    let mut r: Board = Default::default();

    for i in 0..5 {
        for (j, d) in lines.next().unwrap().split_whitespace().enumerate() {
            r[i][j] = d.parse().unwrap();
        }
    }
    return r;
}

mod tests {
    use super::*;

    #[test]
    fn test_day4() {
        let mut boards = Vec::<Board>::new();
        let mut lines = include_str!("../inputs/day4.example.txt").lines();
        let numbers: Vec<i32> = lines
            .next()
            .unwrap()
            .split(',')
            .map(|i| i.parse().unwrap())
            .collect();

        loop {
            match lines.next() {
                Some(_) => boards.push(parse_board(&mut lines)),
                None => break,
            }
        }
        assert_eq!(part1(&numbers, &boards), 4512);
        assert_eq!(part2(&numbers, &boards), 1924);
    }
}

fn main() {
    let mut boards = Vec::<Board>::new();
    let mut lines = include_str!("../inputs/day4.txt").lines();
    let numbers: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();

    loop {
        match lines.next() {
            Some(_) => boards.push(parse_board(&mut lines)),
            None => break,
        }
    }

    println!("Part 1 {}", part1(&numbers, &boards));
    println!("Part 2 {}", part2(&numbers, &boards));
}
