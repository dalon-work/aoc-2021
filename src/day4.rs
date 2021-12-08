fn check_done(b: &[[i32;5];5], i: usize, j: usize) -> bool {
    let mut row_done = true;
    let mut col_done = true;
    for k in 0..5 {
        row_done = row_done && b[i][k] == -1;
        col_done = col_done && b[k][j] == -1;
    }
    return row_done || col_done;
}

fn sum(b: &[[i32;5];5]) -> i32 {
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

fn part1(numbers: &[i32], boards: &[ [[i32;5];5] ]) -> i32 {
    let mut boards = boards.to_vec();

    for n in numbers {
        for b in boards.iter_mut() {
            for i in 0..5 {
                for j in 0..5 {
                    if b[i][j] == *n {
                        b[i][j] = -1;
                        if check_done(&b, i, j) {
                            return n*sum(&b);
                        }
                    }
                }
            }
        }
    }
    return 0;
}


fn parse_board(lines: &mut std::str::Lines) -> [[i32;5];5] {
    let mut r: [[i32;5];5] = Default::default();

    for i in 0..5 {
        for (j, d) in lines.next().unwrap().split_whitespace().enumerate() {
            r[i][j] = d.parse().unwrap();
        }
    }
    return r
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
    let mut boards = Vec::<[[i32;5];5]>::new();
    let mut lines = include_str!("../inputs/day4.example.txt").lines();
    let numbers: Vec<i32> = lines.next().unwrap()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();

    loop {
        match lines.next() {
            Some(_) => boards.push( parse_board(&mut lines) ),
            None => break,
        }
    }
    assert_eq!(part1(&numbers, &boards), 4512);
    }
}


fn main() {
    let mut boards = Vec::<[[i32;5];5]>::new();
    let mut lines = include_str!("../inputs/day4.txt").lines();
    let numbers: Vec<i32> = lines.next().unwrap()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();

    loop {
        match lines.next() {
            Some(_) => boards.push( parse_board(&mut lines) ),
            None => break,
        }
    }

    println!("Part 1 {}", part1(&numbers, &boards));

}
