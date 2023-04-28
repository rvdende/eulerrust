/*
https://projecteuler.net/problem=4

A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
Find the largest palindrome made from the product of two 3-digit numbers.

*/

fn is_palindrome(input: u128) -> bool {
    input.to_string() == input.to_string().chars().rev().collect::<String>()
}

fn find_largest() -> u128 {
    let mut largest = 0;

    for x in 100..999 {
        for y in 100..999 {
            let test: u128 = x * y;
            if is_palindrome(test) && test > largest {
                largest = test;
            }
        }
    }

    largest
}

pub fn run() {
    find_largest();
}

#[test]
fn test() {
    assert_eq!(is_palindrome(9009), true);
    assert_eq!(is_palindrome(9010), false);

    assert_eq!(find_largest(), 906609);
}
