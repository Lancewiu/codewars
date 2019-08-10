fn invert_brace(close: char) -> char {
    match close {
        ')' => '(',
        '}' => '{',
        ']' => '[',
        _ => unsafe { ::std::hint::unreachable_unchecked() },
    }
}

#[allow(dead_code)]
pub fn valid_braces(s: &str) -> bool {
    let midpoint = s.len() / 2;
    let mut brace_buf: Vec<char> = Vec::with_capacity(midpoint);
    for brace in s.chars() {
        match brace {
            '(' | '[' | '{' => {
                brace_buf.push(brace);
                if brace_buf.len() > midpoint {
                    return false;
                }
            }
            ')' | ']' | '}' => {
                if Some(invert_brace(brace)) != brace_buf.pop() {
                    return false;
                }
            }
            _ => {
                return false;
            }
        }
    }
    true
}
