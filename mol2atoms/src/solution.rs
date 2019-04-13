// type Atom = (String, usize);
// type Molecule = Vec<Atom>;
// Remove this afterwards
use crate::Molecule;

use std::collections::HashMap;
use std::ops;

pub type ParseError = String;

#[derive(Debug, PartialEq, Eq)]
enum BracketType {
    Curly,
    Round,
    Square,
}

#[derive(Debug)]
enum Token {
    Num(usize),                //numerical digit
    Lower(char),               //lowercase letter
    Upper(char),               //uppercase letter
    OpenBracket(BracketType),  // {, (, [
    CloseBracket(BracketType), // }, ), ]
}

impl Token {
    pub fn try_from_char(c: char) -> Result<Token, String> {
        match c {
            '{' => Ok(Token::OpenBracket(BracketType::Curly)),
            '(' => Ok(Token::OpenBracket(BracketType::Round)),
            '[' => Ok(Token::OpenBracket(BracketType::Square)),
            '}' => Ok(Token::CloseBracket(BracketType::Curly)),
            ')' => Ok(Token::CloseBracket(BracketType::Round)),
            ']' => Ok(Token::CloseBracket(BracketType::Square)),
            n if n.is_ascii_digit() => Ok(Token::Num(
                n.to_digit(10).expect("Impossible ASCII digit") as usize,
            )),
            upper if upper.is_ascii_uppercase() => Ok(Token::Upper(upper)),
            lower if lower.is_ascii_lowercase() => Ok(Token::Lower(lower)),
            _ => Err(format!("Invalid char {}", c)),
        }
    }
}

//look-behind required for the scan
#[derive(PartialEq, Eq)]
enum TokenAccum {
    Empty,               //Null Accumulation
    Num(usize),          //multiplier
    Suffix(char, usize), //lowercase character + number prior
}

impl ops::AddAssign for TokenAccum {
    fn add_assign(&mut self, rhs: TokenAccum) {
        *self = if let TokenAccum::Num(base) = self {
            if let TokenAccum::Num(digit) = rhs {
                let base_digit_count = format!("{}", base).len() as u32;
                TokenAccum::Num((digit * 10usize.pow(base_digit_count)) + *base)
            } else {
                rhs
            }
        } else {
            rhs
        };
    }
}

//Processable item
#[derive(Debug, PartialEq, Eq)]
enum Elem {
    Skip,
    Chemical(String, usize),
    MultGroupStart(BracketType, usize), //multiplier for group start
    MultGroupEnd(BracketType),          //multiplier for group end
}

struct ChemNames(HashMap<char, Vec<char>>);

impl ChemNames {
    pub fn new() -> ChemNames {
        let mut hm = HashMap::with_capacity(25);
        hm.insert('A', vec!['c', 'g', 'l', 'm', 'r', 's', 't', 'u']);
        hm.insert('B', vec!['_', 'a', 'e', 'h', 'i', 'k', 'r']);
        hm.insert(
            'C',
            vec!['_', 'a', 'd', 'e', 'f', 'l', 'm', 'n', 'o', 'r', 's', 'u'],
        );
        hm.insert('D', vec!['b', 's', 'y']);
        hm.insert('E', vec!['r', 's', 'u']);
        hm.insert('F', vec!['_', 'e', 'l', 'm', 'r']);
        hm.insert('G', vec!['a', 'd', 'e']);
        hm.insert('H', vec!['_', 'e', 'f', 'g', 'o', 's']);
        hm.insert('I', vec!['_', 'n', 'r']);
        hm.insert('K', vec!['_', 'r']);
        hm.insert('L', vec!['a', 'i', 'r', 'u', 'v']);
        hm.insert('M', vec!['c', 'd', 'g', 'n', 'o', 't']);
        hm.insert('N', vec!['_', 'a', 'b', 'd', 'e', 'h', 'i', 'o', 'p']);
        hm.insert('O', vec!['_', 'g', 's']);
        hm.insert('P', vec!['_', 'a', 'b', 'd', 'm', 'o', 'r', 't', 'u']);
        hm.insert('R', vec!['a', 'b', 'e', 'f', 'g', 'h', 'n', 'u']);
        hm.insert('S', vec!['_', 'b', 'c', 'e', 'g', 'i', 'm', 'n', 'r']);
        hm.insert('T', vec!['a', 'b', 'c', 'e', 'h', 'i', 'l', 'm', 's']);
        hm.insert('U', vec!['_']);
        hm.insert('V', vec!['_']);
        hm.insert('W', vec!['_']);
        hm.insert('X', vec!['e']);
        hm.insert('Y', vec!['_', 'b']);
        hm.insert('Z', vec!['n', 'r']);
        ChemNames(hm)
    }

