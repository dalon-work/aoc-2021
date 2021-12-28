type Octopi = [[u32;12];12];
type Flashed = [[bool;12];12];

fn increment(energy: &mut Octopi) {
    for i in 1..11 {
        for j in 1..11 {
            energy[i][j] += 1;
        }
    }
}

fn flash(energy: &mut Octopi, flashed: &mut Flashed) -> bool {
    let mut anything_flashed = false;
    for i in 1..11 {
        for j in 1..11 {
            if !flashed[i][j] && energy[i][j] > 9 {
                anything_flashed = true;
                flashed[i][j] = true;
                for ii in i-1..=i+1 {
                    for jj in j-1..=j+1 {
                        if ii == i && jj == j { continue; }
                        energy[ii][jj] += 1;
                    }
                }
            }
        }
    }
    return anything_flashed;
}

fn reset_flashed(energy: &mut Octopi, flashed: &Flashed) -> u32 {
    let mut count = 0;
    for i in 1..11 {
        for j in 1..11 {
            if flashed[i][j] {
                energy[i][j] = 0;
                count += 1;
            }
        }
    }
    return count;
}

fn part1(energy: &mut Octopi) -> u32 {

    let mut count = 0;
    for _ in 0..100 {
        let mut flashed: Flashed = Default::default();
        increment(energy);
        while flash(energy, &mut flashed) {}
        count += reset_flashed(energy, &flashed);
    }
    return count;
}

fn main() {
    let lines = include_str!("../inputs/day11.txt").lines();
    let mut energy: Octopi = Default::default();

    for (i, l) in lines.enumerate() {
        for (j, c) in l.chars().enumerate() {
            energy[i+1][j+1] = c.to_digit(10).unwrap();
        }
    }

    println!("{}", part1(&mut energy));
}
