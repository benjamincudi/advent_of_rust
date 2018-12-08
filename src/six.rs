use std::cmp::Ordering;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::str::FromStr;
use std::usize;

struct RawPoint {
    x_offset: usize,
    y_offset: usize,
}

#[derive(Clone)]
struct Point {
    x_offset: usize,
    y_offset: usize,
    id: usize,
}

impl Point {
    fn taxicab_to(&self, p: &RawPoint) -> usize {
        let x_diff = match self.x_offset.cmp(&p.x_offset) {
            Ordering::Less => p.x_offset - self.x_offset,
            _ => self.x_offset - p.x_offset,
        };
        let y_diff = match self.y_offset.cmp(&p.y_offset) {
            Ordering::Less => p.y_offset - self.y_offset,
            _ => self.y_offset - p.y_offset,
        };
        return x_diff + y_diff;
    }
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
        let data_chunks: Vec<usize> = s.split(", ").map(|s| s.parse().unwrap()).collect();
        let p = RawPoint {
            x_offset: data_chunks[0],
            y_offset: data_chunks[1],
        };
        return Ok(p);
    }
}

pub fn part_one(file_contents: &String) -> () {
    let mut x_max: usize = 0;
    let mut y_max: usize = 0;
    let mut count: usize = 0;

    let mut point_id_to_area: HashMap<usize, usize> = HashMap::new();

    let input_points: Vec<Point> = file_contents
        .clone()
        .as_mut_str()
        .split("\n")
        .map(|p_str| {
            let rp = RawPoint::from_str(p_str).unwrap();
            count += 1;
            let p = Point {
                x_offset: rp.x_offset,
                y_offset: rp.y_offset,
                id: count,
            };
            x_max = match x_max.cmp(&p.x_offset) {
                Ordering::Less => p.x_offset,
                _ => x_max,
            };
            y_max = match y_max.cmp(&p.y_offset) {
                Ordering::Less => p.y_offset,
                _ => y_max,
            };
            point_id_to_area.insert(count, 0);
            return p;
        }).collect();

    let mut border_points: HashMap<usize, bool> = HashMap::new();

    for x in 0..x_max {
        for y in 0..y_max {
            let current_point = RawPoint {
                x_offset: x,
                y_offset: y,
            };

            let mut min_distance: usize = usize::MAX;
            let mut count_of_min: usize = 0;
            let mut min_id: usize = 0;
            input_points.clone().into_iter().for_each(|p| {
                let distance = p.taxicab_to(&current_point);
                match distance.cmp(&min_distance) {
                    Ordering::Less => {
                        min_distance = distance;
                        count_of_min = 1;
                        min_id = p.id;
                    }
                    Ordering::Equal => {
                        count_of_min += 1;
                        min_id = 0;
                    }
                    _ => (),
                }
            });

            if count_of_min == 1 {
                let a = point_id_to_area.entry(min_id).or_insert(0);
                *a += 1;
            }

            if x == 0 || x == x_max || y == 0 || y == y_max {
                border_points.insert(min_id, true);
            }
        }
    }

    println!("found {} points", input_points.len());
}
