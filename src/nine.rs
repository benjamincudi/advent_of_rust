use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::VecDeque;

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

    let mut board: Vec<usize> = vec![0];

    let mut scoreboard: HashMap<usize, usize> = HashMap::new();

    let mut current_index: usize = 0;
    let mut current_player: usize = 0;
    for n in 1..final_marble_value {
        let board_len = board.len();
        if n % 23 == 0 {
            let player_score = scoreboard.entry(current_player).or_insert(0);
            *player_score += n;
            current_index = match current_index.cmp(&7) {
                Ordering::Equal => 0,
                Ordering::Greater => current_index - 7,
                Ordering::Less => {
                    let c_i = if board_len > 7 {
                        board_len - 7 + current_index
                    } else {
                        // Here be dragons
                        println!("math shenanigans to find new index, cycling on a small board");
                        let net = 7 % board_len;
                        board_len + current_index - net
                    };
                    c_i
                }
            };
            let secondary_scored_marble = board.remove(current_index);
            *player_score += secondary_scored_marble
        } else {
            current_index = match (current_index + 2).cmp(&board_len) {
                Ordering::Less => {
                    board.insert(current_index + 2, n);
                    current_index + 2
                }
                Ordering::Equal => {
                    board.push(n);
                    board.len() - 1
                }
                Ordering::Greater => {
                    board.insert(1, n);
                    1
                }
            };
        }

        current_player = (current_player + 1) % players;
    }

    let (_, top_score) = scoreboard
        .into_iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    println!("{} players, {} marbles", players, final_marble_value + 1);
    println!("high score was {}", top_score);
}

pub fn part_two(file_contents: &String) -> () {
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
    let final_marble_value: usize = data_chunks[1] * 100;

    let mut board: VecDeque<usize> = VecDeque::with_capacity(final_marble_value);
    board.push_front(0);

    let mut scoreboard: HashMap<usize, usize> = HashMap::new();

    let mut current_player: usize = 0;
    for n in 1..final_marble_value {
        if n % 23 == 0 {
            for _ in 0..7 {
                let m = board.pop_back().expect("popping for move failed");
                board.push_front(m);
            }

            let player_score = scoreboard.entry(current_player).or_insert(0);
            *player_score += n + board
                .pop_front()
                .expect("board somehow empty despite recent interaction");
        } else {
            for _ in 0..2 {
                let m = board.pop_front().expect("board somehow has no first item");
                board.push_back(m);
            }
            board.push_front(n);
        }

        current_player = (current_player + 1) % players;
    }

    let (_, top_score) = scoreboard
        .into_iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    println!("{} players, {} marbles", players, final_marble_value + 1);
    println!("high score was {}", top_score);
}
