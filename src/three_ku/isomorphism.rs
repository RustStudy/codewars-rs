/*
https://www.codewars.com/kata/isomorphism/train/rust

We will walk through what is isomorphism, and define some common isomorphism.

This kata possibly unlocks Algebraic Isomorphism, Peano And Church.

代数同构

https://www.codewars.com/kata/algebraic-isomorphism
https://en.wikipedia.org/wiki/Algebraic_data_type

https://www.codewars.com/kata/peano-and-church/
*/

#![allow(dead_code)]

/// so, when are two type, `a` and `b`, considered equal?
/// a definition might be, it is possible to go from `a` to `b`,
/// and from `b` to `a`.
/// Going a roundway trip should leave you the same value.
/// Unfortunately it is virtually impossible to test this in Haskell.
/// This is called ISO.
pub enum Void { }

impl PartialEq for Void {
    fn eq(&self, _: &Void) -> bool {
        true
    }
}

pub fn absurd(_: Void) -> ! {
    panic!("You must be kidding! Where did you find that void instance?");
}

pub type ISO<A: 'static, B: 'static> = (Box<Fn(A) -> B>, Box<Fn(B) -> A>);

pub type IsoFL<A, B, C, D> = Box<FnOnce(Box<Fn(A) -> C>) -> Box<FnOnce(B) -> D>>;
pub type IsoFR<A, B, C, D> = Box<FnOnce(Box<Fn(B) -> D>) -> Box<FnOnce(A) -> C>>;
pub type IsoF<A, B, C, D> = (IsoFL<A, B, C, D>, IsoFR<A, B, C, D>);

pub fn iso<A: 'static, B: 'static, F1, F2>(a: F1, b: F2) -> ISO<A, B>
    where F1: 'static + Fn(A) -> B,
          F2: 'static + Fn(B) -> A,
{
    (Box::new(a), Box::new(b))
}

/// given ISO a b, we can go from a to b
pub fn sub_st_l<A, B>(iso: ISO<A, B>) -> Box<Fn(A) -> B> { iso.0 }

/// and vise versa
pub fn sub_st_r<A, B>(iso: ISO<A, B>) -> Box<Fn(B) -> A> { iso.1 }

/// There can be more than one ISO a b
pub fn iso_bool() -> ISO<bool, bool> {
}

pub fn iso_bool_not() -> ISO<bool, bool> {
}

/// isomorphism is reflexive
pub fn refl<A: 'static>() -> ISO<A, A> {
}

/// isomorphism is symmetric
pub fn symm<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<B, A> {
}

/// isomorphism is transitive
pub fn trans<A: 'static, B: 'static, C: 'static>
    (ab: ISO<A, B>, bc: ISO<B, C>) -> ISO<A, C> {
    }

/// we can combine isomorphism
pub fn iso_tuple<A: 'static, B: 'static, C: 'static, D: 'static>
    (ab: ISO<A, B>, cd: ISO<C, D>) -> ISO<(A, C), (B, D)> {
    }

pub fn iso_vec<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<Vec<A>, Vec<B>> {
}

pub fn iso_option<A: 'static, B: 'static>
    (i: ISO<A, B>) -> ISO<Option<A>, Option<B>> {
    }

pub fn iso_result<A: 'static, B: 'static, C: 'static, D: 'static>
    (ab: ISO<A, B>, cd: ISO<C, D>) -> ISO<Result<A, C>, Result<B, D>> {
    }

/// Going another way is hard (and is generally impossible)
/// Remember, for all valid ISO, converting and converting back
/// is the same as the original value.
/// You need this to prove some case are impossible.
pub fn iso_un_option<A: 'static, B: 'static>
    (i: ISO<Option<A>, Option<B>>) -> ISO<A, B> {
    }

pub fn iso_eu() -> ISO<Result<Vec<()>, ()>, Result<Vec<()>, Void>> {
}

pub fn iso_func<A: 'static, B: 'static, C: 'static, D: 'static>
    (ab: ISO<A, B>, cd: ISO<C, D>) -> IsoF<A, B, C, D> {
    }

/// And we have isomorphism on isomorphism!
pub fn iso_symm<A: 'static, B: 'static>() -> ISO<ISO<A, B>, ISO<B, A>> {
}


// for test


// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html


#[test]
fn sub_st_l_test() {
    assert!(sub_st_l(iso_bool())(true));
    assert!(!sub_st_l(iso_bool())(false));
    assert!(sub_st_l(iso_bool_not())(false));
}

#[test]
fn sub_st_r_test() {
    assert!(sub_st_r(iso_bool())(true));
    assert!(!sub_st_r(iso_bool())(false));
}
