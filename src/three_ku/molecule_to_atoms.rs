/*
For a given chemical formula represented by a string, count the number of atoms of each element contained in the molecule and return an object.

For example:

parse_molecule("H2O");           // water
// Ok([("H", 2), ("O", 1)])

parse_molecule("Mg(OH)2");       // magnesium hydroxide
// Ok([("Mg", 1), ("O", 2), ("H", 2)]

parse_molecule("K4[ON(SO3)2]2"); // Fremy's salt
// Ok([("K", 4), ("O", 14),("N", 2),("S", 4)])

parse_molecule("pie")
// Err(ParseError)
As you can see, some formulas have brackets in them. The index outside the brackets tells you that you have to multiply count of each atom inside the bracket on this index. For example, in Fe(NO3)2 you have one iron atom, two nitrogen atoms and six oxygen atoms.

Note that brackets may be round, square or curly and can also be nested. Index after the braces is optional.

example:

H2O
B2H6
C6H12O6
Mo(CO)6
Mg(OH)2
Fe(C5H5)2
(C5H5)Fe(CO)2CH3
Pd[P(C6H5)3]4
K4[ON(SO3)2]2
As2{Be4C5[BCo3(CO2)3]2}4Cu5
{[Co(NH3)4(OH)2]3Co}(SO4)3

*/


// solution 1

use std::collections::HashMap;

#[derive(Debug)]
pub struct ParseError {
    message: String
}

#[derive(Debug)]
pub struct LoopMapError {
    err: bool
}

// type Atom = (String, usize);
// type Molecule = Vec<Atom>;

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let v = match tokenizing(s){
        Ok(v) => v,
        _ => vec![],
    };
    if v.is_empty() {
        return Err(ParseError{message: "Not a valid molecule".to_string()})
    }

    let mut level_1_numer = 1;
    let mut level_2_numer = 1;
    let mut level_3_numer = 1;

    let brace_map = loop_map(&v);

    match brace_map {
        Ok(map) => {
            let mut result_map = HashMap::new();
            for i in 0..v.len(){
                match v[i].as_str() {
                    "{" => {
                        match map.get(&i) {
                            Some(number) => level_1_numer = *number,
                            _ => {}
                        }
                    },
                    "}" => { level_1_numer = 1; },
                    "[" => {
                        match map.get(&i) {
                            Some(number) => level_2_numer = *number,
                            _ => {}
                        }
                    },
                    "]" => { level_2_numer = 1; },
                    "(" => {
                        match map.get(&i) {
                            Some(number) => level_3_numer = *number,
                            _ => {}
                        }
                    },
                    ")" => {
                        level_3_numer = 1;
                    },
                    _ => {
                        let total_number = level_1_numer * level_2_numer * level_3_numer;
                        if !string_is_digit(v[i].clone()){
                            if i+1 <= v.len()-1 && string_is_digit(v[i+1].clone()){
                                if result_map.contains_key(&v[i]) {
                                    let val: usize ;
                                    {
                                        val = *result_map.get_mut(&v[i]).unwrap();
                                    }
                                    result_map.insert(v[i].clone(), val + string_to_usize(v[i+1].clone())*total_number);
                                }else{
                                    result_map.insert(v[i].clone(), string_to_usize(v[i+1].clone())*total_number);
                                }
                            }else{
                                if result_map.contains_key(&v[i]) {
                                    let val: usize;
                                    {
                                        val = *result_map.get_mut(&v[i]).unwrap();
                                    }
                                    result_map.insert(v[i].clone(), val + 1*total_number);
                                }else{
                                    result_map.insert(v[i].clone(), 1*total_number);
                                }
                            }
                        }
                    }
                }
            }

            let mut vec = vec![];
            let mut vec2 = vec![];

            for i in v{
                if !vec.contains(&i){ vec.push(i); }
            }

            for e in vec {
                if result_map.contains_key(&e){
                    vec2.push((e.clone(), result_map.get(&e).unwrap()));
                }
            }
            vec2
        },
        Err(e) => {
            ParseError{ message: "Mismatched parenthesis"}
        },
    }
}

