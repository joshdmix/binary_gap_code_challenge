extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("binary_gap")
        .arg(
            Arg::with_name("integer")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Enter an integer"),
        )
        .get_matches();
    let input = matches.value_of("integer").unwrap();
    calculate_max_binary_gap(input);
}

fn calculate_max_binary_gap(input: &str) {
    let parse_input = input
        .parse::<i64>()
        .expect("******Invalid input. Please enter an integer between -9223372036854775807 and 9223372036854775807.******");
    let input_bin: String = format!("{:b}", parse_input);
    let trim_split_bin: Vec<&str> = input_bin
        .trim_start_matches('0')
        .trim_end_matches('0')
        .split("1")
        .collect::<Vec<&str>>();
    let len_vec: Vec<_> = trim_split_bin.iter().map(|gap| gap.len()).collect();
    let max_gap = len_vec.iter().max().unwrap();
    println!("Your number {:?}", parse_input);
    println!("Your number in binary {:?}", input_bin);
    println!("Max binary gap {:?}", max_gap);
}
