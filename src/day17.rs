#[cfg(test)]
mod test {
}

fn invert_v2(N: i32) -> i32 {
    let a = (-1 + (( (1+8*N) as f32).sqrt() as i32))/2;
    return a as i32;
}

fn v2(v: i32) -> i32 {
    v*(v+1)/2
}

fn fire(mut u: i32, mut v: i32, TARGET: [[i32;2];2]) -> bool {
    const X: usize = 0;
    const Y: usize = 1;
    let mut x = 0;
    let mut y = 0;
    while y > TARGET[0][Y] {
        x += u;
        y += v;
        if (x >= TARGET[0][X] && x <= TARGET[1][X]) && (y >= TARGET[0][Y] && y <= TARGET[1][Y]) {
            return true;
        }
        v -= 1;
        if u != 0 {
            u -= 1;
        }
    }
    return false;
}

fn main() {
    const X: usize = 0;
    const Y: usize = 1;
    const TARGET: [[i32;2];2] = [ [207,-115],[263,-63] ];

    let mut uniq = 0;

    for u in invert_v2(TARGET[0][X])..=TARGET[1][X] {
        println!("u {}",u);
        for v in TARGET[0][Y]..=(v2(TARGET[0][Y])+TARGET[0][Y]) {
            uniq += fire(u,v, TARGET) as i32;
        }
    }

    println!("{}", uniq);
}
