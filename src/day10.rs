#[derive(PartialEq, Eq)]
enum Brace {
    Round,
    Square,
    Curly,
    Angle,
}

#[derive(Copy, Clone)]
enum LineError {
    Incomplete(usize),
    Corrupted(usize)
}

fn corrupted(b: Brace) -> LineError {
    LineError::Corrupted(
        match b {
            Brace::Round => 3,
            Brace::Square => 57,
            Brace::Curly => 1197,
            Brace::Angle => 25137,
        }
    )
}

fn incomplete(b: Brace, mut score: usize) -> LineError {
    score *= 5;
    LineError::Incomplete(score +
        match b {
            Brace::Round => 1,
            Brace::Square => 2,
            Brace::Curly => 3,
            Brace::Angle => 4,
        }
    )
}

// processes an open bracket
fn open(b: Brace, ch: &mut std::str::Chars) -> Result<Brace, LineError> {

    let parsed = parse(ch);

    match parsed {
        Ok(closing_brace) => {
            if closing_brace != b {
                return Err(corrupted(closing_brace));
            } else {
                return parse(ch);
            }
        }
        Err(err) => {
            match err {
                LineError::Incomplete(score) => {
                    return Err(incomplete(b, score));
                },
                // pass corrupted back up the stack
                LineError::Corrupted(_) => return parsed,
            }
        }
    }
}

fn parse(ch: &mut std::str::Chars) -> Result<Brace, LineError> {
    let c = ch.next();
    if let Some(c) = c {
        match c {
            '(' => return open(Brace::Round, ch),
            '{' => return open(Brace::Curly, ch),
            '[' => return open(Brace::Square, ch),
            '<' => return open(Brace::Angle, ch),
            ')' => return Ok(Brace::Round),
            '}' => return Ok(Brace::Curly),
            ']' => return Ok(Brace::Square),
            '>' => return Ok(Brace::Angle),
            _ => unreachable!(),
        }
    } else { // We reached the end of the input
        return Err(LineError::Incomplete(0));
    }
}

fn run(lines: &mut std::str::Lines) -> (usize,usize) {
    let mut part1 = 0usize;
    let mut part2 = Vec::new();
    for l in lines {
        if let Err(e) = parse(&mut l.chars()) {
            match e {
                LineError::Corrupted(score) => { part1 += score; },
                LineError::Incomplete(score) => { part2.push(score) },
            }
        }
    }
    part2.sort();
    return (part1, part2[part2.len()/2])
}

fn main() {
    let mut lines = include_str!("../inputs/day10.txt").lines();
    println!("{:?}",run(&mut lines));
}