/*
利用堆栈来判断对应的括号: {},[], ()
因为有层级关系，所以将花括号定为一级、中括号定位二级、圆括号定位三级
利用此堆栈来得到层级对应的数，比如：{[Co(NH3)4(OH)2]3Co}(SO4)3
对应于： {[()4()2]3}()3，则返回：Ok({ 0: 1, 1: 3, 3: 4, 9: 2, 18: 3})

*/
fn loop_map(tokens: &Vec<String>) -> Result<HashMap<usize, usize>, LoopMapError> {
    let mut map =  HashMap::new();
    let mut map_stack = Vec::new();
    let (mut lv1_before_brace, mut lv1_after_brace) = (false, false);
    let (mut lv2_before_brace, mut lv2_after_brace) = (false, false);
    let (mut lv3_before_brace, mut lv3_after_brace) = (false, false);


    for i in 0..tokens.len()-1 {
        match tokens[i].as_str() {
            "{" => {
                map_stack.push(i);
                lv1_before_brace = true;
            },
            "[" => {
                map_stack.push(i);
                lv2_before_brace = true;
            },
            "(" => {
                map_stack.push(i);
                lv3_before_brace = true;
            },
            "}" => {
                if lv1_before_brace { lv1_after_brace = true }
                let start = match map_stack.pop() {
                    None => break,
                    Some(x) => x,
                };

                if string_is_digit(tokens[i+1].clone()){
                    map.insert(start, string_to_usize(tokens[i+1].clone()));
                }else{
                    map.insert(start, 1);
                }
            },
            "]" => {
                if lv2_before_brace { lv2_after_brace = true }
                let start = match map_stack.pop() {
                    None => break,
                    Some(x) => x,
                };
                if string_is_digit(tokens[i+1].clone()){
                    map.insert(start, string_to_usize(tokens[i+1].clone()));
                }else{
                    map.insert(start, 1);
                }
            },
            ")" => {
                if lv3_before_brace { lv3_after_brace = true }
                let start = match map_stack.pop() {
                    None => break,
                    Some(x) => x,
                };

                if string_is_digit(tokens[i+1].clone()){
                    map.insert(start, string_to_usize(tokens[i+1].clone()));
                }else{
                    map.insert(start, 1);
                }
            },
            _ => {}
        }
    }

    if (lv1_before_brace && lv1_after_brace) || (lv2_before_brace && lv2_after_brace) || (lv3_before_brace && lv3_after_brace){
        return Ok(map);
    }else if lv1_before_brace == false && lv2_before_brace == false && lv3_before_brace == false{
        return Ok(map);
    }else{
        return Err(LoopMapError{err: false});
    }
}

/*
分词，将化学式字符串分解为Vector
"K4[ON(SO3)2]2"  => ["K", "4", "[", "O", "N", "(", "S", "O", "3", ")", "2", "]", "2"]
*/
fn tokenizing(str: &str) -> Result<Vec<String>, LoopMapError>{
    let string = str.to_string();
    let chars_arr: Vec<char> = string.chars().collect();
    println!("chars{:?}", chars_arr);
    let mut vec = vec![];
    let mut i = 0;
    let mut valid = false;

    while i < chars_arr.len()  {
        if is_uppercase(chars_arr[i]) { valid = true;}
        if i+1 <= chars_arr.len()-1 && is_uppercase(chars_arr[i]) && is_downcase(chars_arr[i+1]) {
            let str = char_join_char(chars_arr[i], chars_arr[i+1]);
            vec.push(str);
            i += 2;
        }else if i+1 <= chars_arr.len()-1 && is_digit(chars_arr[i]) && is_digit(chars_arr[i+1]){
            let str = char_join_char(chars_arr[i], chars_arr[i+1]);
            vec.push(str);
            i += 2;
        }else {
            vec.push(chars_arr[i].to_string());
            i += 1;
        }

    }
    if valid {
        return Ok(vec);
    }else {
        return Err(LoopMapError{err: false});
    }
}