    pub fn exists(&self, prefix: char, suffix: char) -> bool {
        self.0
            .get(&prefix)
            .and_then(|v| v.binary_search(&suffix).ok())
            .is_some()
    }
}

fn accumulate_tokens(
    name_reg: &ChemNames,
    prev: &mut TokenAccum,
    tok: Token,
) -> Result<Elem, String> {
    //whitelist guard
    if let TokenAccum::Suffix(_, _) = *prev {
        match tok {
            Token::Upper(_) => {}
            _ => return Err(String::from("Invalid Syntax")),
        }
    }

    match tok {
        Token::Num(n) => {
            *prev += TokenAccum::Num(n);
            Ok(Elem::Skip)
        }
        Token::Lower(c) => {
            let mult = match *prev {
                TokenAccum::Num(n) => n,
                _ => 1,
            };
            *prev = TokenAccum::Suffix(c, mult);
            Ok(Elem::Skip)
        }
        Token::Upper(prefix) => {
            let (suffix, mult) = match *prev {
                TokenAccum::Suffix(c, n) => (c, n),
                TokenAccum::Num(n) => ('_', n),
                _ => ('_', 1),
            };
            *prev = TokenAccum::Empty;

            let mut s = format!("{}", prefix);
            if suffix != '_' {
                s.push(suffix);
            }
            if name_reg.exists(prefix, suffix) {
                Ok(Elem::Chemical(s, mult))
            } else {
                Err(format!("Invalid chemical name {}", s))
            }
        }
        Token::CloseBracket(b_type) => {
            //as we're reading backwards, this is at "start"
            let mult = match *prev {
                TokenAccum::Num(m) => m,
                _ => 1,
            };
            *prev = TokenAccum::Empty;
            Ok(Elem::MultGroupStart(b_type, mult))
        }
        Token::OpenBracket(b_type) => {
            if let TokenAccum::Num(_) = *prev {
                Err(String::from("Invalid Syntax"))
            } else {
                *prev = TokenAccum::Empty;
                Ok(Elem::MultGroupEnd(b_type))
            }
        }
    }
}

fn proc_brackets(
    bracket_mult: &mut (Vec<BracketType>, Vec<usize>),
    e: Elem,
) -> Result<Elem, String> {
    match e {
        Elem::Chemical(s, n) => {
            let mult = bracket_mult.1.iter().product::<usize>();
            let mult_n = if mult > 0 { n * mult } else { n } as usize;
            Ok(Elem::Chemical(s, mult_n))
        }
        Elem::MultGroupStart(b_type, n) => {
            bracket_mult.0.push(b_type);
            bracket_mult.1.push(n);
            Ok(Elem::Skip)
        }
        Elem::MultGroupEnd(b_type) => {
            bracket_mult.1.pop();
            if let Some(bracket) = bracket_mult.0.pop() {
                if bracket == b_type {
                    Ok(Elem::Skip)
                } else {
                    Err(String::from("Mismatched Parentheses"))
                }
            } else {
                Err(String::from("Mismatched Parentheses"))
            }
        }
        _ => unreachable!(),
    }
}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let chem_names = ChemNames::new();

    let mut tokens = s
        .chars()
        .map(Token::try_from_char)
        .collect::<Result<Vec<Token>, ParseError>>()?;
    //first character case
    match tokens[0] {
        Token::Lower(_) | Token::Num(_) => return Err(String::from("Invalid Syntax")),
        _ => {}
    }
    //accumulate tokens in reverse
    tokens.reverse();
    let mut elems = tokens //map tokens -> elem
        .into_iter()
        .scan(TokenAccum::Empty, |prev, tok| {
            Some(accumulate_tokens(&chem_names, prev, tok))
        })
        .collect::<Result<Vec<Elem>, ParseError>>()?;
    elems = elems //filter elems down to chemicals
        .into_iter()
        .filter(|e| *e != Elem::Skip)
        .scan((Vec::new(), Vec::new()), |bracket_mult, elem| {
            Some(proc_brackets(bracket_mult, elem))
        })
        .collect::<Result<Vec<Elem>, ParseError>>()?;
    elems.reverse();

    let mut resmap =
        elems
            .into_iter()
            .filter(|e| *e != Elem::Skip)
            .fold(HashMap::new(), |mut map, mol| {
                if let Elem::Chemical(chem, mult) = mol {
                    map.entry(chem)
                        .and_modify(|n| {
                            *n += mult;
                        })
                        .or_insert(mult);
                }
                map
            });
    let res = resmap.drain().collect::<Molecule>();
    Ok(res)
}
