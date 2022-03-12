use std::ops::Add;
use std::fmt;

#[derive(Debug, Clone, Copy)]
enum Token {
    Open,
    Close,
    Comma,
    Num(usize),
}

#[derive(Clone)]
struct SF {
    tokens: Vec<Token>
}

impl SF {
    fn parse_value(c: char) -> Token {
        Token::Num(c.to_digit(10).unwrap() as usize)
    }

    fn parse_chars(ch: &mut std::str::Chars) -> Vec<Token> {
        let mut t = vec![];

        while let Some(c) = ch.next() {
            match c {
                '[' => t.push(Token::Open),
                ',' => t.push(Token::Comma),
                ']' => t.push(Token::Close),
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => t.push( SF::parse_value(c) ),
                _ => unreachable!(),
            }
        }

        'outer: loop {
            let len = t.len();
            for i in 0..len-2 {
                if let (Token::Num(d1), Token::Num(d2)) = (t[i],t[i+1]) {
                    let new = d1*10 + d2;
                    t[i] = Token::Num(new);
                    t.remove(i+1);
                    continue 'outer;
                }
            }
            break;
        }

        return t;
    }

    fn parse(s: &str) -> Self {
        SF{ tokens: SF::parse_chars(&mut s.chars()) }
    }

    fn mag(&self) -> usize {
        let mut mult_stack = vec![1];
        let mut mag = 0;
        for c in &self.tokens {
            match c {
                Token::Open => {
                    let mult: usize = *mult_stack.last().unwrap();
                    mult_stack.push(mult*3);
                },
                Token::Comma => {
                    mult_stack.pop();
                    let mult: usize = *mult_stack.last().unwrap();
                    mult_stack.push(mult*2);
                },
                Token::Close => {
                    mult_stack.pop().unwrap();
                },
                Token::Num(val) => {
                    let mult: usize = *mult_stack.last().unwrap();
                    mag += mult*val;
                },
            }
        }
        return mag;
    }

    fn find_left_nbr(&self, start: usize) -> Option<usize> {
        for i in (0..start).rev() {
            if let Token::Num(_) = self.tokens[i] {
                return Some(i);
            }
        }
        return None;
    }

    fn find_right_nbr(&self, start: usize) -> Option<usize> {
        for i in start..self.tokens.len() {
            if let Token::Num(_) = self.tokens[i] {
                return Some(i);
            }
        }
        return None;
    }

    fn explode_impl(&mut self, i: usize) {
        if let (Token::Open, 
                Token::Num(left),
                Token::Comma,
                Token::Num(right),
                Token::Close) = (self.tokens[i],
                                 self.tokens[i+1],
                                 self.tokens[i+2],
                                 self.tokens[i+3],
                                 self.tokens[i+4]) {
            let left_nbr = self.find_left_nbr(i);
            let right_nbr = self.find_right_nbr(i+4);

            if let Some(left_nbr) = left_nbr {
                if let Token::Num(ref mut left_nbr_val) = self.tokens[left_nbr] {
                    *left_nbr_val += left;
                }
            }

            if let Some(right_nbr) = right_nbr {
                if let Token::Num(ref mut right_nbr_val) = self.tokens[right_nbr] {
                    *right_nbr_val += right;
                }
            }

            self.tokens[i] = Token::Num(0);
            self.tokens.drain(i+1..i+5);
        } else {
            unreachable!();
        }
    }


    fn explode(&mut self) -> bool {
        let mut depth = 0;
        let len = self.tokens.len();

        for i in 0..len {
            match self.tokens[i] {
                Token::Open => {
                    depth += 1;
                    if depth > 4 && i+4 < len {
                        self.explode_impl(i);
                        return true;
                    }
                },
                Token::Close => depth -= 1,
                _ => {}
            };
        }
        return false;
    }

    fn split(&mut self) -> bool {
        let len = self.tokens.len();

        for i in 0..len {
            if let Token::Num(num) = self.tokens[i] {
                if num > 9 {
                    let left = num/2;
                    let right = num-left;
                    self.tokens.insert(i+1,Token::Close);
                    self.tokens.insert(i+1,Token::Num(right));
                    self.tokens.insert(i+1,Token::Comma);
                    self.tokens.insert(i+1,Token::Num(left));
                    self.tokens[i] = Token::Open;
                    return true;
                }
            }
        }
        return false;
    }

    fn reduce_impl(&mut self) -> bool {
        if self.explode() {
            return true;
        }

        return self.split();
    }

    fn reduce(&mut self) {
        while self.reduce_impl() {}
    }
}

impl Add for &SF {
    type Output = SF;