fn is_uppercase(c: char) -> bool {
    (c as u8) >= 65 && (c as u8) <= 90
}

fn is_downcase(c: char) -> bool {
    (c as u8) >= 97 && (c as u8) <= 122
}

fn is_digit(c: char) -> bool {
    (c as u8) >= 48 && (c as u8) <= 57
}

fn string_to_usize(str: String) -> usize {
    str.parse::<usize>().unwrap()
}

fn string_is_digit(str: String) -> bool {
    match str.parse::<usize>() {
        Ok(v) => true,
        _ => false,
    }
}

fn char_join_char(c1: char, c2: char) -> String{
    let mut str = String::from("");
    str.push(c1);
    str.push(c2);
    str
}


// solution 2

// type Atom = (String, usize);
// type Molecule = Vec<Atom>;

pub fn parse_molecule_2(s: &str) -> Result<Molecule, ParseError> {
    Parser::new(s).molecule()
        .map(normalize)
}

struct Parser<'a> {
    chars: std::str::Chars<'a>,
    current: Option<char>,
}
impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        let mut chars = s.chars();
        let current = chars.next();
        Parser{
            chars: chars,
            current: current,
        }
    }
    pub fn molecule(&mut self) -> Result<Molecule, ParseError> {
        let mut mol = vec![];
        while let Some(result) = self.group() {
            let group = try!(result);
            let count = self.number().unwrap_or(1);
            for g in group {
                mol.push((g.0, count * g.1));
            }
        }
        if mol.len() > 0 { Ok(mol) } else { Err(ParseError::EmptyMolecule) }
    }
    pub fn group(&mut self) -> Option<Result<Molecule, ParseError>> {
        if let Some(atom) = self.atom() {
            Some(Ok(vec![(atom, 1)]))
        } else if let Some(close) = self.open_paren() {
            Some(self.molecule().and_then(|mol| {
                self.close_paren(close)
                    .ok_or(ParseError::UnbalancedParens)
                    .and(Ok(mol))
            }))
        } else {
            None
        }
    }
    pub fn atom(&mut self) -> Option<String> {
        self.accept(|c| c.is_uppercase()).map(|c| {
            let mut name = vec![c];
            while let Some(c) = self.accept(|c| c.is_lowercase()) {
                name.push(c);
            }
            name.into_iter().collect()
        })
    }
    pub fn number(&mut self) -> Option<usize> {
        self.parse(|c| c.to_digit(10)).map(|mut number| {
            while let Some(d) = self.parse(|c| c.to_digit(10)) {
                number = number * 10 + d;
            }
            number as usize
        })
    }
    pub fn open_paren(&mut self) -> Option<char> {
        self.parse(|c| match c {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            _ => None,
        })
    }
    pub fn close_paren(&mut self, expected: char) -> Option<char> {
        self.accept(|c| c == expected)
    }
    pub fn accept<F>(&mut self, pred: F) -> Option<char>
        where F: FnOnce(char) -> bool
    {
        self.parse(|c| if pred(c) { Some(c) } else { None })
    }
    pub fn parse<F, T>(&mut self, map: F) -> Option<T>
        where F: FnOnce(char) -> Option<T>
    {
        self.current.and_then(map).and_then(|t| {
            self.current = self.chars.next();
            Some(t)
        })
    }
}

fn normalize(mut unsorted: Molecule) -> Molecule {
    unsorted.sort();
    let mut iter = unsorted.into_iter();
    let mut sorted = vec![iter.next().unwrap()];
    while let Some((atom, count)) = iter.next() {
        let prev = sorted.len() - 1;
        if atom == sorted[prev].0 {
            sorted[prev].1 += count;
        } else {
            sorted.push((atom, count));
        }
    }
    sorted
}

