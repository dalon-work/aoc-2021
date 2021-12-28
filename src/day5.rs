use regex::Regex;

type Vent = [[usize; 2]; 2];
type Map = [[usize; 1000]; 1000];

fn parse_line(line: &str, re: &Regex) -> Vent {
    let mut v: Vent = Default::default();
    let cap = re.captures(line).unwrap();
    v[0][0] = cap[1].parse().unwrap();
    v[0][1] = cap[2].parse().unwrap();
    v[1][0] = cap[3].parse().unwrap();
    v[1][1] = cap[4].parse().unwrap();
    return v;
}

fn mark(map: &mut Map, v: &Vent, diag: bool) {
    if v[0][0] == v[1][0] || v[0][1] == v[1][1] {
        let x0 = std::cmp::min(v[0][0], v[1][0]);
        let x1 = std::cmp::max(v[0][0], v[1][0]);
        let y0 = std::cmp::min(v[0][1], v[1][1]);
        let y1 = std::cmp::max(v[0][1], v[1][1]);
        for x in x0..=x1 {
            for y in y0..=y1 {
                map[x][y] += 1
            }
        }
    } else if diag {
        let xi = (v[0][0] < v[1][0]) as isize - (v[0][0] > v[1][0]) as isize;
        let yi = (v[0][1] < v[1][1]) as isize - (v[0][1] > v[1][1]) as isize;
        let mut x = v[0][0] as isize;
        let mut y = v[0][1] as isize;
        for _ in std::cmp::min(v[0][0], v[1][0])..=std::cmp::max(v[0][0], v[1][0]) {
            map[x as usize][y as usize] += 1;
            x += xi;
            y += yi;
        }
    }
}

fn run(vents: &[Vent], diag: bool) -> usize {
    let mut map = [[0; 1000]; 1000];
    for v in vents {
        mark(&mut map, v, diag);
    }

    return map.iter().flatten().map(|c| (*c > 1) as usize).sum();
}

fn main() {
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let vents: Vec<Vent> = include_str!("../inputs/day5.txt")
        .lines()
        .map(|line| parse_line(line, &re))
        .collect();
    println!("Part 1 {}", run(&vents, false));
    println!("Part 2 {}", run(&vents, true));
}
