// Check to see if a string has the same amount of 'x's and 'o's. The method must return a boolean and be case insensitive. The string can contains any char.
//
// Examples input/output:
//
// XO("ooxx") => true
// XO("xooxx") => false
// XO("ooxXm") => true
// XO("zpzpzpp") => true // when no 'x' and 'o' is present should return true
// XO("zzoo") => false
//
//

// solution1
fn xo(string: &'static str) -> bool {
    let (mut x_num, mut o_num) = (0, 0);

    for c in string.chars() {
        match c {
            'x' | 'X' => x_num += 1,
            'o' | 'O' => o_num += 1,
            _ => (),
        }
    }

    x_num == o_num
}

// solution2

fn xo_2(string: &'static str) -> bool {
    string.chars().fold(0, |a, c| {
        match c {
            'x' | 'X' => a + 1,
            'o' | 'O' => a - 1,
            _ => a,
        }
    }) == 0
}

// solution3

fn xo_3(string: &'static str) -> bool {
    string.to_lowercase().matches("x").count() == string.to_lowercase().matches("o").count()
}

#[test]
fn returns_expected() {
    assert_eq!(xo("xo"), true);
    assert_eq!(xo("Xo"), true);
    assert_eq!(xo("xxOo"), true);
    assert_eq!(xo("xxxm"), false);
    assert_eq!(xo("Oo"), false);
    assert_eq!(xo("ooom"), false);
}