use std::error::Error;

#[derive(Debug)]
pub enum ParseError {
    EmptyMolecule,
    UnbalancedParens,
}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "parse error: {}", self.description())
    }
}
impl Error for ParseError {
    fn description(&self) -> &str {
        use ParseError::*;
        match *self {
            EmptyMolecule => "Empty molecule",
            UnbalancedParens => "Unbalanced parenthesis",
        }
    }
}

// solution 3
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub struct ParseError {}

#[derive(Debug,PartialEq)]
enum Token {
  OpenParen(char),
  CloseParen(char),
  Number(usize),
  Atom(String),
}

fn tokenize_number(it : &mut Peekable<Chars>) -> usize {
    let mut nums : Vec<char> = vec![];
    while let Some(&ch) = it.peek() {
        if ch.is_numeric() {
            nums.push(it.next().unwrap());
        } else {
            break;
        }
    }
    nums.into_iter().collect::<String>().parse().unwrap()
}

fn tokenize_atom(it : &mut Peekable<Chars>) -> String {
    let mut res : Vec<char> = vec![it.next().unwrap()];
    if let Some(&ch) = it.peek() {
        if ch.is_lowercase() {
            res.push(it.next().unwrap());
        }
    }
    res.into_iter().collect::<String>()
}

// Tokenize:
// Mg(O)2 ->
// [Atom("Mg"), OpenParam('('), Atom("O"), CloseParam("("), Number(2)]
fn tokenize(s: &str) -> Result<Vec<Token>,ParseError> {
    let mut it = s.chars().peekable();
    let mut v: Vec<Token> = vec![];
    loop {
        match it.peek() {
            None => break,
            Some(&c) => v.push(match c {
                'A' ... 'Z' => {
                    Token::Atom(tokenize_atom(&mut it))
                },
                '0' ... '9' => {
                    Token::Number(tokenize_number(&mut it))
                },
                '{'|'['|'(' => {
                    it.next();
                    Token::OpenParen(c)
                },
                '}' => {
                    it.next();
                    Token::CloseParen('{')
                },
                ']' => {
                    it.next();
                    Token::CloseParen('[')
                },
                ')' => {
                    it.next();
                    Token::CloseParen('(')
                },
                _ => return Err(ParseError{}),
            }),
        }
    }
    Ok(v)
}

fn parse_until_matching_paren<T>(it : &mut Peekable<T>, p : char)
                                 -> Result<Molecule, ParseError>
                                 where T : Iterator<Item=Token>+Sized {
    let mut res : Molecule = vec![];
    loop {
        match it.next() {
            Some(Token::OpenParen(p)) => {
                let mut r = parse_until_matching_paren(it, p);
                if let Err(e) = r { return Err(e) }
                let mut r = r.unwrap();
                if let Some(&Token::Number(n)) = it.peek() {
                    it.next();
                    multiply(&mut r, n);
                }
                res.append(&mut r);
            },
            Some(Token::Atom(a)) => {
                if let Some(&Token::Number(n)) = it.peek() {
                    it.next();
                    res.push((a, n));
                } else {
                    res.push((a, 1));
                }
            },
            Some(Token::CloseParen(p2)) => {
                if p != p2 { return Err(ParseError{}); }
                break;
            },
            _ => return Err(ParseError{}),
        }
    }
    Ok(res)
}

fn multiply(v : &mut Vec<Atom>, n : usize) {
    for c in v.iter_mut() {
      c.1 *= n;
    }
}

