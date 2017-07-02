/*
Inspired from real-world Brainf**k, we want to create an interpreter of that language which will support the following instructions (the machine memory or 'data' should behave like a potentially infinite array of bytes, initialized to 0):

> increment the data pointer (to point to the next cell to the right).
< decrement the data pointer (to point to the next cell to the left).
+ increment (increase by one, truncate overflow: 255 + 1 = 0) the byte at the data pointer.
- decrement (decrease by one, treat as unsigned byte: 0 - 1 = 255 ) the byte at the data pointer.
. output the byte at the data pointer.
, accept one byte of input, storing its value in the byte at the data pointer.
[ if the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
] if the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.
The function will take in input...

the program code, a string with the sequence of machine instructions,
the program input, a string, eventually empty, that will be interpreted as an array of bytes using each character's ASCII code and will be consumed by the , instruction
... and will return ...

the output of the interpreted code (always as a string), produced by the . instruction.
*/


// the function ez_vec takes a static string and a terminating byte and returns an owned Vec<u8> for convenience
// Without it, character-based tests are a pain

/*
>: 指针右移一位。
<: 指针左移一位。
+: 当前指针所指的内存值加一，以 255 为界，溢出为 0，即 255 + 1 = 0。
-: 当前指针所指的内存值减一，以 0 为界，溢出为 255，即 0 - 1 = 255。
.: 输出当前指针所指的值，即输出该值 ASCII 码所对应的字符。
,: 从输入取一个字符转为 ASCII 码存入当前指针所指的内存。
[: 若当前指针所指的值为 0，则命令跳到该 [ 匹配的结束 ] 符号位置的下一位置的指令。
]: 若当前指针所指的值不为 0，则指令向前跳到该 ] 匹配到的 [ 符号位置的下一位置的指令。

注： https://fatiherikli.github.io/brainfuck-visualizer/

Rust中 u8类型：

加一，以 255 为界，溢出为 0，即 255 + 1 = 0，使用wrapping_add方法
减一，以 0 为界，溢出为 255，即 0 - 1 = 255，使用wrapping_sub方法

*/

// solution 1
use std::collections::HashMap;

fn loop_map(tokens: &Vec<char>) -> (HashMap<usize,usize>, HashMap<usize,usize>) {
    let (mut forward_map, mut backward_map) = (HashMap::new(), HashMap::new());
    let mut map_stack = Vec::new();

    // map_stack栈结构用来记录`[`和`]`两头的索引，制造循环
    // example: `[-<<+>>]`
    // 碰到`[`推入栈（map_stack）中
    // 碰到`]`，将栈顶的起始下标弹出，赋值给start
    //    定义forward_map为： {1, 8}
    //    定义backward_map为： {8, 1}
    for i in 0..tokens.len() {
        match tokens[i] {
            '[' => map_stack.push(i),
            ']' => {
                let start = match map_stack.pop() {
                    None => break,
                    Some(x) => x,
                };
                forward_map.insert(start, i);
                backward_map.insert(i, start);
            }
            _ => {}
        }
    }
    return (forward_map, backward_map)
}

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut cells: Vec<u8> = vec![0; 30]; // 初始化内存单元格
    let tokens = code.chars().collect();
    let (forward_map, backward_map) = loop_map(&tokens);
    let mut code_pointer = 0; // 当前代码的位置
    let mut cell_pointer = 0; // 当前单元格的位置
    let mut input_pointer = 0; // 当前输入的位置指针
    let mut output: Vec<u8> = vec![]; // 记录输出

    while code_pointer < tokens.len() {
        match tokens[code_pointer] {
            '+' => cells[cell_pointer] = cells[cell_pointer].wrapping_add(1), // 当前单元格内存值加1
            '-' => cells[cell_pointer] = cells[cell_pointer].wrapping_sub(1), // 当前单元格内存值减1
            '>' => cell_pointer += 1,        // 单元格指针向右移动一位
            '<' => cell_pointer -= 1,        // 单元格指针向左移动一位
            '[' => {
                if cells[cell_pointer] == 0 { // 若当前指针所指的值为 0，
                                              // 则命令跳到该 [ 匹配的结束 ] 符号位置的下一位置的指令
                    match forward_map.get(&code_pointer) {
                        Some(position) => code_pointer = *position,
                        _ => {}
                    }
                }
            },
            ']' => {
                if cells[cell_pointer] != 0 { // 若当前指针所指的值不为 0，
                                              // 则指令向前跳到该 ] 匹配到的 [ 符号位置的下一位置的指令
                    match backward_map.get(&code_pointer) {
                        Some(position) => code_pointer = *position,
                        _ => {}
                    }
                }
            },
            '.' => output.push(cells[cell_pointer]),
            ',' => {
                if input_pointer <= input.len() {
                    cells[cell_pointer] = input[input_pointer];
                }
                input_pointer += 1;

            },
            _ => {}
        }
        code_pointer += 1;
    }
    output
}

