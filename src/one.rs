use std::collections::HashMap;
use std::str;

struct Solution {
    value: isize,
    is_valid: bool,
}

pub fn part_one(file_contents: &String) -> () {
    let r: isize = file_contents
        .clone()
        .as_mut_str()
        .split_whitespace()
        .fold(0, input_iter);

    println!("answer: {}", r);
}

pub fn part_two(file_contents: &String) -> () {
    let mut sln = Solution {
        value: 0,
        is_valid: false,
    };
    let mut seen: HashMap<isize, bool> = HashMap::new();
    let mut init: isize = 0;

    loop {
        init = file_contents
            .clone()
            .as_mut_str()
            .split_whitespace()
            .fold(init, |acc, next_val| {
                let r: isize = input_iter(acc, next_val);

                if seen.get(&r).is_some() && !sln.is_valid {
                    sln = Solution {
                        value: r,
                        is_valid: true,
                    };
                } else {
                    seen.insert(r, true);
                }

                return r;
            });
        if sln.is_valid {
            break;
        }
    }

    println!("answer: {}", sln.value);
}

fn input_iter(acc: isize, next_val: &str) -> isize {
    let nv: isize = next_val.to_string().parse().unwrap();

    return acc + nv;
}