pub fn parse_molecule_3(s: &str) -> Result<Molecule, ParseError> {
  let mut s = String::from(s);
  s += ")";
  let s : &str = &s;
  let tokens = tokenize(s);
  if let Err(e) = tokens {
      return Err(e);
  }
  let tokens = tokens.unwrap();

  let mut it = tokens.into_iter().peekable();
  let mut res = parse_until_matching_paren(&mut it, '(');
  if let Err(e) = res { return Err(e) }
  if let Some(x) = it.peek() { return Err(ParseError{}); }
  let mut res = res.unwrap();

  res.sort_by(|&(ref a1, n1), &(ref a2, n2)| a1.cmp(&a2));

  let mut new_res : Molecule = vec![];
  let mut o = res.pop().unwrap(); // Assume there is an atom...
  while let Some(n) = res.pop() {
     if o.0 == n.0 {
         o.1 += n.1
     } else {
         new_res.push(o);
         o = n;
     }
  }
  new_res.push(o);
  Ok(new_res)
}


// solution 4

use std::collections::HashMap;

#[derive(Debug)]
pub struct ParseError {}

fn is_open_bracket(&c: &char) -> bool {
    (c == '{') | (c == '[') | (c == '(')
}
fn is_closed_bracket(&c: &char) -> bool {
    (c == '}') | (c == ']') | (c == ')')
}
fn flipped_bracket(&c: &char) -> char {
    match c {
        '{' => '}',
        '[' => ']',
        '(' => ')',
        _ => unreachable!(),
    }
}
fn char_to_str(c: &[char]) -> String {
    c.iter().cloned().collect::<String>()
}
fn balanced_braces(s: &[char]) -> bool {
    let mut stack = Vec::new();
    for c in s.iter() {
        if is_open_bracket(c) {
            stack.push(flipped_bracket(c));
        } else if is_closed_bracket(c) {
            match stack.pop() {
                Some(d) if d == *c => (),
                Some(d) => {return false},
                None => {return false},
            }
        }
    }
    stack.is_empty()
}

