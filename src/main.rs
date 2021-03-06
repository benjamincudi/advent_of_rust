mod eight;
mod five;
mod four;
mod nine;
mod one;
mod seven;
mod six;
mod ten;
mod three;
mod two;
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
        2 => match part {
            1 => two::part_one(&file_contents),
            2 => two::part_two(&file_contents),
            _ => println!("unknown part"),
        },
        3 => match part {
            1 => three::part_one(&file_contents),
            2 => three::part_two(&file_contents),
            _ => println!("unknown part"),
        },
        4 => match part {
            1 => four::part_one(&file_contents),
            2 => four::part_two(&file_contents),
            _ => println!("unknown part"),
        },
        5 => match part {
            1 => five::part_one(&file_contents),
            2 => five::part_two(&file_contents),
            _ => println!("unknown part"),
        },
        6 => match part {
            1 => six::part_one(&file_contents),
            2 => six::part_two(&file_contents),
            _ => println!("unknown part"),
        },
        7 => match part {
            1 => seven::part_one(&file_contents),
            2 => seven::part_two(&file_contents),
            _ => println!("unknown part"),
        },
        8 => match part {
            1 => eight::part_one(&file_contents),
            2 => eight::part_two(&file_contents),
            _ => println!("unknown part"),
        },
        9 => match part {
            1 => nine::part_one(&file_contents),
            2 => nine::part_two(&file_contents),
            _ => println!("unknown part"),
        },
        10 => match part {
            1 => ten::part_one(&file_contents),
            2 => ten::part_two(&file_contents),
            _ => println!("unknown part"),
        },
        _ => println!("unknown day"),
    }
}
