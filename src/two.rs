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
