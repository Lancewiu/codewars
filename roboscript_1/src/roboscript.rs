use std::string::ToString;
use std::fmt::{self, Display};

enum Color {
    Pink,
    Red,
    Green,
    Orange,
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Color::*;
        write!(f, "{}", match self {
            Pink => "pink",
            Red => "red",
            Green => "green",
            Orange => "orange",
        })
    }
}

enum Prepend {
    Full(Color),
    StartOnly(Color),
    None,
}

impl ToString for Prepend {
    fn to_string(&self) -> String {
        use Prepend::*;
        match self {
            Full(col) => format!("</span><span style=\"color: {}\">", col),
            StartOnly(col) => format!("<span style=\"color: {}\">", col),
            None => String::new(),
        }
    }
}

pub fn highlight(code: &str) -> String {
  // Implement your syntax highlighter here
  let code_len = code.len();
  let mut code_iter = code.chars().peekable();

  code.chars()
      .scan(None, |mut prev, c| {
          match c {
              'F' => match prev {
                  Some('F') => format!(""),
                  None =>
                  _ =>
              },
              'L' => match prev {
                  Some('L') =>,
                  None =>,
                  _ =>
              },
              'R' => match prev {
                  Some('R') =>,
              },
              digit if digit.is_ascii_digit() => match prev {
                  Some(d) if d.is_ascii_digit() =>,
              },
              '(' | ')' => match prev {
                  Some('(') | Some(')') => Prepend::None
              },
              _ => unreachable!(),
          }
          prev = Some(c);
      })
      .join()
}
