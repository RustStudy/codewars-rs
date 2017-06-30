/*
To participate in a prize draw each one gives his/her firstname.

Each letter of a firstname has a value which is its rank in the English alphabet. A and a have rank 1, B and b rank 2 and so on.

The length of the firstname is added to the sum of these ranks hence a number n. An array of random weights is linked to the firstnames and each n is multiplied by its corresponding weight to get what they call a winning number.

Example: names: COLIN,AMANDBA,AMANDAB,CAROL,PauL,JOSEPH weights: [1, 4, 4, 5, 2, 1]

PAUL -> n = length of firstname + 16 + 1 + 21 + 12 = 4 + 50 -> 54 The weight associated with PAUL is 2 so Paul's winning number is 54 * 2 = 108.

Now one can sort the firstnames in decreasing order of the winning numbers. When two people have the same winning number sort them alphabetically by their firstnames.

#Task:

parameters: st a string of firstnames, we an array of weights, n a rank

return: the firstname of the participant whose rank is n (ranks are numbered from 1)

#Example: names: COLIN,AMANDBA,AMANDAB,CAROL,PauL,JOSEPH weights: [1, 4, 4, 5, 2, 1] n: 4

The function should return: PauL

#Note: If st is empty return "No participants".

If n is greater than the number of participants then return "Not enough participants".
*/
// 说明
// 英文26个字母顺序分别代表级别 1到26
// 示例：
// Example:
//        names: COLIN,AMANDBA,AMANDAB,CAROL,PauL,JOSEPH
//        weights: [1, 4, 4, 5, 2, 1]
// 比如 选Paul, n = 字符串的长度 + （首字母的级别之和）= 4 + 16 + 1 + 21 + 12 = 54
//  Paul的权重是2，所以得到 54 * 2 = 108
// 然后把每个字符串的最终优先级算出来之后排序

// 现在要求st代表的是传入的字符串，we代表的是权重数组，n是rank（最终的优先级结果算出来之后排序位置）
// 比如，names: COLIN,AMANDBA,AMANDAB,CAROL,PauL,JOSEPH weights: [1, 4, 4, 5, 2, 1] n: 4，返回Paul
// 注意，st为空的时候返回 "No participants"
// 如果n大于数组长度，则返回"Not enough participants"

use std;

fn ponder(s: &str, w: i32) -> i32 {
    let ss = s.to_uppercase();
    w * ss.chars().fold(ss.len() as i32, |p: i32, q| p + (q as i32 - 64))
}
fn comp(l: (&str, i32), r: (&str, i32)) -> std::cmp::Ordering {
    if l.1 == r.1 {
        (l.0).cmp(&(r.0))
    } else {
        (r.1).cmp(&(l.1))
    }
}
pub fn rank(st: &str, we: Vec<i32>, n: usize) -> &str {
    if st == "" {return "No participants"};
    let f = st.split(',').collect::<Vec<&str>>();
    if n > f.len() {return "Not enough participants"};
    let mut res = vec![];
    for i in 0..f.len() {
        res.push((f[i], ponder(f[i], we[i])));
    }
    res.sort_by(|l, r| comp(*l, *r));
    res[n - 1].0
}


fn testing(st: &str, we: Vec<i32>, n: usize, exp: &str) -> () {
    assert_eq!(rank(st, we, n), exp)
}

#[test]
fn basics_rank() {

    testing("Addison,Jayden,Sofia,Michael,Andrew,Lily,Benjamin", vec![4, 2, 1, 4, 3, 1, 2], 4, "Benjamin");
    testing("Elijah,Chloe,Elizabeth,Matthew,Natalie,Jayden", vec![1, 3, 5, 5, 3, 6], 2, "Matthew");
    testing("Aubrey,Olivai,Abigail,Chloe,Andrew,Elizabeth", vec![3, 1, 4, 4, 3, 2], 4, "Abigail");
    testing("Lagon,Lily", vec![1, 5], 2, "Lagon");

}
