/*
The Fibonacci numbers are the numbers in the following integer sequence (Fn):

0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, ...
such as

F(n) = F(n-1) + F(n-2) with F(0) = 0 and F(1) = 1.
Given a number, say prod (for product), we search two Fibonacci numbers F(n) and F(n+1) verifying

F(n) * F(n+1) = prod.
Your function productFib takes an integer (prod) and returns an array:

[F(n), F(n+1), true] or {F(n), F(n+1), 1} or (F(n), F(n+1), True)
depending on the language if F(n) * F(n+1) = prod.

If you don't find two consecutive F(m) verifying F(m) * F(m+1) = prodyou will return

[F(m), F(m+1), false] or {F(n), F(n+1), 0} or (F(n), F(n+1), False)
F(m) being the smallest one such as F(m) * F(m+1) > prod.

Examples

productFib(714) # should return (21, 34, true),
                # since F(8) = 21, F(9) = 34 and 714 = 21 * 34

productFib(800) # should return (34, 55, false),
                # since F(8) = 21, F(9) = 34, F(10) = 55 and 21 * 34 < 800 < 34 * 55
Notes: Not useful here but we can tell how to choose the number n up to which to go: we can use the "golden ratio" phi which is (1 + sqrt(5))/2 knowing that F(n) is asymptotic to: phi^n / sqrt(5). That gives a possible upper bound to n.

You can see examples in "Example test".

References

http://en.wikipedia.org/wiki/Fibonacci_number

http://oeis.org/A000045
*/

// solution 1
fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut i = 0;

    loop {
        let n = fib(i) as u64;
        let n_add_1 = fib(i+1) as u64;
        let fib_prod = n * n_add_1;
        if fib_prod == prod {
            return (n , n_add_1, true);
        }else if fib_prod  > prod {
            return (n, n_add_1, false)
        }
        i += 1;
    }
}


// 根据非波那契通项公式求第n个数
// Fn=(((1+sqrt(5))/2)^n-((1-sqrt(5))/2)^n)/sqrt(5)
fn fib(n: i32) -> f64 {
    let f: f64 = (((1.0+5f64.sqrt())/2 as f64).powi(n)-((1.0-5f64.sqrt())/2 as f64).powi(n))/5f64.sqrt();
    f.round()
}

// solution 2
// 更简单的方法
fn product_fib_2(prod: u64) -> (u64, u64, bool) {
    let mut a = 0; let mut b = 1;
    while a * b < prod {
        let tmp = a;
        a = b;
        b = tmp + b;
    }
    let bl = if a * b == prod {true} else {false};
    (a, b, bl)
}


// solution3

fn product_fib_3(prod: u64) -> (u64, u64, bool) {
    let mut f_last = 1;
    let mut f_this = 1;
    while f_last * f_this < prod {
      let f_next = f_last + f_this;
      f_last = f_this;
      f_this = f_next;
    }
    (f_last, f_this, f_last * f_this == prod)
}



// for test

fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
    assert_eq!(product_fib(prod), exp)
}

#[test]
fn basics_product_fib() {
    dotest(4895, (55, 89, true));
    dotest(5895, (89, 144, false));
}
