pub fn part_one(file_contents: &String) -> () {
    let data_chunks: Vec<usize> = file_contents
        .clone()
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse())
        .filter(|o| match o {
            Err(_) => false,
            Ok(_) => true,
        }).map(|o| o.unwrap())
        .collect();

    let players: usize = data_chunks[0];
    let final_marble_value: usize = data_chunks[1];

    println!("{} players, {} marbles", players, final_marble_value + 1);
}
