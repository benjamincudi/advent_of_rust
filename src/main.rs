use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let filename = &args[1];

    println!("Using file: {}", filename);

    let res = fs::read_to_string(filename)
        .expect("something went horribly wrong")
        .as_mut_str()
        .split_whitespace()
        .fold(0, |acc, next_val| input_iter(acc, next_val));

    println!("solution is: {}", res);
}

fn input_iter(acc: isize, next_val: &str) -> isize {
    let nv: isize = next_val.to_string().parse().unwrap();

    return acc + nv;
}
