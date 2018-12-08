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
