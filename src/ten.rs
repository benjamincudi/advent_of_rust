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
