use std::error;
use std::fmt;
use std::str::FromStr;

struct Claim {
    claim_id: isize,
    x_offset: isize,
    y_offset: isize,
    x_size: isize,
    y_size: isize,
}

#[derive(Debug)]
struct ClaimParseError;
impl fmt::Display for ClaimParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid claim string")
    }
}
impl error::Error for ClaimParseError {
    fn description(&self) -> &str {
        "invalid claim"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl FromStr for Claim {
    type Err = ClaimParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data_chunks: Vec<&str> = s.split(" @ ").flat_map(|s| s.split(": ")).collect();
        let claim_id: isize = data_chunks[0][1..].parse().unwrap();
        let coords: Vec<isize> = data_chunks[1]
            .split(",")
            .map(|c| c.parse().unwrap())
            .collect();
        let sizes: Vec<isize> = data_chunks[2]
            .split("x")
            .map(|c| c.parse().unwrap())
            .collect();

        let c = Claim {
            claim_id: claim_id,
            x_offset: coords[0],
            y_offset: coords[1],
            x_size: sizes[0],
            y_size: sizes[1],
        };
        return Ok(c);
    }
}
