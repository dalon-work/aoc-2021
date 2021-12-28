
#[derive(PartialEq, Eq)]
enum Bracket {
    Round,
    Square,
    Curly,
    Angle,
}

enum LineError {
    Incomplete,
    Corrupted(Bracket)
}

// processes an open bracket
fn open(b: Bracket, ch: &mut std::str::Chars) -> Result<Bracket, LineError> {

    // If we had an error, throw it up the stack
    let pair: Bracket = parse_part1(ch)?;

    if pair != b {
        return Err(LineError::Corrupted(pair));
    }

    // If we made it here, we successfully matched a closing brace, so parse the next char.
    return parse_part1(ch);
}

fn parse_part1(ch: &mut std::str::Chars) -> Result<Bracket, LineError> {
    let c = ch.next();
    if let Some(c) = c {
        match c {
            '(' => return open(Bracket::Round, ch),
            '{' => return open(Bracket::Curly, ch),
            '[' => return open(Bracket::Square, ch),
            '<' => return open(Bracket::Angle, ch),
            ')' => return Ok(Bracket::Round),
            '}' => return Ok(Bracket::Curly),
            ']' => return Ok(Bracket::Square),
            '>' => return Ok(Bracket::Angle),
            _ => unreachable!(),
        }
    } else { // We reached the end of the input
        return Err(LineError::Incomplete);
    }
}

fn part1(lines: &mut std::str::Lines) -> usize {
    let mut score = 0usize;
    for l in lines {
        match parse_part1(&mut l.chars()) {
            Ok(_) => {},
            Err(e) => {
                match e {
                    LineError::Corrupted(b) => {
                        match b {
                            Bracket::Round => score += 3,
                            Bracket::Square => score += 57,
                            Bracket::Curly => score += 1197,
                            Bracket::Angle => score += 25137,
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    return score;
}

mod test {
    use super::*;

    #[test]
    fn test_day10() {
        let mut lines = include_str!("../inputs/day10.example.txt").lines();
        assert_eq!(part1(&mut lines), 26397);
    }
}

fn main() {
    let mut lines = include_str!("../inputs/day10.txt").lines();
    println!("{}",part1(&mut lines));
}
