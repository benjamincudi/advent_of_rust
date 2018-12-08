use std::collections::HashMap;

pub fn part_one(file_contents: &String) -> () {
    let polymer_init = file_contents.clone();
    let mut polymer = polymer_init.clone();

    let chars: Vec<&str> = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];
    let replace_pairs: Vec<String> = chars
        .clone()
        .into_iter()
        .map(|c| c.to_owned() + c.to_uppercase().as_str())
        .collect();
    let reversed_pairs: Vec<String> = chars
        .into_iter()
        .map(|c| c.to_uppercase().as_str().to_owned() + c)
        .collect();

    let mut last_known_len: usize = polymer.len();
    loop {
        replace_pairs.clone().into_iter().for_each(|r| {
            polymer = polymer.clone().replace(r.as_str(), "");
        });
        reversed_pairs.clone().into_iter().for_each(|r| {
            polymer = polymer.clone().replace(r.as_str(), "");
        });
        if polymer.len() < last_known_len {
            last_known_len = polymer.len();
        } else {
            break;
        }
    }

    println!(
        "initial size was {}, final size is {}",
        polymer_init.len(),
        polymer.len()
    );
}

pub fn part_two(file_contents: &String) -> () {
    let polymer_init = file_contents.clone();
    let polymer = polymer_init.clone();

    let chars: Vec<&str> = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];
    let replace_pairs: Vec<String> = chars
        .clone()
        .into_iter()
        .map(|c| c.to_owned() + c.to_uppercase().as_str())
        .collect();
    let reversed_pairs: Vec<String> = chars
        .clone()
        .into_iter()
        .map(|c| c.to_uppercase().as_str().to_owned() + c)
        .collect();

    let mut result_by_char: HashMap<&str, usize> = HashMap::new();

    for current_char in chars.into_iter() {
        let mut without_current = polymer
            .clone()
            .replace(current_char, "")
            .replace(current_char.to_uppercase().as_str(), "");
        let mut last_known_len: usize = without_current.len();
        loop {
            replace_pairs.clone().into_iter().for_each(|r| {
                without_current = without_current.clone().replace(r.as_str(), "");
            });
            reversed_pairs.clone().into_iter().for_each(|r| {
                without_current = without_current.clone().replace(r.as_str(), "");
            });
            if without_current.len() < last_known_len {
                last_known_len = without_current.len();
            } else {
                break;
            }
        }
        result_by_char.insert(current_char, without_current.len());
    }

    let (min_char, min_size) = result_by_char
        .into_iter()
        .min_by(|(_, c_a), (_, c_b)| c_a.cmp(c_b))
        .unwrap();

    println!("min size was {} for char {}", min_size, min_char);
}
