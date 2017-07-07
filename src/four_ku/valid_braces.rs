/*
Valid Braces

Write a function called validBraces that takes a string of braces, and determines if the order of the braces is valid. validBraces should return true if the string is valid, and false if it's invalid.

This Kata is similar to the Valid Parentheses Kata, but introduces four new characters. Open and closed brackets, and open and closed curly braces. Thanks to @arnedag for the idea!

All input strings will be nonempty, and will only consist of open parentheses '(' , closed parentheses ')', open brackets '[', closed brackets ']', open curly braces '{' and closed curly braces '}'.

What is considered Valid? A string of braces is considered valid if all braces are matched with the correct brace.
For example:
'(){}[]' and '([{}])' would be considered valid, while '(}', '[(])', and '[({})](]' would be considered invalid.

Examples:
validBraces( "(){}[]" ) => returns true
validBraces( "(}" ) => returns false
validBraces( "[(])" ) => returns false
validBraces( "([{}])" ) => returns true
*/

// solution 1

use std::collections::HashMap;

macro_rules! hash(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

fn valid_map(tokens: &Vec<char>) -> bool {
    // let mut map =  HashMap::new();
    // map.insert('{', '}');
    // map.insert('[', ']');
    // map.insert('(', ')');
    let mut map = hash!{'{' => '}', '[' => ']', '(' => ')'};
    let mut map_stack = Vec::new();
    let mut result: bool = false;

    for i in 0..tokens.len() {
        match tokens[i] {
            '{' | '[' | '(' => {
                map_stack.push(tokens[i]);
                result = false;
            },
            '}' | ']' | ')' => {
                match map_stack.pop() {
                    None => { result = false; },
                    Some(x) => {
                        match map.get(&x) {
                            None => { result = false; },
                            Some(val) => {
                                if tokens[i] == *val { result  = true } else {result = false};
                            }
                        }
                        println!("===result : {:?}", result);
                    },
                };
            },
            _ => { result = false}
        }
    }
    result
}

fn valid_braces(s: &str) -> bool {
    let string = s.to_string();
    let chars_arr: Vec<char> = string.chars().collect();
    valid_map(&chars_arr)
}

// solution 2

fn valid_braces_2(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
          match c {
              '(' => stack.push(c),
              ')' => if stack.pop() != Some('(') {return false;},
              '{' => stack.push(c),
              '}' => if stack.pop() != Some('{') {return false;},
              '[' => stack.push(c),
              ']' => if stack.pop() != Some('[') {return false;},
              _   => panic!("Invalid input")
          }
      }
      stack.is_empty()
}


// solution 3

fn valid_braces_3(s: &str) -> bool {
  let mut stack = Vec::new();
  for c in s.chars() { match c {
    '(' | '[' | '{' => stack.push(c),
    ')' if stack.last() == Some(&'(') => drop(stack.pop()),
    ']' if stack.last() == Some(&'[') => drop(stack.pop()),
    '}' if stack.last() == Some(&'{') => drop(stack.pop()),
    _ => return false,
  } }
  return stack.is_empty();
}

// solution 4

fn valid_braces_4(s: &str) -> bool {
  let mut br = vec![];
  for x in s.chars() {
    match x {
      '(' => br.push(')'),
      '[' => br.push(']'),
      '{' => br.push('}'),
      _ => if Some(x) != br.pop() { return false; },
    }
  }
  br.is_empty()
}

// for codewars test

#[test]
fn basic_tests() {
  expect_true("()");
  expect_false("[(])");
  expect_false("())({}}{()][][");
}
