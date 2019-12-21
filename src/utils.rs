use std::fs;
use std::io;
use std::str::FromStr;

pub fn get_number_from_stdio<T: FromStr>() -> Result<T, T::Err>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).unwrap();
    let input_number: T = input_str.trim().parse()?;
    Ok(input_number)
}

pub fn get_string_from_stdio() -> String {
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).unwrap();
    input_str
}

pub fn get_list_of_numbers<T: FromStr>() -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    get_string_from_stdio()
        .trim()
        .split(',')
        .map(|val| val.parse().unwrap())
        .collect()
}

pub fn get_gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        get_gcd(b, a % b)
    }
}

pub fn get_lcm(a: usize, b: usize) -> usize {
    (a / get_gcd(a, b)) * b
}

pub fn get_strings_from_file(path: &str) -> Vec<String> {
    let all_inputs = fs::read_to_string(path)
        .expect("Unable to read file")
        .to_string()
        .trim()
        .to_string();
    let inputs: Vec<String> = all_inputs.split('\n').map(|x| x.to_string()).collect();
    return inputs;
}
pub fn get_strings_from_file_no_trim(path: &str) -> Vec<String> {
    let all_inputs = fs::read_to_string(path)
        .expect("Unable to read file")
        .to_string();
    let inputs: Vec<String> = all_inputs.split('\n').map(|x| x.to_string()).collect();
    return inputs;
}

pub fn get_list_of_numbers_from_file<T: FromStr>(path: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fs::read_to_string(path)
        .expect("Unable to read file")
        .to_string()
        .trim()
        .split(',')
        .map(|val| val.parse().unwrap())
        .collect()
}
