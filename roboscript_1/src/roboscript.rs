use std::fmt::{self, Write};

fn write_open_tag(f: &mut String, c: char) -> fmt::Result {
    let color = match c {
        'F' => "pink",
        'L' => "red",
        'R' => "green",
        _ if c.is_ascii_digit() => "orange",
        _ => {
            return Ok(());
        }
    };
    write!(f, "<span style=\"color: {}\">", color)
}

fn write_close_tag(f: &mut String, prev: char) -> fmt::Result {
    match prev {
        'F' | 'L' | 'R' => write!(f, "</span>"),
        _ if prev.is_ascii_digit() => write!(f, "</span>"),
        _ => Ok(()),
    }
}

#[allow(dead_code)]
pub fn highlight(code: &str) -> String {
    let mut res = String::with_capacity(code.len());
    let mut prev_opt: Option<char> = None;

    for c in code.chars() {
        if let Some(prev) = prev_opt {
            if prev == c || (c.is_ascii_digit() && prev.is_ascii_digit()) {
                Ok(())
            } else {
                write_close_tag(&mut res, prev).and(write_open_tag(&mut res, c))
            }
        } else {
            write_open_tag(&mut res, c)
        }
        .expect("Write tags failed");
        write!(res, "{}", c).expect("Write character failed");
        prev_opt = Some(c);
    }
    if let Some(prev) = prev_opt {
        write_close_tag(&mut res, prev).expect("Write end tag failed");
    }
    res
}
