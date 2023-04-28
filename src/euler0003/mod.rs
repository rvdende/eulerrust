/*
https://projecteuler.net/problem=3

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/

pub fn number_to_vector(number: u128) -> Vec<u128> {
    let mut numbers: Vec<u128> = Vec::new();

    for i in 1..number + 1 {
        numbers.push(i);
    }

    return numbers;
}

fn get_primes(user_input: u128) -> Vec<u128> {
    let mut numbers: Vec<u128> = number_to_vector(user_input);
    numbers.remove(numbers.iter().position(|x| *x == 1).unwrap());
    let mut numbers_to_remove: Vec<u128> = Vec::new();
    let mut primes: Vec<u128> = Vec::new(); // new code
    let mut i = 0; // new code

    let ceiling_root: u128 = (user_input as f64).sqrt().ceil() as u128;

    for i in 2..ceiling_root + 1 {
        for j in i..(user_input / i) + 1 {
            // FIX #1:  user_input/i
            numbers_to_remove.push(i * j);
        }
    }

    numbers_to_remove.sort_unstable();
    numbers_to_remove.dedup();
    //numbers_to_remove.retain(|x| *x <= user_input);   // not needed now

    for n in numbers {
        // FIX #2:
        if n < numbers_to_remove[i] {
            //   two linear enumerations
            primes.push(n);
        } else {
            i += 1; //   in unison
        }
    }

    // println!(
    //     "Last prime number up to {}: {:?}",
    //     user_input,
    //     primes.last()
    // );
    // println!(
    //     "Total prime numbers up to {}: {:?}",
    //     user_input,
    //     primes.iter().count()
    // );

    return primes;
}

#[allow(dead_code)]
fn is_prime(input: u128) -> bool {
    if input % 2 == 0 && input > 2 {
        return false;
    }

    println!("{}", input);

    // (1..input).into_iter().filter(|n| input % n == 0).count() == 1

    for n in 1..(input / 2) {
        if input % n == 0 && n > 2 && n != input {
            return false;
        }
    }

    return true;
}

#[allow(dead_code)]
fn solve_prime_factors(input: u128) -> Vec<u128> {
    let mut factors: Vec<u128> = [].to_vec();

    let primes = get_primes(input / 2);

    println!("primes: {}", primes.len());

    for n in primes {
        if input % n == 0 {
            println!("factor found! {}", n);
            factors.push(n);
        }
    }

    return factors;
}

fn largest_prime(input: u128) -> u128 {
    let mut n = input.clone();
    let mut i: u128 = 2;

    while i * i < n {
        while n % i == 0 {
            n /= i;
        }
        i += 1;
    }

    println!("{}", n);

    return n;
}

#[allow(dead_code)]
pub fn run() {
    // let res = get_primes(13195);
    // println!("\nanswer:{:?}", res);
    // let res = largest_prime(13195);
    let res = largest_prime(600_851_475_143);
    println!("\nanswer:{:?}", res);
}

#[test]
fn test() {
    assert_eq!(largest_prime(13195), 29);

    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(6), false);
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(8), false);
    assert_eq!(is_prime(9), false);
    assert_eq!(is_prime(10), false);
    assert_eq!(is_prime(11), true);
    assert_eq!(is_prime(12), false);
    assert_eq!(is_prime(13), true);
    assert_eq!(is_prime(29), true);
}
