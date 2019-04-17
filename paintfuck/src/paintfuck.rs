//TODO custom edge case testing

use std::default::Default;
use std::{cmp, fmt, str, string};

#[derive(Debug, Clone, Copy)]
enum Cardinal {
    N,
    S,
    E,
    W,
}

struct ParseTokenErr(char);

impl fmt::Display for ParseTokenErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Character `{}`", self.0)
    }
}

enum Token {
    Move(Cardinal),
    Flip,
    CondJmp,
    CondLoop,
}

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
            _ => Err(ParseTokenErr(c)),
        }
    }
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
    code: Vec<Token>,
    board: Board,
}

impl Interpreter {
    fn new(code: Vec<Token>, board: Board, life: usize) -> Self {
        Self {
            next: 0,
            life,
            scope: Vec::new(),
            code,
            board,
        }
    }

    fn run(mut self) -> Board {
        let codelen = self.code.len();
        while self.life > 0 && codelen > self.next {
            self.next();
        }
        self.board
    }

    fn next(&mut self) {
        match self.code[self.next] {
            Token::Move(dir) => {
                self.board.move_cursor(dir);
                self.life -= 1;
                self.next += 1;
            }
            Token::Flip => {
                self.board.flip();
                self.life -= 1;
                self.next += 1;
            }
            Token::CondJmp => {
                if self.board.is_cursor_set() {
                    self.next += 1;
                    self.scope.push(self.next);
                } else {
                    self.next += self.jump(self.next + 1).expect("Syntax Error!");
                }
            }
            Token::CondLoop => {
                if self.board.is_cursor_set() {
                    self.next = *self.scope.last().expect("Syntax Error!");
                } else {
                    self.next += 1;
                    self.scope.pop();
                }
            }
        }
    }

    fn jump(&self, from: usize) -> Option<usize> {
        self.code
            .get(from..)?
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

    let code_tok = code
        .chars()
        .map(Token::try_from_char)
        .collect::<Result<Vec<Token>, ParseTokenErr>>()
        .unwrap_or_else(|e| panic!("{}", e));

    Interpreter::new(code_tok, Board::new(width, height), iterations)
        .run()
        .to_string()
}
