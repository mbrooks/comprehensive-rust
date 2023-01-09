/*
 * The Luhn algorithm is used to validate credit card numbers. The algorithm takes
 * a string as input and does the following to validate the credit card number:
 * - Ignore all spaces. Reject number with less than two digits.
 * - Moving from right to left, double every second digit: for the number 1234, we double 3 and 1.
 * - After doubling a digit, sum the digits. So doubling 7 becomes 14 which becomes 5.
 * - Sum all the undoubled and doubled digits.
 * - The credit card number is valid if the sum is ends with 0.
*/

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn sum_of_digits(num: u32) -> u32 {
    // Initialize a running total
    let mut total = 0;

    // Use a loop to repeatedly divide the number by 10 and add the remainder to the total
    let mut n = num;
    while n > 0 {
        total += n % 10;
        n /= 10;
    }

    // Return the total
    total
}

pub fn luhn(cc_number: &str) -> bool {
    let clean_cc_number = remove_whitespace(cc_number);
    let mut digit_sum = 0;

    if clean_cc_number.len() < 2 {
        return false
    }

    for (i, c) in clean_cc_number.chars().rev().enumerate() {
        match c.to_digit(10) {
            Some(digit) => {
                if i % 2 == 1 {
                    let digitdoubled = digit * 2;
                    digit_sum += sum_of_digits(digitdoubled);
                } else {
                    digit_sum += digit;
                }
            }
            None => return false
        }
    }

    digit_sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}