    fn add(self, other: Self) -> SF {

        if self.tokens.len() == 0 {
            return other.clone();
        }

        if other.tokens.len() == 0 {
            return self.clone();
        }

        let mut tokens = Vec::<Token>::with_capacity( self.tokens.len() + other.tokens.len() + 3 );
        tokens.push(Token::Open);
        tokens.extend_from_slice(&self.tokens);
        tokens.push(Token::Comma);
        tokens.extend_from_slice(&other.tokens);
        tokens.push(Token::Close);

        let mut s = SF{tokens};
        s.reduce();

        return s;
    }
}

impl fmt::Display for SF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for t in &self.tokens {
            match t {
                Token::Open => write!(f,"[")?,
                Token::Comma => write!(f, ",")?,
                Token::Close => write!(f, "]")?,
                Token::Num(i) => write!(f, "{}",i)?,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mag() {
        assert_eq!(29, SF::parse("[9,1]").mag());
        assert_eq!(21, SF::parse("[1,9]").mag());
        assert_eq!(129, SF::parse("[[9,1],[1,9]]").mag());
        assert_eq!(143, SF::parse("[[1,2],[[3,4],5]]").mag());
        assert_eq!(1384, SF::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").mag());
        assert_eq!(445, SF::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").mag());
        assert_eq!(791, SF::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").mag());
        assert_eq!(1137, SF::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").mag());
        assert_eq!(3488, SF::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").mag());

    }

    #[test]
    fn test_add() {
        let a = SF::parse("[1,2]");
        let b = SF::parse("[[3,4],5]");
        let c = a + b;
        assert_eq!(c.to_string(), "[[1,2],[[3,4],5]]");
    }

    #[test]
    fn test_explode() {
        let mut a = SF::parse("[[[[[9,8],1],2],3],4]");
        assert!( a.explode() );
        assert_eq!(a.to_string(),"[[[[0,9],2],3],4]");

        let mut a = SF::parse("[7,[6,[5,[4,[3,2]]]]]");
        assert!( a.explode() );
        assert_eq!(a.to_string(), "[7,[6,[5,[7,0]]]]");

        let mut a = SF::parse("[[6,[5,[4,[3,2]]]],1]");
        assert!( a.explode() );
        assert_eq!(a.to_string(), "[[6,[5,[7,0]]],3]");

        let mut a = SF::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        assert!( a.explode() );
        assert_eq!(a.to_string(), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        let mut a = SF::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert!( !a.explode() );
        assert_eq!(a.to_string(), "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    }

    #[test]
    fn test_split() {
        let mut a = SF::parse(    "[[[[0,7],4],[15,[0,13]]],[1,1]]");
        assert_eq!(a.to_string(), "[[[[0,7],4],[15,[0,13]]],[1,1]]");
        assert!( a.split() );
        assert_eq!(a.to_string(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
        assert!( a.split() );
        assert_eq!(a.to_string(), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");

        let mut a = SF::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert!( !a.split() );
    }

    #[test]
    fn test_reduce() {
        let mut a = SF::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        a.reduce();
        assert_eq!(a.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    #[ignore]
    fn test_add_many() {
        let a = SF::parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
        let b = SF::parse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");
        let c = a + b;
        assert_eq!(c.to_string(), "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
    }

    #[test]
    fn test_sum_lines() {
        let s = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n\
                 [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n\
                 [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]\n\
                 [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]\n\
                 [7,[5,[[3,8],[1,4]]]]\n\
                 [[2,[2,2]],[8,[8,1]]]\n\
                 [2,9]\n\
                 [1,[[[9,3],9],[[9,0],[0,7]]]]\n\
                 [[[5,[7,4]],7],1]\n\
                 [[[[4,2],2],6],[8,7]]";
        let a = sum_lines(s.lines());
        assert_eq!(a.to_string(), "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    }
}

fn sum_lines(lines: std::str::Lines) -> SF {
    let mut sf = SF{tokens: vec![]};

    for l in lines {
        sf = &sf + &SF::parse(l);
    }

    return sf;
}

fn main() {
    let input = include_str!("../inputs/day18.txt");
    let sum = sum_lines(input.lines());
    println!("part 1 mag {}", sum.mag());

    let allsf: Vec::<SF> = input.lines().map(|l| SF::parse(l)).collect();

    let mut max_mag = 0;
    for i in 0..allsf.len() {
        for j in 0..allsf.len() {
            if i == j {
                continue;
            }
            max_mag = std::cmp::max(max_mag, (&allsf[i] + &allsf[j]).mag());
            max_mag = std::cmp::max(max_mag, (&allsf[j] + &allsf[i]).mag());
        }
    }
    println!("part 2 max {}", max_mag);
}
