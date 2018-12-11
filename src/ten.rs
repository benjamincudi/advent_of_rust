use std::cmp::Ordering;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::isize;
use std::str::FromStr;

#[derive(Clone)]
struct RawPoint {
    x_init: isize,
    y_init: isize,
    x_vel: isize,
    y_vel: isize,
}

#[derive(Debug)]
struct PointParseError;
impl fmt::Display for PointParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid Point string")
    }
}
impl error::Error for PointParseError {
    fn description(&self) -> &str {
        "invalid Point"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl FromStr for RawPoint {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data_chunks: Vec<isize> = s
            .split("")
            .filter(|c| {
                if c.to_string() == " ".to_string() || c.to_string() == "-".to_string() {
                    return true;
                }
                match c.parse::<isize>() {
                    Ok(_) => return true,
                    _ => return false,
                };
            }).fold("".to_string(), |acc, s| acc.to_string() + s)
            .as_mut_str()
            .split_whitespace()
            .map(|s| s.parse::<isize>())
            .filter(|o| match o {
                Err(_) => false,
                Ok(_) => true,
            }).map(|o| o.unwrap())
            .collect();

        if data_chunks.len() != 4 {
            panic!("parsing failed");
        }
        let p = RawPoint {
            x_init: data_chunks[0],
            y_init: data_chunks[1],
            x_vel: data_chunks[2],
            y_vel: data_chunks[3],
        };
        return Ok(p);
    }
}

pub fn part_one(file_contents: &String) -> () {
    let points = file_contents
        .clone()
        .as_mut_str()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<RawPoint>>();

    // Since the points must be converging, compare vertical spread against MAX
    let mut min_spread: isize = isize::MAX;

    // These update every loop - their initial value is only to appease the compiler
    let mut min_spread_time: isize = 0;
    let mut y_max: isize = 0;
    let mut y_min: isize = 0;
    for t in 1.. {
        // For each moment in time, determine the current vertical spread
        let mut y_max_loop: isize = isize::MIN;
        let mut y_min_loop: isize = isize::MAX;
        for p in points.clone().into_iter() {
            let current_y: isize = p.y_init + (p.y_vel * t);
            match current_y.cmp(&y_max_loop) {
                Ordering::Greater => y_max_loop = current_y,
                _ => (),
            }
            match current_y.cmp(&y_min_loop) {
                Ordering::Less => y_min_loop = current_y,
                _ => (),
            }
        }
        let spread = y_max_loop - y_min_loop;
        match min_spread.cmp(&spread) {
            Ordering::Greater => min_spread = spread,
            // As soon as we're diverging, we know the previous second was the closes things get
            _ => break,
        }
        // Since these only update if we're still converging, these represent the values we're interested in
        y_max = y_max_loop;
        y_min = y_min_loop;
        min_spread_time = t;
    }
    // Once we have the max and min Y values, find the same for X at the solution tim
    let x_min_point: RawPoint = points
        .clone()
        .into_iter()
        .min_by(|a, b| {
            (a.x_init + (a.x_vel * min_spread_time)).cmp(&(b.x_init + (b.x_vel * min_spread_time)))
        }).unwrap();
    let x_min: isize = x_min_point.x_init + (x_min_point.x_vel * min_spread_time);
    let x_max_point: RawPoint = points
        .clone()
        .into_iter()
        .max_by(|a, b| {
            (a.x_init + (a.x_vel * min_spread_time)).cmp(&(b.x_init + (b.x_vel * min_spread_time)))
        }).unwrap();
    let x_max: isize = x_max_point.x_init + (x_max_point.x_vel * min_spread_time);

    // To make it easier to print the solution letters,
    // Make a HashMap<y_value, <x_value, should_print>>
    let mut grid: HashMap<isize, HashMap<isize, bool>> = HashMap::new();
    for p in points.clone().into_iter() {
        // Calculate the current value for each point
        // and then shift everything towards the origin
        let y: isize = p.y_init + (p.y_vel * min_spread_time) - y_min;
        let x: isize = p.x_init + (p.x_vel * min_spread_time) - x_min;

        let row = grid.entry(y).or_insert(HashMap::new());
        row.insert(x, true);
    }
    // For each row
    for y in 0..y_max - y_min + 1 {
        let mut row_str: String = "".to_string();
        let row = grid.entry(y).or_default();
        // Find every point in that row and render a # for it
        for x in 0..x_max - x_min + 1 {
            match row.get(&x) {
                Some(_) => row_str += "#",
                None => row_str += " ",
            }
        }

        // Print each row as we complete it
        println!("{}", row_str);
    }
}

pub fn part_two(file_contents: &String) -> () {
    let points = file_contents
        .clone()
        .as_mut_str()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<RawPoint>>();

    let mut min_spread: isize = isize::MAX;
    let mut min_spread_time: isize = 0;

    for t in 1.. {
        let mut y_max_loop: isize = isize::MIN;
        let mut y_min_loop: isize = isize::MAX;
        for p in points.clone().into_iter() {
            let current_y: isize = p.y_init + (p.y_vel * t);
            match current_y.cmp(&y_max_loop) {
                Ordering::Greater => y_max_loop = current_y,
                _ => (),
            }
            match current_y.cmp(&y_min_loop) {
                Ordering::Less => y_min_loop = current_y,
                _ => (),
            }
        }
        let spread = y_max_loop - y_min_loop;
        match min_spread.cmp(&spread) {
            Ordering::Greater => min_spread = spread,
            // As soon as we're diverging, we know the previous second was the closes things get
            _ => break,
        }

        min_spread_time = t;
    }

    println!(
        "solution occurred at time {}, with vertical spread of {}",
        min_spread_time, min_spread
    );
}
