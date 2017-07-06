// When we throw 2 classical dice (values on each side from 1 to 6) we have 36 (6 * 6) different results.
//
// We want to know the probability of having the sum of the results equals to 11. For that result we have only the combination of 6 and 5. So we will have two events: {5, 6} and {6, 5}
//
// So the probability for that result will be:
//
// P(11, 2) = 2/(6*6) = 1/18    (The two is because we have 2 dice)
// Now, we want to calculate the probability of having the sum equals to 8. The combinations for that result will be the following: {4,4}, {3,5}, {5,3}, {2,6}, {6,2} with a total of five combinations.
//
// P(8, 2) = 5/36
// Things may be more complicated if we have more dices and sum values higher.
//
// We want to know the probability of having the sum equals to 8 but having 3 dice.
//
// Now the combinations and corresponding events are:
//
// {2,3,3}, {3,2,3}, {3,3,2}
// {2,2,4}, {2,4,2}, {4,2,2}
// {1,3,4}, {1,4,3}, {3,1,4}, {4,1,3}, {3,4,1}, {4,3,1}
// {1,2,5}, {1,5,2}, {2,1,5}, {5,1,2}, {2,5,1}, {5,2,1}
// {1,1,6}, {1,6,1}, {6,1,1}
//
// A total amount of 21 different combinations
//
// So the probability is:
// P(8, 3) = 21/(6*6*6) = 0.09722222222222222
// Summarizing the cases we have seen with a function that receives the two arguments
//
// rolldice_sum_prob(11, 2) == 0.0555555555 # or 1/18
//
// rolldice_sum_prob(8, 2) ==  0.13888888889# or 5/36
//
// rolldice_sum_prob(8, 3) == 0.0972222222222  # or 7/72
// And think why we have this result:
//
// rolldice_sum_prob(22, 3) == 0
// Create the function rolldice_sum_prob() for this calculation.
//
// Have a nice time!
//
// (You do not have to round the results)
//

// solution1
// 递推求组合次数
// f64类型的可以简写，比如： `1.0`写为`1.` 省略掉后面的0
fn rolldice_sum_prob(sum: i32, dice_amount: i32) -> f64 {
    if sum > dice_amount * 6 {
        return 0.;
    }

    if dice_amount == 0 {
        return if sum == 0 { 1. } else { 0. };
    }

    let mut prob = 0.;

    for i in 1..6 + 1 {
        if i > sum {
            break;
        }
        prob += rolldice_sum_prob(sum - i, dice_amount - 1) / 6.;
    }
    prob


}

// solution2

const DICE_SIZE: i32 = 6;

fn count_combinations(sum: i32, dice_amount: i32) -> i64 {
    if sum < dice_amount || dice_amount * DICE_SIZE < sum {
        0
    } else if dice_amount == 1 {
        1
    } else {
        println!("sum:{} amount:{}", sum, dice_amount);
        (1..(DICE_SIZE + 1))
            .map(|i| count_combinations(sum - i, dice_amount - 1))
            .sum()
    }
}

fn rolldice_sum_prob_2(sum: i32, dice_amount: i32) -> f64 {
    count_combinations(sum, dice_amount) as f64 / (DICE_SIZE.pow(dice_amount as u32)) as f64
}

// for test

fn assert_fuzzy_eq(actual: f64, expected: f64, eps: f64) {
    assert!((actual - expected).abs() < eps,
            format!("Expected {}, but got {}", expected, actual));
}

#[test]
fn returns_expected() {
    assert_fuzzy_eq(rolldice_sum_prob(11, 2), 0.055555555555, 1e-10);
    assert_fuzzy_eq(rolldice_sum_prob(8, 2), 0.13888888889, 1e-10);
    assert_fuzzy_eq(rolldice_sum_prob(8, 3), 0.0972222222222, 1e-10);
}
