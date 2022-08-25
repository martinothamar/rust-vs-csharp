use std::{num::ParseIntError};


fn main() {
    let num_str = "2s";

    let real_num_result = parse_number_correct(num_str);
    println!("{:?}", real_num_result);

    let real_num = parse_number_wrong(num_str);
    println!("{}", real_num);
}

fn parse_number_correct(num_str: &str) -> Result<i32, ParseIntError> {
    num_str.parse()
}

fn parse_number_wrong(num_str: &str) -> i32 {
    num_str.parse().expect("I promise this is a number")
}
