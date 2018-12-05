use std::collections::HashMap;
use std::env;
use std::fs;
use std::str;

struct Solution {
    value: isize,
    is_valid: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut seen_v: HashMap<isize, bool> = HashMap::new();
    let mut first_repeat = Solution {
        value: 0,
        is_valid: false,
    };

    let mut init: isize = 0;

    loop {
        init = fs::read_to_string(filename)
            .expect("something went horribly wrong")
            .as_mut_str()
            .split_whitespace()
            .fold(init, |acc, next_val| {
                let res = input_iter(acc, next_val);

                if seen_v.get(&res).is_some() && !first_repeat.is_valid == true {
                    first_repeat = Solution {
                        value: res,
                        is_valid: true,
                    };
                } else {
                    seen_v.insert(res, true);
                }

                return res;
            });

        if first_repeat.is_valid {
            break;
        }
    }

    println!("solution is: {}", first_repeat.value);
}

fn input_iter(acc: isize, next_val: &str) -> isize {
    let nv: isize = next_val.to_string().parse().unwrap();

    return acc + nv;
}
