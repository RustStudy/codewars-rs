/*
Write a function called repeatStr which repeats the given string string exactly n times.

repeatStr(6, "I") // "IIIIII"
repeatStr(5, "Hello") // "HelloHelloHelloHelloHello"
*/


/*

定义了一个宏，可以对一段代码执行固定的次数。

Usage:

times!(5, {
      let a = 42;
      println!("{:?}", a)
})
*/

macro_rules! times {
    ($times:expr, $body:expr) => {{
        for _ in 0..$times {
            $body
        }
    }};
}

fn repeat_str_1(src: &str, count: usize) -> String {
  let mut string = String::new();
  times!(count, {
      string += src
  });
  string
}

// solution2
// 主要是实现类似Ruby的写法： 5.times(|x| x + 2);
trait Time {
    fn times<F>(&self, mut closure: F) where F: FnMut() ;
}

impl Time for u32 {
    fn times<F>(&self, mut closure: F) where F: FnMut() {
        for _ in 0..*self {closure()}
    }
}

fn repeat_str(src: &str, count: u32) -> String {
  let mut string = String::new();
  count.times(|| string += src);
  string
}

#[test]
fn example_tests() {
  assert_eq!(repeat_str("a", 4), "aaaa");
  assert_eq!(repeat_str("hello ", 3), "hello hello hello ");
  assert_eq!(repeat_str("abc", 2), "abcabc");
}