// solution 2

fn brain_luck_2(code: &str, input: Vec<u8>) -> Vec<u8> {
  let code = code.as_bytes();
  let mut input = input.into_iter();
  let mut output = vec![];
  let mut data = [0u8; 5000];
  let mut cp = 0;
  let mut dp = 0;
  while cp < code.len() {
    match code[cp] {
      b'>' => dp += 1,
      b'<' => dp -= 1,
      b'+' => data[dp] = data[dp].wrapping_add(1),
      b'-' => data[dp] = data[dp].wrapping_sub(1),
      b'.' => output.push(data[dp]),
      b',' => data[dp] = input.next().expect("input"),
      b'[' if data[dp] == 0 => cp += jump(code[cp..].iter()),
      b']' if data[dp] != 0 => cp -= jump(code[0..cp + 1].iter().rev()),
      _ => {},
    }
    cp += 1;
  }
  output
}

fn jump<'a, I: 'a>(code: I) -> usize
  where I: Iterator<Item=&'a u8>
{
  let mut n = 0;
  for (i, &c) in code.enumerate() {
    if c == b'[' { n += 1; }
    if c == b']' { n -= 1; }
    if n == 0 {
      return i;
    }
  }
  unreachable!();
}

// solution 3
// 面向对象思维
use std::str;

fn brain_luck_3(code: &str, mut input: Vec<u8>) -> Vec<u8> {
  // your solution here
  let mut data = Data::new();
  let code = code.as_bytes();
  let mut curr = 0;
  let mut output = vec![];

  while curr < code.len() {
      match code[curr] {
          b'>' => data.move_right(),
          b'<' => data.move_left(),
          b'+' => data.inc(),
          b'-' => data.dec(),
          b'.' => data.put(&mut output),
          b',' => data.set(input.remove(0)),
          b'[' => {
              if data.get() == 0 {
                  let mut indent = 0;
                  for i in curr+1.. {
                      match code[i] {
                          b'[' => indent += 1,
                          b']' => if indent == 0 { curr = i; break } else { indent -= 1 },
                          _ => {}
                      }
                  }
              }
          }
          b']' => {
              if data.get() != 0 {
                  let mut indent = 0;
                  for i in (0..curr).rev() {
                      match code[i] {
                          b']' => indent += 1,
                          b'[' => if indent == 0 { curr = i; break } else { indent -= 1 },
                          _ => {}
                      }
                  }
              }
          }
          _ => unreachable!()
      }
      curr += 1;
  }
  output
}

struct Data {
    curr: usize,
    buf: Vec<u8>
}

impl Data {
    fn new() -> Data {
        Data {
            curr: 0,
            buf: vec![0; 1]
        }
    }
    fn move_left(&mut self) {
        if self.curr > 0 {
            self.curr -= 1;
        }
    }
    fn move_right(&mut self) {
        self.curr += 1;
        if self.curr > self.buf.len() - 1 {
            self.buf.push(0);
        }
    }
    fn set(&mut self, input: u8) {
        self.buf[self.curr] = input;
    }
    fn inc(&mut self) {
        self.buf[self.curr] = self.buf[self.curr].wrapping_add(1);
    }
    fn dec(&mut self) {
        self.buf[self.curr] = self.buf[self.curr].wrapping_sub(1);
    }
    fn get(&self) -> u8 {
        self.buf[self.curr]
    }
    fn put(&self, output: &mut Vec<u8>) {
        output.push(self.buf[self.curr]);
    }
}

// solution 4
// 面向对象思维2
use std::collections::HashMap;
use std::fmt;

struct Memory {
    data: HashMap<i64, u8>,
    data_ptr: i64
}

enum Mode {
    Execute,
    SeekForward(u32),
    SeekBackward(u32)
}

struct State {
    instr_ptr: usize,
    loop_level: u32,
    mode: Mode
}

struct Machine<'a> {
    code: &'a str,
    mem: Memory,
    state: State,
    input: Vec<u8>,
    output: Vec<u8>,
}


impl Memory {
    fn new() -> Memory {
        Memory {
            data: HashMap::new(),
            data_ptr: 0
        }
    }

    fn get(&self) -> u8 {
        self.data.get(&self.data_ptr).map_or(0, |val| val.clone())
    }

    fn set(&mut self, val: u8) {
        self.data.insert(self.data_ptr, val);
    }

    fn inc(&mut self) {
        let mut val = self.get();

        val = if val == <u8>::max_value() {
            <u8>::min_value()
        } else {
            val + 1
        };

        self.set(val);
    }

