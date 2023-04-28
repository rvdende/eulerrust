// https://projecteuler.net/problem=1

/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/

fn sum_of_multiples_of_3_or_5(max: usize) -> usize {
    let mut sum = 0;
    for i in 1..max {
        if i % 3 == 0 || i % 5 == 0 {
            sum = sum + i;
        }
    }
    sum
}

fn sum_of_multiples_of_3_or_5_iter(max: usize) -> usize {
    (1..max)
        .into_iter()
        .filter(|i| i % 3 == 0 || i % 5 == 0)
        .sum()
}

#[allow(dead_code)]
pub fn run() {
    println!("{}", sum_of_multiples_of_3_or_5(1000));
    println!("{}", sum_of_multiples_of_3_or_5_iter(1000));
}

#[test]
fn test() {
    assert_eq!(sum_of_multiples_of_3_or_5(10), 23);
    assert_eq!(sum_of_multiples_of_3_or_5(1000), 233168);

    assert_eq!(sum_of_multiples_of_3_or_5_iter(10), 23);
    assert_eq!(sum_of_multiples_of_3_or_5_iter(1000), 233168);
}
