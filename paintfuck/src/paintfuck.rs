//TODO custom testing with conditional jumps and loops
//TODO custom edge case testing
//TODO rewrite using iterator

use std::default::Default;
use std::{cmp, str, string};

#[derive(Debug, Clone, Copy)]
enum Cardinal {
    N,
    S,
    E,
    W,
}

enum Token {
    Move(Cardinal),
    Flip,
    CondJmp,
    CondLoop,
}
#[derive(Debug)]
struct ParseTokenErr(String);

impl Token {
    fn try_from_char(c: char) -> Result<Self, ParseTokenErr> {
        match c {
            'n' => Ok(Token::Move(Cardinal::N)),
            'e' => Ok(Token::Move(Cardinal::E)),
            's' => Ok(Token::Move(Cardinal::S)),
            'w' => Ok(Token::Move(Cardinal::W)),
            '*' => Ok(Token::Flip),
            '[' => Ok(Token::CondJmp),
            ']' => Ok(Token::CondLoop),
            _ => Err(ParseTokenErr(format!("Invalid token character `{}`", c))),
        }
    }
}

struct TokenIter {
}

#[derive(Debug)]
enum Instruction {
    Move(Cardinal),
    Flip,
}

struct Interpreter {
    next: usize,
    life: usize,
    scope: Vec<usize>,
    code: &str,
}

impl Interpreter {
    fn parse_str(source: &str, life: usize) -> Result<Self, ParseTokenErr> {
        Ok(Self {
            next: 0,
            life,
            scope: Vec::new(),
            code: source
                .chars()
                .map(Token::try_from_char)
                .collect::<Result<Vec<Token>, ParseTokenErr>>()?,
        })
    }

    fn next_ins(&mut self, set: bool) -> Option<Instruction> {
        self.life = self.life.checked_sub(1)?;

        if let Some(tok) = self.code.get(self.next) {
            self.next += 1;
            match tok {
                Token::Move(dir) => Some(Instruction::Move(*dir)),
                Token::Flip => Some(Instruction::Flip),
                Token::CondJmp => {
                    if set {
                        self.next += 1;
                        self.scope.push(self.next);
                    } else {
                        self.next += self.jump().expect("Syntax Error!");
                    }
                    self.next_ins(set)
                }
                Token::CondLoop => {
                    if set {
                        self.next = *self.scope.last().expect("Syntax Error!");
                    } else {
                        self.next += 1;
                        self.scope.pop();
                    }
                    self.next_ins(set)
                }
            }
        } else {
            None
        }
    }

    fn jump(&self) -> Option<usize> {
        self.code
            .get(self.next + 1..)?
            .iter()
            .scan(1, |scope, tok| {
                match tok {
                    Token::CondJmp => *scope += 1,
                    Token::CondLoop => *scope -= 1,
                    _ => {}
                }
                Some(*scope)
            })
            .position(|scope| scope == 0)
            .map(|idx| idx + 1)
    }
}

//y is above x such that natural ordering is column first
//this allows Board to properly sort the flipped points in
//such a way that to_string() works properly with inner x and outer y
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    y: usize,
    x: usize,
}

impl Point {
    fn dist_from(&self, other: &Point) -> Point {
        Point {
            x: cmp::max(self.x, other.x) - cmp::min(self.x, other.x),
            y: cmp::max(self.y, other.y) - cmp::min(self.y, other.y),
        }
    }
}

struct Board {
    width: usize,
    height: usize,
    cursor: Point,
    flipped: Vec<Point>,
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cursor: Point::default(),
            flipped: Vec::new(),
        }
    }

    fn move_cursor(&mut self, dir: Cardinal) {
        self.cursor = self.move_pt(&self.cursor, dir);
    }

    fn move_pt(&self, pt: &Point, dir: Cardinal) -> Point {
        let height_bound = self.height - 1;
        let width_bound = self.width - 1;
        let mut new_pt: Point = pt.clone();
        match dir {
            Cardinal::N => {
                if pt.y == 0 {
                    new_pt.y = height_bound;
                } else {
                    new_pt.y -= 1;
                }
            }
            Cardinal::S => {
                if pt.y == height_bound {
                    new_pt.y = 0;
                } else {
                    new_pt.y += 1;
                }
            }
            Cardinal::E => {
                if pt.x == width_bound {
                    new_pt.x = 0;
                } else {
                    new_pt.x += 1;
                }
            }
            Cardinal::W => {
                if pt.x == 0 {
                    new_pt.x = width_bound;
                } else {
                    new_pt.x -= 1;
                }
            }
        }

        new_pt
    }

    fn flip(&mut self) {
        match self.flipped.binary_search(&self.cursor) {
            Ok(idx) => {
                self.flipped.remove(idx);
            }
            Err(idx) => self.flipped.insert(idx, self.cursor.clone()),
        }
    }

    fn is_cursor_set(&self) -> bool {
        self.flipped.binary_search(&self.cursor).is_ok()
    }
}

impl string::ToString for Board {
    fn to_string(&self) -> String {
        let mut flipped_points = self.flipped.iter().peekable();
        (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| {
                        let out = if let Some(pt) = flipped_points.peek() {
                            Point { x, y } == **pt
                        } else {
                            false
                        };
                        if out {
                            flipped_points.next();
                            '1'
                        } else {
                            '0'
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\r\n")
    }
}

pub fn interpreter(code: &str, iterations: usize, width: usize, height: usize) -> String {
    assert!(width > 0 && height > 0, "Invalid Board Dimensions!");

    let mut instructions = Interpreter::parse_str(code, iterations).expect("Invalid Code");
    let mut board = Board::new(width, height);

    loop {
        let set = board.is_cursor_set();
        let ins = instructions.next_ins(set);
        match ins {
            Some(Instruction::Move(dir)) => board.move_cursor(dir),
            Some(Instruction::Flip) => board.flip(),
            None => break board.to_string(),
        }
    }
}