#[derive(Debug,PartialEq)]
enum Component <'a> {
    Element(&'a [char]),
    Group(&'a [char]),
}
use Component::*;

struct ChemStr<'a> {
    data: &'a [char],
    idx: usize,
    multiplier: usize,
}
impl<'a> ChemStr<'a> {
    fn new(s: &'a [char], n: usize) -> ChemStr {
        ChemStr{data: s, idx: 0, multiplier: n}
    }
    fn next_char(&mut self) -> Option<&'a char> {
        self.idx += 1;
        self.data.get(self.idx - 1)
    }
    fn peek_char(&self) -> Option<&'a char> {
        self.data.get(self.idx)
    }
}
impl<'a> Iterator for ChemStr<'a> {
    type Item = Result<(Component<'a>, usize), ParseError>;
    fn next(&mut self) -> Option<Result<(Component<'a>, usize), ParseError>> {
        let next_char = self.next_char();
        if next_char.is_none() {return None};

        let out: Component = match next_char.unwrap() {
            open if is_open_bracket(&open) => {
                let start_pos = self.idx;
                let target_bracket = flipped_bracket(&open);
                while let Some(&c) = self.next_char() {
                    if c == target_bracket {break}
                }
                Component::Group(&self.data[start_pos..self.idx-1])
            },
            c if c.is_alphabetic() & c.is_uppercase() => {
                let n = match self.peek_char() {
                    Some(&c) if c.is_alphabetic() & c.is_lowercase() => {self.idx+=1; 2},
                    _ => {1}
                };
                Component::Element(&self.data[self.idx-n..self.idx])
            },
            _ => {return Some(Err(ParseError{}))}
        };

        // Atom count?
        let start_pos = self.idx;
        while let Some(c) = self.peek_char() {
            if !c.is_numeric() {break};
            self.idx += 1;
        }
        let count = char_to_str(&self.data[start_pos..self.idx]).parse().unwrap_or(1);
        Some(Ok((out, self.multiplier * count)))
    }
}

fn parse_molecule_4(s: &str) -> Result<Molecule, ParseError> {
    let s: Vec<char> = s.chars().collect();
    if !balanced_braces(&s) {return Err(ParseError{})}
    let mut stack = Vec::new();
    stack.push(ChemStr::new(&s, 1));
    let mut counts = HashMap::new();

    while let Some(mut cs) = stack.pop() {
        while let Some(comp) = cs.next() {
            match comp? {
                (Element(e), n) => {
                    let count = counts.entry(e).or_insert(0);
                    *count += n;
                },
                (Group(g), multiplier) => {
                    stack.push(ChemStr::new(&g, multiplier));
                },
            };
        }
    };
    Ok(counts.iter().map(|(&k, &v)| (char_to_str(k), v)).collect())
}

fn compare_test_str(s: &str, expected: Vec<(&str, usize)>) -> () {
    let s: Vec<char> = s.chars().collect();
    let cs = ChemStr::new(&s, 1);
    let parsed: Vec<Result<_,_>> = cs.collect();
    //let parsed: Vec<(Component, usize)> = parsed.iter().flat_map(|x| x).collect();
    let parsed: Vec<(String, usize)> = parsed.iter().flat_map(|x| x).map(|&(ref comp, n)| {
        match comp {
            &Element(e) => (char_to_str(e), n),
            &Group(e) => (char_to_str(e), n)
        }}).collect();
    let expected: Vec<(String, usize)> = expected.iter().map(|&(s, n)| (s.to_string(), n)).collect();
    assert_eq!(parsed, expected);
}


// for test

mod molecules {
  assert_parse!("H", [("H",1)], hydrogen);
  assert_parse!("O2", [("O",2)], oxygen);
  assert_parse!("H2O", [("H",2),("O",1)], water);
  assert_parse!("Mg(OH)2", [("Mg",1),("O",2),("H",2)], magnesium_hydroxide);
  assert_parse!("K4[ON(SO3)2]2", [("K",4),("O",14),("N",2),("S",4)], fremys_salt);
}

#[test]
fn errors() {
  assert_fail("pie", "Not a valid molecule");
  assert_fail("Mg(OH", "Mismatched parenthesis");
  assert_fail("Mg(OH}2", "Mismatched parenthesis");
}


// 与题无关
// #[derive(Debug)]
// pub enum Element {
//     B, C, H, O, N, K, S, P,
//     As, Be, Co, Cu, Fe, Mg,Mo, Pd,
// }
//
// impl Element {
//     // Element::from_str("B")
//     pub fn from_str(s: &str) -> Option<Element> {
//         match s {
//             "B" => Some(Element::B),
//             "C" => Some(Element::C),
//             "H" => Some(Element::H),
//             "O" => Some(Element::O),
//             "N" => Some(Element::N),
//             "K" => Some(Element::K),
//             "S" => Some(Element::S),
//             "P" => Some(Element::P),
//             "As" => Some(Element::As),
//             "Be" => Some(Element::Be),
//             "Co" => Some(Element::Co),
//             "Cu" => Some(Element::Cu),
//             "Fe" => Some(Element::Fe),
//             "Mg" => Some(Element::Mg),
//             "Mo" => Some(Element::Mo),
//             "Pd" => Some(Element::Pd),
//             _ => None,
//         }
//     }
//     // Element::B.as_str()
//     pub fn as_str(&self) -> &'static str {
//         match *self {
//             Element::B => "B",
//             Element::C => "C",
//             Element::H => "H",
//             Element::O => "O",
//             Element::N => "N",
//             Element::K => "K",
//             Element::S => "S",
//             Element::P => "P",
//             Element::As => "As",
//             Element::Be => "Be",
//             Element::Co => "Co",
//             Element::Cu => "Cu",
//             Element::Fe => "Fe",
//             Element::Mg => "Mg",
//             Element::Mo => "Mo",
//             Element::Pd => "Pd",
//             _ => None,
//         }
//     }
// }
