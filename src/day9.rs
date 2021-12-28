use ndarray::Array2;
use ndarray::ArrayView2;
use ndarray::ArrayViewMut2;

fn part1(height: ArrayView2<u32>) -> u32 {
    let mut count = 0u32;
    for win in height.windows((3, 3)) {
        let n = win[[2, 1]];
        let s = win[[0, 1]];
        let e = win[[1, 2]];
        let w = win[[1, 0]];
        let c = win[[1, 1]];
        if c < n && c < s && c < e && c < w {
            count += c + 1;
        }
    }
    return count;
}

fn floodfill(height: &mut ArrayViewMut2<u32>, i: usize, j: usize) -> u32 {
    if height[[i, j]] == 9 {
        return 0;
    }

    height[[i, j]] = 9;

    let mut area = 1;

    area += floodfill(height, i, j + 1);
    area += floodfill(height, i, j - 1);
    area += floodfill(height, i + 1, j);
    area += floodfill(height, i - 1, j);

    return area;
}

fn part2(mut height: ArrayViewMut2<u32>) -> u32 {
    let mut basins = Vec::new();

    for i in 1..=101 {
        for j in 1..=101 {
            if height[[i, j]] != 9 {
                basins.push(floodfill(&mut height, i, j));
            }
        }
    }

    basins.sort();
    basins.reverse();

    return basins[0] * basins[1] * basins[2];
}

fn main() {
    let mut lines = include_str!("../inputs/day9.txt").lines();
    let mut height = Array2::<u32>::from_elem((102, 102), 9);

    for (i, l) in lines.enumerate() {
        for (j, c) in l.chars().enumerate() {
            height[[i + 1, j + 1]] = c.to_digit(10).unwrap();
        }
    }

    println!("{}", part1(height.view()));
    println!("{}", part2(height.view_mut()));
}
