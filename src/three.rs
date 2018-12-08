use std::collections::HashMap;
use std::error;
use std::fmt;
use std::str::FromStr;

static ONE: isize = 1;

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

pub fn part_one(file_contents: &String) -> () {
    let mut hm: HashMap<(isize, isize), isize> = HashMap::new();

    file_contents
        .clone()
        .as_mut_str()
        .split("\n")
        .for_each(|claim| {
            let c: Claim = claim.parse().unwrap();

            for x in 0..c.x_size {
                let x_coord = x + c.x_offset;
                for y in 0..c.y_size {
                    let y_coord = y + c.y_offset;

                    let v = hm.entry((x_coord, y_coord)).or_insert(0);
                    *v += 1;
                }
            }
        });

    let mut sq_in: isize = 0;
    for v in hm.values() {
        if v > &ONE {
            sq_in += 1;
        }
    }

    println!("total sq_in that have overlapping claims: {}", sq_in)
}

pub fn part_two(file_contents: &String) -> () {
    let mut hm: HashMap<(isize, isize), isize> = HashMap::new();
    let mut claims: HashMap<isize, bool> = HashMap::new();

    file_contents
        .clone()
        .as_mut_str()
        .split("\n")
        .for_each(|claim| {
            let c: Claim = claim.parse().unwrap();
            claims.insert(c.claim_id, true);

            for x in 0..c.x_size {
                let x_coord = x + c.x_offset;
                for y in 0..c.y_size {
                    let y_coord = y + c.y_offset;
                    let v = hm.entry((x_coord, y_coord)).or_insert(c.claim_id);
                    if v != &c.claim_id {
                        claims.insert(*v, false);
                        claims.insert(c.claim_id, false);
                    }
                }
            }
        });

    let valid_claim_ids: Vec<isize> = claims
        .into_iter()
        .filter(|(_, no_overlap)| *no_overlap)
        .map(|(claim_id, _)| claim_id)
        .collect();

    if valid_claim_ids.is_empty() {
        println!("no answers found");
    } else {
        println!("found {} answer(s)", valid_claim_ids.len());
        for c_id in valid_claim_ids.iter() {
            println!("answer: {}", c_id);
        }
    }
}
