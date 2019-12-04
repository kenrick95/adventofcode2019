fn is_number_valid_pt1(number: u32) -> bool {
    // Two adjacent digits are the same (like 22 in 122345).
    // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
    return has_two_adjacent_digits(number) && has_non_decreasing_digits(number);
}

fn is_number_valid_pt2(number: u32) -> bool {
    // There must be at least one matching pair, AND that matching pair must not be part of a larger group of digits.
    // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
    return has_matching_pair_not_larger_group(number) && has_non_decreasing_digits(number);
}

fn has_matching_pair_not_larger_group(number: u32) -> bool {
    let number_str = number.to_string();
    let number_strs: Vec<&str> = number_str.split("").collect();

    let mut matching_str_count = 1;

    for i in 0..(number_strs.len() - 1) {
        if number_strs[i] == number_strs[i + 1] {
            matching_str_count += 1;
        } else {
            // Check if valid or not
            if matching_str_count == 2 {
                return true;
            }

            // Reset
            matching_str_count = 1;
        }
    }

    return false;
}

fn has_two_adjacent_digits(number: u32) -> bool {
    let number_str = number.to_string();
    let number_strs: Vec<&str> = number_str.split("").collect();
    for i in 0..(number_strs.len() - 1) {
        if number_strs[i] == number_strs[i + 1] {
            return true;
        }
    }

    return false;
}
fn has_non_decreasing_digits(number: u32) -> bool {
    let number_str = number.to_string();
    let digits: Vec<u32> = number_str
        .trim()
        .chars()
        .map(|val| {
            val.to_string().parse().unwrap()
        })
        .collect();
    for i in 0..(digits.len() - 1) {
        if digits[i] > digits[i + 1] {
            return false;
        }
    }

    return true;
}
// 124075-580769
// 122235-122235 // expect not ok
// 124444-124444 // expect not ok
// 112233-112233 // expect ok
// 111223-111223 // expect ok
// 113444-113444 // expect ok
pub fn main() {
    let numbers: Vec<u32> = super::utils::get_string_from_stdio()
        .trim()
        .split("-")
        .map(|val| val.parse().unwrap())
        .collect();
    let number_min = numbers[0];
    let number_max = numbers[1];

    let mut answer_pt1 = 0;
    let mut answer_pt2 = 0;
    for i in number_min..=number_max {
        if is_number_valid_pt1(i) {
            answer_pt1 += 1;
        }
        if is_number_valid_pt2(i) {
            answer_pt2 += 1;
        }
    }
    println!("Answer part 1: {}", answer_pt1);
    println!("Answer part 2: {}", answer_pt2);
}
