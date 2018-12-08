use std::cmp::Ordering;
use std::error;
use std::fmt;
use std::str::FromStr;

struct Point {
    x_offset: usize,
    y_offset: usize,
}

impl Point {
    fn taxicab_to(&self, p: Point) -> usize {
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

impl FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data_chunks: Vec<usize> = s.split(", ").map(|s| s.parse().unwrap()).collect();
        let p = Point {
            x_offset: data_chunks[0],
            y_offset: data_chunks[1],
        };
        return Ok(p);
    }
}
