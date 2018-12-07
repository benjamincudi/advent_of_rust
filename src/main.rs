mod one;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage is `advent_of_rust <day> <part> <filepath>");
        return;
    }

    let day: i8 = args[1].parse().unwrap();

    let part: i8 = args[2]
        .parse()
        .expect("part must be a parseable number; valid values are only 1 and 2");

    let filename = &args[3];
    let file_contents = fs::read_to_string(filename)
        .expect("reading the file went horribly wrong, are you sure it exists?");

    match day {
        1 => match part {
            1 => one::part_one(&file_contents),
            2 => one::part_two(&file_contents),
            _ => println!("unknown part"),
        },
        _ => println!("unknown day"),
    }
}
