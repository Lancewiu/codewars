use std::default::Default;
use std::{convert, hint, str, string};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cardinal {
    N,
    S,
    E,
    W,
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Move(Cardinal),
    Flip,
    CondJmp,
    CondLoop,
    Comment,
}

impl convert::From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            'n' => Token::Move(Cardinal::N),
            'e' => Token::Move(Cardinal::E),
            's' => Token::Move(Cardinal::S),
            'w' => Token::Move(Cardinal::W),
            '*' => Token::Flip,
            '[' => Token::CondJmp,
            ']' => Token::CondLoop,
            _ => Token::Comment,
        }
    }
}

struct Interpreter {
    next: usize,
    scope: Vec<usize>,
    code: Vec<Token>,
    board: Board,
}

impl Interpreter {
    fn new(code: Vec<Token>, board: Board) -> Self {
        Self {
            next: 0,
            scope: Vec::new(),
            code,
            board,
        }
    }

    fn run(mut self, life: usize) -> Board {
        let codelen = self.code.len();
        for _ in 0..life {
            if self.next >= codelen {
                break;
            }
            match self.code[self.next] {
                Token::Move(dir) => {
                    self.board.move_cursor(dir);
                    self.next += 1;
                }
                Token::Flip => {
                    self.board.flip();
                    self.next += 1;
                }
                Token::CondJmp => {
                    if self.board.is_cursor_set() {
                        self.next += 1;
                        self.scope.push(self.next);
                    } else {
                        self.next = self.jump(self.next + 1).expect("Syntax Error!");
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
                Token::Comment => unsafe { hint::unreachable_unchecked() },
            }
        }
        self.board
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
            .map(|idx| idx + from + 1) //one after
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
        match dir {
            Cardinal::N => {
                self.cursor.y = self.cursor.y.checked_sub(1).unwrap_or(self.height - 1);
            }
            Cardinal::S => {
                self.cursor.y = (self.cursor.y + 1) % self.height;
            }
            Cardinal::E => {
                self.cursor.x = (self.cursor.x + 1) % self.width;
            }
            Cardinal::W => {
                self.cursor.x = self.cursor.x.checked_sub(1).unwrap_or(self.width - 1);
            }
        }
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
        let mut res = Vec::with_capacity(self.height);
        for y in 0..self.height {
            let mut row = String::with_capacity(self.width);
            for x in 0..self.width {
                let out = if let Some(pt) = flipped_points.peek() {
                    Point { x, y } == **pt
                } else {
                    false
                };
                row.push(if out {
                    flipped_points.next();
                    '1'
                } else {
                    '0'
                });
            }
            res.push(row);
        }
        res.join("\r\n")
    }
}

#[allow(dead_code)]
pub fn interpreter(code: &str, iterations: usize, width: usize, height: usize) -> String {
    assert!(width > 0 && height > 0, "Invalid Board Dimensions!");

    let code_tok = code
        .chars()
        .map(Token::from)
        .filter(|t| *t != Token::Comment)
        .collect();

    Interpreter::new(code_tok, Board::new(width, height))
        .run(iterations)
        .to_string()
}
