use ndarray::Array2;
use ndarray::ArrayView2;

fn part1(height: ArrayView2<u32>) -> u32 {

    let mut count = 0u32;
    for win in height.windows((3,3)) {
        let n = win[[2,1]];
        let s = win[[0,1]];
        let e = win[[1,2]];
        let w = win[[1,0]];
        let c = win[[1,1]];
        if c < n && c < s && c < e && c < w {
            count += c+1;
        }
    }
    return count;
}

fn main() {
    let mut lines = include_str!("../inputs/day9.txt").lines();
    let mut height = Array2::<u32>::from_elem((102,102),9);

    for (i, l) in lines.enumerate() {
        for (j, c) in l.chars().enumerate() {
            height[[i+1,j+1]] = c.to_digit(10).unwrap();
        }
    }

    println!("{}",part1(height.view()));
}
