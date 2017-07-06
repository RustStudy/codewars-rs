/*
A simple substitution cipher replaces one character from an alphabet with a character from an alternate alphabet, where each character's position in an alphabet is mapped to the alternate alphabet for encoding or decoding.

E.g.

let map1 = "abcdefghijklmnopqrstuvwxyz";
let map2 = "etaoinshrdlucmfwypvbgkjqxz";

let cipher = Cipher::new(map1, map2);
cipher.encode("abc") // => "eta"
cipher.encode("xyz") // => "qxz"
cipher.encode("aeiou") // => "eirfg"

cipher.decode("eta") // => "abc"
cipher.decode("qxz") // => "xyz"
cipher.decode("eirfg") // => "aeiou"
If a character provided is not in the opposing alphabet, simply leave it as be.


*/

struct Cipher {
  map: Vec<(char, char)>
}

impl Cipher {
  fn new(map1: &str, map2: &str) -> Cipher {
    Cipher {
      map: map1.chars().zip(map2.chars()).collect()
    }
  }

  fn encode(&self, string: &str) -> String {
    string.chars().map(|c| self.map.iter().find(|x| x.0 == c).map_or(c, |y| y.1)).collect()
  }

  fn decode(&self, string: &str) -> String {
    string.chars().map(|c| self.map.iter().find(|x| x.1 == c).map_or(c, |y| y.0)).collect()
  }
}

// for test

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn examples() {
  let map1 = "abcdefghijklmnopqrstuvwxyz";
  let map2 = "etaoinshrdlucmfwypvbgkjqxz";

  let cipher = Cipher::new(map1, map2);

  assert_eq!(cipher.encode("abc"), "eta");
  assert_eq!(cipher.encode("xyz"), "qxz");
  assert_eq!(cipher.decode("eirfg"), "aeiou");
  assert_eq!(cipher.decode("erlang"), "aikcfu");
}
