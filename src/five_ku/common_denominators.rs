// Common denominators
//
// https://www.codewars.com/kata/common-denominators
//
// You will have a list of rationals in the form
//
// { {numer_1, denom_1} , ... {numer_n, denom_n} }
// or
//
// [ [numer_1, denom_1] , ... [numer_n, denom_n] ]
// or
//
// [ (numer_1, denom_1) , ... (numer_n, denom_n) ]
// where all numbers are positive ints.
//
// You have to produce a result in the form
//
// (N_1, D) ... (N_n, D)
// or
//
// [ [N_1, D] ... [N_n, D] ]
// or
//
// [ (N_1', D) , ... (N_n, D) ]
// or
//
// {{N_1, D} ... {N_n, D}}
// depending on the language (See Example tests)
//
// in which D is as small as possible and
//
// N_1/D == numer_1/denom_1 ... N_n/D == numer_n,/denom_n.
// Example:
//
// convertFracs [(1, 2), (1, 3), (1, 4)] `shouldBe` [(6, 12), (4, 12), (3, 12)]
//
//
//

// fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
//     // your code
// }
//
// fn testing(l: Vec<(i64, i64)>, exp: Vec<(i64, i64)>) -> () {
//     assert_eq!(convert_fracts(l), exp)
// }
//
// #[test]
// fn basics_convert_fracts() {
//
//     testing(vec![(69, 130), (87, 1310), (3, 4)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
//     testing(vec![(690, 1300), (87, 1310), (30, 40)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
//
// }
