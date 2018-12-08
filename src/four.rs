use std::cmp::Ordering;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::str::FromStr;

#[derive(Eq)]
enum LogActivity {
    StartDuty(isize),
    FallAsleep,
    WakeUp,
}
impl fmt::Display for LogActivity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogActivity::StartDuty(_n) => write!(f, "StartDuty"),
            LogActivity::FallAsleep => write!(f, "FallAsleep"),
            LogActivity::WakeUp => write!(f, "WakeUp"),
        }
    }
}

fn get_guard_number(l: &LogActivity) -> Option<isize> {
    match l {
        LogActivity::StartDuty(n) => Some(*n),
        _ => None,
    }
}
impl PartialEq for LogActivity {
    fn eq(&self, other: &LogActivity) -> bool {
        match self {
            LogActivity::StartDuty(_n) => get_guard_number(self) == get_guard_number(other),
            _ => self == other,
        }
    }
}

#[derive(Debug)]
struct LogParseError;
impl fmt::Display for LogParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid log string")
    }
}
impl error::Error for LogParseError {
    fn description(&self) -> &str {
        "invalid log"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Eq, PartialEq)]
struct RawLog {
    year: i16,
    month: i8,
    day: i8,
    hour: i8,
    minute: isize,
    event: LogActivity,
}

impl Ord for RawLog {
    fn cmp(&self, other: &RawLog) -> Ordering {
        match self.year.cmp(&other.year) {
            Ordering::Equal => (),
            x => return x,
        }

        match self.month.cmp(&other.month) {
            Ordering::Equal => (),
            x => return x,
        }

        match self.day.cmp(&other.day) {
            Ordering::Equal => (),
            x => return x,
        }

        match self.hour.cmp(&other.hour) {
            Ordering::Equal => (),
            x => return x,
        }

        return self.minute.cmp(&other.minute);
    }
}
impl PartialOrd for RawLog {
    fn partial_cmp(&self, other: &RawLog) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl fmt::Display for RawLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}-{}-{} at {}:{} - {}",
            self.year, self.month, self.day, self.hour, self.minute, self.event
        )
    }
}

impl FromStr for RawLog {
    type Err = LogParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<&str> = s.splitn(3, " ").collect();
        let date_chunks: Vec<&str> = data[0][1..].split("-").collect();
        let time_chunks: Vec<&str> = data[1].split(":").collect();
        let event: LogActivity = if data[2].starts_with("wakes up") {
            LogActivity::WakeUp
        } else if data[2].starts_with("falls asleep") {
            LogActivity::FallAsleep
        } else {
            let guard_number: isize = data[2]
                .split("#")
                .nth(1)
                .unwrap()
                .split(" ")
                .nth(0)
                .unwrap()
                .parse()
                .unwrap();
            LogActivity::StartDuty(guard_number)
        };

        let c = RawLog {
            year: date_chunks[0].replace("[", "").parse().unwrap(),
            month: date_chunks[1].parse().unwrap(),
            day: date_chunks[2].parse().unwrap(),
            hour: time_chunks[0].parse().unwrap(),
            minute: time_chunks[1].replace("]", "").parse().unwrap(),
            event: event,
        };
        return Ok(c);
    }
}

pub fn part_one(file_contents: &String) -> () {
    let mut guard_to_minute: HashMap<isize, (isize, isize)> = HashMap::new();
    let mut logs = file_contents
        .clone()
        .as_mut_str()
        .split("\n")
        .map(|log| {
            let l = RawLog::from_str(log).unwrap();
            match l.event {
                LogActivity::StartDuty(n) => {
                    guard_to_minute.entry(n).or_insert((-1, -1));
                }
                _ => (),
            };
            return l;
        }).collect::<Vec<RawLog>>();

    logs.sort_unstable();

    guard_to_minute
        .keys()
        .for_each(|k| println!("guard found with id {}", k));
}
