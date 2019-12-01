use std::io;
use std::str::FromStr;

pub fn get_number_from_stdio<T: FromStr>() -> Result<T, T::Err>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).unwrap();
    let input_number: T = input_str.trim().parse()?;
    return Ok(input_number);
}
