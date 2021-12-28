extern crate nalgebra as na;

use na::DMatrix;
use na::DVector;

fn main() {
    let ages: Vec<usize> = include_str!("../inputs/day6.txt")
        .trim()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();

    let mut lanternfish = [DVector::<usize>::zeros(9), DVector::<usize>::zeros(0)];

    let transform = DMatrix::from_row_slice(
        9,
        9,
        &[
            0, 1, 0, 0, 0, 0, 0, 0, 0, // 0
            0, 0, 1, 0, 0, 0, 0, 0, 0, // 1
            0, 0, 0, 1, 0, 0, 0, 0, 0, // 2
            0, 0, 0, 0, 1, 0, 0, 0, 0, // 3
            0, 0, 0, 0, 0, 1, 0, 0, 0, // 4
            0, 0, 0, 0, 0, 0, 1, 0, 0, // 5
            1, 0, 0, 0, 0, 0, 0, 1, 0, // 6
            0, 0, 0, 0, 0, 0, 0, 0, 1, // 7
            1, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    ); // 8

    for i in 0..9 {
        lanternfish[0][i] = ages.iter().filter(|a| **a == i).count();
    }

    let (mut pre, mut cur) = (0usize, 1usize);
    for _ in 0..256 {
        lanternfish[cur] = &transform * &lanternfish[pre];
        std::mem::swap(&mut pre, &mut cur);
    }

    println!("{}", lanternfish[pre].sum());
}
