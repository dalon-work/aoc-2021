use std::ops::Add;
use std::fmt;

#[derive(Debug)]
enum Token {
    Open,
    Close,
    Comma,
    Num(usize),
}

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
        return t;
    }

    fn parse(s: &str) -> Self {
        SF{ tokens: SF::parse_chars(&mut s.chars()) }
    }

    fn mag(&self) -> usize {
        let mut mult_stack = vec![1];
        let mut mag = 0;
        for c in &self.tokens {
            println!("{:?}",c);
            println!("{:?}",mult_stack);
            match c {
                Token::Open => {
                    let mult = mult_stack.last().unwrap();
                    mult_stack.push(mult*3);
                },
                Token::Comma => {
                    mult_stack.pop();
                    let mult = mult_stack.last().unwrap();
                    mult_stack.push(mult*2);
                },
                Token::Close => {
                    mult_stack.pop().unwrap();
                },
                Token::Num(val) => {
                    let mult = mult_stack.last().unwrap();
                    mag += mult*val;
                },
            }
        }
        return mag;
    }
}

impl Add for SF {
    type Output = Self;

    fn add(mut self, mut other: Self) -> Self {

        if self.tokens.len() == 0 {
            return other;
        }

        if other.tokens.len() == 0 {
            return self;
        }

        let mut tokens = Vec::<Token>::with_capacity( self.tokens.len() + other.tokens.len() + 3 );
        tokens.push(Token::Open);
        tokens.append(&mut self.tokens);
        tokens.push(Token::Comma);
        tokens.append(&mut other.tokens);
        tokens.push(Token::Close);

        return Self{tokens};
    }
}

impl fmt::Display for SF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for t in &self.tokens {
            match t {
                Token::Open => write!(f,"["),
                Token::Comma => write!(f, ","),
                Token::Close => write!(f, "]"),
                Token::Num(i) => write!(f, "{}",i),
            };
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
/*
    #[test]
    fn test_explode() {
        let mut a = SF::parse("[[[[[9,8],1],2],3],4]");
        assert!( a.explode() );
        assert_eq!(a.to_string(),"[[[[0,9],2],3],4]");
    }
*/
}

fn main() {
    /*
        let input = include_str!("../inputs/day18.example.txt");
        let sum = sum_lines(input.lines());
        let mut idx = 0;
        println!("part 1 mag {}", mag(&sum, &mut idx, 1));
    */
}
