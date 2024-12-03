use aoc24 as aoc_lib;

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Mul,
    Number(u64),
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Invalid,
    Do,
    Dont,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut token_stream: Vec<Token> = Vec::new();
    let mut bytes = input.bytes().peekable();
    while let Some(b) = bytes.next() {
        let token = match b {
            b'm' => match (bytes.next(), bytes.next()) {
                (Some(b'u'), Some(b'l')) => Token::Mul,
                _ => continue,
            },
            b'0'..=b'9' => {
                let mut num: Vec<u8> = vec![b];
                while let Some(b) = bytes.peek() {
                    match *b {
                        b'0'..=b'9' => {
                            num.push(*b);
                            bytes.next();
                        }
                        _ => break,
                    }
                }
                let Some(num) = String::from_utf8(num).ok().and_then(|n| n.parse().ok()) else {
                    continue;
                };
                Token::Number(num)
            }
            b'(' => Token::LeftParenthesis,
            b')' => Token::RightParenthesis,
            b',' => Token::Comma,
            b'd' => {
                let mut d1 = b"do()".iter().skip(1);
                let mut d2 = b"don't()".iter().skip(1);
                while let Some(b) = bytes.peek() {
                    let d1 = d1.next();
                    let d2 = d2.next();
                    if Some(b) == d2 || Some(b) == d1 {
                        bytes.next();
                    } else {
                        break;
                    }
                }

                if d2.next().is_none() {
                    Token::Dont
                } else if d1.next().is_none() {
                    Token::Do
                } else {
                    continue;
                }
            }
            _ => Token::Invalid,
        };
        token_stream.push(token);
    }

    token_stream
}

#[derive(Default)]
struct Context {
    alive: bool,
    paren: bool,
    comma: bool,
    do_mul: bool,
    left: Option<u64>,
    right: Option<u64>,
}

impl Context {
    fn new() -> Self {
        Self {
            do_mul: true,
            ..Self::default()
        }
    }

    fn start(&mut self) {
        *self = Self {
            alive: true,
            do_mul: self.do_mul,
            ..Self::default()
        }
    }

    fn reset(&mut self) {
        self.alive = false;
        self.paren = false;
        self.comma = false;
        self.left = None;
        self.right = None;
    }
}

fn sum_mul(token_stream: &[Token], use_do_cx: bool) -> u64 {
    let tokens = token_stream.iter();
    let mut sum = 0;
    let mut cx = Context::new();
    for t in tokens {
        match t {
            Token::Mul => cx.start(),
            Token::LeftParenthesis
                if cx.alive
                    && !cx.paren
                    && !cx.comma
                    && cx.left.is_none()
                    && cx.right.is_none() =>
            {
                cx.paren = true;
            }
            Token::Number(l)
                if cx.alive && cx.paren && !cx.comma && cx.left.is_none() && cx.right.is_none() =>
            {
                cx.left = Some(*l);
            }
            Token::Comma
                if cx.alive && cx.paren && !cx.comma && cx.left.is_some() && cx.right.is_none() =>
            {
                cx.comma = true
            }
            Token::Number(r)
                if cx.alive && cx.paren && cx.comma && cx.left.is_some() && cx.right.is_none() =>
            {
                cx.right = Some(*r);
            }
            Token::RightParenthesis
                if cx.alive && cx.paren && cx.comma && cx.left.is_some() && cx.right.is_some() =>
            {
                if cx.do_mul || !use_do_cx {
                    sum += cx.left.unwrap() * cx.right.unwrap();
                }
                cx.reset();
            }
            Token::Dont => cx.do_mul = false,
            Token::Do => cx.do_mul = true,
            _ => cx.reset(),
        }
    }

    sum
}

fn main() {
    let input = aoc_lib::read_file("day03.txt");
    let token_stream = tokenize(&input);

    let part_1 = sum_mul(&token_stream, false);
    println!("part_1 {part_1}");

    let part_2 = sum_mul(&token_stream, true);
    println!("part_2 {part_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = aoc_lib::read_file("day03.txt");

        let token_stream = tokenize(&input);
        let sum = sum_mul(&token_stream, false);

        assert_eq!(sum, 166357705)
    }

    #[test]
    fn part_2() {
        const INPUT: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let token_stream = tokenize(INPUT);
        println!("{token_stream:?}");
        let sum = sum_mul(&token_stream, true);

        assert_eq!(sum, 88811886);
    }
}