    fn dec(&mut self) {
        let mut val = self.get();

        val = if val == <u8>::min_value() {
            <u8>::max_value()
        } else {
            val - 1
        };

        self.set(val);
    }

    fn inc_ptr(&mut self) {
        self.data_ptr += 1;
    }

    fn dec_ptr(&mut self) {
        self.data_ptr -= 1;
    }

}

impl State {
    fn new() -> State {
        State {
            instr_ptr: 0,
            loop_level: 0,
            mode: Mode::Execute
        }
    }
}

impl<'a> Machine<'a> {
    fn new(code: &str, mut input: Vec<u8>) -> Machine {
        input.reverse();
        Machine {
            code: code,
            mem: Memory::new(),
            state: State::new(),
            input: input,
            output: Vec::new()
        }
    }

    fn step(&mut self) {


        match self.state.mode {
            Mode::Execute => self.execute(),
            Mode::SeekBackward(target_level) => self.seek_backward(target_level),
            Mode::SeekForward(target_level) => self.seek_forward(target_level)
        }

    }

    fn execute(&mut self) {
        let mut auto_inc_instr_ptr = true;

        match self.get_instr() {
            '[' => {
                self.loop_start();
                auto_inc_instr_ptr = false;
            },
            ']' => {
                self.loop_end();
                auto_inc_instr_ptr = false;
            },
            '+' => self.mem.inc(),
            '-' => self.mem.dec(),
            '>' => self.mem.inc_ptr(),
            '<' => self.mem.dec_ptr(),
            '.' => self.write(),
            ',' => self.read(),
            c @ _ => panic!(format!("Unknown instruction '{}'", c))
        };

        if auto_inc_instr_ptr {
            self.state.instr_ptr += 1;
        }
    }

    fn seek_forward(&mut self, target_level: u32) {
        match self.get_instr() {
            '[' => {
                self.state.loop_level += 1;
            },
            ']' => {
                if self.state.loop_level - 1 == target_level {
                    self.state.mode = Mode::Execute;
                }

                self.state.loop_level -= 1;
            },
            _ => ()
        };

        self.state.instr_ptr += 1;
    }

    fn seek_backward(&mut self, target_level: u32) {

        match self.get_instr() {
            ']' => {
                self.state.loop_level += 1;
                self.state.instr_ptr -= 1;
            },
            '[' => {
                if self.state.loop_level - 1 == target_level {
                    self.state.mode = Mode::Execute;
                    self.state.instr_ptr += 1;
                } else {
                    self.state.loop_level -= 1;
                    self.state.instr_ptr -= 1;
                }
            },
            _ => {
                self.state.instr_ptr -= 1;
            }
        };



    }

    fn loop_start(&mut self) {
        self.state.mode = if self.mem.get() > 0 { Mode::Execute } else {
            Mode::SeekForward(self.state.loop_level)
        };

        self.state.loop_level += 1;
        self.state.instr_ptr += 1;
    }

    fn loop_end(&mut self) {
        if self.mem.get() == 0 {
            self.state.mode = Mode::Execute;
            self.state.instr_ptr += 1;
            self.state.loop_level -= 1;
        } else {
            self.state.mode = Mode::SeekBackward(self.state.loop_level - 1);
            self.state.instr_ptr -= 1;
        };
    }

    fn read(&mut self) {
        self.mem.set(self.input.pop().expect("Trying to read from exhausted input"));
    }

    fn write(&mut self) {
        self.output.push(self.mem.get());
    }

    fn get_instr(&self) -> char {
        self.code.as_bytes()[self.state.instr_ptr] as char
    }

    fn is_finished(&self) -> bool {
        self.state.instr_ptr >= self.code.len()
    }

    fn run(mut self) -> Vec<u8> {
        while !self.is_finished() {
            self.step();
        }

        self.output
    }
}

fn brain_luck_4(code: &str, input: Vec<u8>) -> Vec<u8> {
    Machine::new(code, input).run()
}



// for test

/*
use std::str;

fn main() {

    let s = String::from("hello");
    let mut bytes = s.into_bytes(); // String -> Vec<u8>

    {
      bytes[0] += 2 ;
      println!("{:?}", bytes);
    }

    {
      let str = str::from_utf8(&mut bytes); // Vec<u8> -> String
      println!("{:?}", str);
    }
}

*/



#[test]
fn example_tests() {
  // Echo until byte 255 encountered
  // assert_eq!(String::from_utf8(brain_luck(",+[-.,+]", ez_vec("Codewars", 255))).unwrap(), "Codewars");
  // Echo until byte 0 encountered
  // assert_eq!(String::from_utf8(brain_luck(",[.[-],]", ez_vec("Codewars", 0))).unwrap(), "Codewars");
  // Multiply two numbers
  assert_eq!(brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9]), vec![72]);
}
