// Bleatrix Trotter the sheep has devised a strategy that helps her fall asleep faster. First, she picks a number N. Then she starts naming N, 2 × N, 3 × N, and so on.
//
// Whenever she names a number, she thinks about all of the digits in that number. She keeps track of which digits (0, 1, 2, 3, 4, 5, 6, 7, 8, and 9) she has seen at least once so far as part of any number she has named. Once she has seen each of the ten digits at least once, she will fall asleep.
//
// Bleatrix must start with N and must always name (i + 1) × N directly after i × N.
//
// For example, suppose that Bleatrix picks N = 1692. She would count as follows:
//
// N = 1692. Now she has seen the digits 1, 2, 6, and 9.
// 2N = 3384. Now she has seen the digits 1, 2, 3, 4, 6, 8, and 9.
// 3N = 5076. Now she has seen all ten digits, and falls asleep.
// The purpose of this kata is to return the last number Bleatrix Trotter sees before falling asleep.
//
// Input
//
// Will always be positive integer or zero
// Output
//
// The last number Bleatrix Trotter sees or "INSOMNIA" (-1 in Rust and C++) if she will count forever
// Please note, this challenge is not my idea. It's from [Google Code Jam 2016](https://code.google.com/codejam/contest/6254486/dashboard)
//
//

// solution1
fn trotter(n: i32) -> i32 {
    if n == 0 {
        return -1;
    }
    let (mut i, mut vec) = (1, vec![]);

    loop {
        for c in (i * n).to_string().chars() {
            if !vec.contains(&c) {
                vec.push(c);
            }
        }
        if vec.len() == 10 {
            return i * n;
        }
        i += 1;
    }
}


// solution 2
// 利用二进制位 saw : 1_111_111_111
// bleatrix % 10 取个位数
// 1 << 个位数，代表将位左移相应位数
//  saw |= 1 << bleatrix % 10; 代表每次左移`bleatrix % 10`位后，与saw进行或运算，最终得到占位
// bleatrix/= 10 切割掉最后一位数
//
//
//
// bleatrix: 1692
// bleatrix%10: 2
// m: 100
// saw: 100
// bleatrix: 169
// bleatrix: 169
// bleatrix%10: 9
// m: 1000000000
// saw: 1000000100
// bleatrix: 16
// bleatrix: 16
// bleatrix%10: 6
// m: 1000000
// saw: 1001000100
// bleatrix: 1
// bleatrix: 1
// bleatrix%10: 1
// m: 10
// saw: 1001000110
// bleatrix: 0
// bleatrix: 3384
// bleatrix%10: 4
// m: 10000
// saw: 1001010110
// bleatrix: 338
// bleatrix: 338
// bleatrix%10: 8
// m: 100000000
// saw: 1101010110
// bleatrix: 33
// bleatrix: 33
// bleatrix%10: 3
// m: 1000
// saw: 1101011110
// bleatrix: 3
// bleatrix: 3
// bleatrix%10: 3
// m: 1000
// saw: 1101011110
// bleatrix: 0
// bleatrix: 5076
// bleatrix%10: 6
// m: 1000000
// saw: 1101011110
// bleatrix: 507
// bleatrix: 507
// bleatrix%10: 7
// m: 10000000
// saw: 1111011110
// bleatrix: 50
// bleatrix: 50
// bleatrix%10: 0
// m: 1
// saw: 1111011111
// bleatrix: 5
// bleatrix: 5
// bleatrix%10: 5
// m: 100000
// saw: 1111111111
// bleatrix: 0
// 5076
//

fn trotter_2(n: i32) -> i32 {
    assert!(n >= 0);
    if n == 0 {
        -1
    } else {
        let mut saw = 0_u16;
        let mut res = 0;
        while saw != 0b1_111_111_111 {
            res += n;
            let mut bleatrix = res;
            while bleatrix > 0 {
                saw |= 1 << bleatrix % 10;
                bleatrix /= 10;
            }
        }
        res
    }
}

// solution3

struct N {
    n: i32,
}

impl Iterator for N {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.n == 0 {
            return None;
        }

        let d = self.n % 10;
        self.n /= 10;
        Some(d)
    }
}

fn trotter_3(n: i32) -> i32 {
    if n == 0 {
        return -1;
    }
    let mut seen: Vec<i32> = vec![];
    let mut f = 0;
    loop {
        f += 1;
        let num = N { n: n * f };
        for d in num {
            if !seen.contains(&d) {
                seen.push(d);
            }
        }

        if seen.len() == 10 {
            return n * f;
        }
    }
}

// solution 4
// `num /= 10` 是个技巧为了分离数字，比如：
//   1692被分离成：
//      169,{2}
//      16, {9, 2}
//      1, {6, 9, 2}
use std::collections::HashSet;

fn trotter_4(n: i32) -> i32 {
    if n == 0 {
        return -1;
    }
    let mut set = HashSet::new();

    for i in 1..100 {
        let mut num = i * n;
        while num > 0 {
            set.insert(num % 10);
            num /= 10;
        }

        if set.len() == 10 {
            return i * n;
        }
    }

    return -1;
}

// solution 5

fn trotter_5(n: i32) -> i32 {
    let mut numbers: [bool; 10] = [false; 10];

    let mut result = -1;

    if n != 0 {
        for x in 1..(i32::max_value() / n) {

            for c in (n * x).to_string().chars() {
                numbers[c.to_digit(10).unwrap() as usize] = true;
            }

            if numbers.iter().all(|&x| x == true) {
                result = n * x;
                break;
            }
        }
    }

    result
}

// solution 6

fn trotter_6(n: i32) -> i32 {
    let (mut i, mut vec, mut last) = (1, vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], 0);
    if n == 0 {
        return -1;
    }

    loop {
        let start = i * n;
        let s: String = start.to_string();
        for c in s.chars() {
            if vec.contains(&c) {
                let index = vec.iter().position(|x| *x == c).unwrap();
                vec.remove(index);
            }
        }
        if vec.is_empty() {
            last = start;
            return -1;
        }
        i += 1;

    }
    last
}


#[test]
fn returns_expected() {
    assert_eq!(trotter(1692), 5076);
    assert_eq!(trotter(16929999), 84649995);
    assert_eq!(trotter(2), 90);
    assert_eq!(trotter(7), 70);
    assert_eq!(trotter(0), -1);
}
