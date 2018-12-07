use std::collections::HashMap;

static TWO: i8 = 2;
static THREE: i8 = 3;

pub fn part_one(file_contents: &String) -> () {
    let mut hm: HashMap<char, i8> = HashMap::new();
    let mut twice: isize = 0;
    let mut thrice: isize = 0;

    file_contents
        .clone()
        .as_mut_str()
        .split_whitespace()
        .for_each(|id| {
            id.chars().for_each(|c| {
                let v = hm.entry(c).or_insert(0);
                *v += 1;
            });
            if hm.iter().any(|(_, x)| &TWO == x) {
                twice += 1;
            }
            if hm.iter().any(|(_, x)| x == &THREE) {
                thrice += 1;
            }

            hm.clear()
        });

    println!("answer: {}", twice * thrice)
}

pub fn part_two(file_contents: &String) -> () {
    let mut hm: HashMap<String, bool> = HashMap::new();

    let mut solution: String = "".to_string();

    file_contents
        .clone()
        .as_mut_str()
        .split_whitespace()
        .for_each(|s| {
            let mut i = 0;
            let len_s = s.len();

            while i < len_s {
                let first = if i == 0 { "" } else { s.get(0..i).unwrap() };
                let second = if i == len_s - 1 {
                    ""
                } else {
                    s.get((i + 1)..).unwrap()
                };

                let key = first.to_owned() + "_" + second;

                if hm.get(&key).is_some() {
                    solution = key;
                } else {
                    hm.insert(key, true);
                }

                i += 1;
            }
        });

    println!("answer: {}", solution)
}
