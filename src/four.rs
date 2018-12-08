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

    let mut current_guard_id: isize = -1;
    let mut sleep_start_minute: isize = -1;
    let mut is_asleep: bool = false;
    let guard_sleeping_minutes: Vec<(isize, isize)> = logs
        .into_iter()
        .flat_map(|log| match log.event {
            LogActivity::StartDuty(n) => {
                current_guard_id = n;
                return vec![];
            }
            LogActivity::FallAsleep => {
                is_asleep = true;
                sleep_start_minute = log.minute;
                return vec![];
            }
            LogActivity::WakeUp => {
                if !is_asleep || sleep_start_minute == -1 || current_guard_id == -1 {
                    panic!("logs are not in a valid order")
                }
                let mut ret_v = vec![];
                for m in sleep_start_minute..log.minute {
                    ret_v.push((current_guard_id, m));
                }
                return ret_v;
            }
        }).collect();

    let guard_ids: Vec<isize> = guard_to_minute
        .clone()
        .into_iter()
        .map(|(k, _)| k)
        .collect();
    for guard_id in guard_ids.into_iter() {
        let mut minute_count: HashMap<isize, isize> = HashMap::new();
        let gsm = guard_sleeping_minutes.clone();
        let mut total_count: isize = 0;
        gsm.into_iter()
            .filter(|(g_id, _)| guard_id == *g_id)
            .for_each(|(_, m)| {
                let c = minute_count.entry(m).or_insert(0);
                *c += 1;
                total_count += 1;
            });

        let (max_minute, _) = minute_count
            .into_iter()
            .max_by(|(_, c_a), (_, c_b)| c_a.cmp(c_b))
            .unwrap_or((-1, -1));
        guard_to_minute.insert(guard_id, (max_minute, total_count));
    }
    // guard_to_minute
    //     .clone()
    //     .into_iter()
    //     .for_each(|(g_id, (m, c))| {
    //         println!("guard {} had max minute {} with count {}", g_id, m, c)
    //     });
    let (g_id, (minute, _)) = guard_to_minute
        .into_iter()
        .max_by(|(_, (_, count_a)), (_, (_, count_b))| count_a.cmp(count_b))
        .unwrap();
    println!(
        "max is guard {} at minute {}, for solution {}",
        g_id,
        minute,
        (g_id * minute)
    );
}

pub fn part_two(file_contents: &String) -> () {
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

    let mut current_guard_id: isize = -1;
    let mut sleep_start_minute: isize = -1;
    let mut is_asleep: bool = false;
    let guard_sleeping_minutes: Vec<(isize, isize)> = logs
        .into_iter()
        .flat_map(|log| match log.event {
            LogActivity::StartDuty(n) => {
                current_guard_id = n;
                return vec![];
            }
            LogActivity::FallAsleep => {
                is_asleep = true;
                sleep_start_minute = log.minute;
                return vec![];
            }
            LogActivity::WakeUp => {
                if !is_asleep || sleep_start_minute == -1 || current_guard_id == -1 {
                    panic!("logs are not in a valid order")
                }
                let mut ret_v = vec![];
                for m in sleep_start_minute..log.minute {
                    ret_v.push((current_guard_id, m));
                }
                return ret_v;
            }
        }).collect();

    let guard_ids: Vec<isize> = guard_to_minute
        .clone()
        .into_iter()
        .map(|(k, _)| k)
        .collect();
    for guard_id in guard_ids.into_iter() {
        let mut minute_count: HashMap<isize, isize> = HashMap::new();
        let gsm = guard_sleeping_minutes.clone();
        let mut total_count: isize = 0;
        gsm.into_iter()
            .filter(|(g_id, _)| guard_id == *g_id)
            .for_each(|(_, m)| {
                let c = minute_count.entry(m).or_insert(0);
                *c += 1;
                total_count += 1;
            });

        let (max_minute, _) = minute_count
            .into_iter()
            .max_by(|(_, c_a), (_, c_b)| c_a.cmp(c_b))
            .unwrap_or((-1, -1));
        guard_to_minute.insert(guard_id, (max_minute, total_count));
    }
    // guard_to_minute
    //     .clone()
    //     .into_iter()
    //     .for_each(|(g_id, (m, c))| {
    //         println!("guard {} had max minute {} with count {}", g_id, m, c)
    //     });
    let (g_id, (minute, _)) = guard_to_minute
        .into_iter()
        .max_by(|(_, (_, count_a)), (_, (_, count_b))| count_a.cmp(count_b))
        .unwrap();
    println!(
        "max is guard {} at minute {}, for solution {}",
        g_id,
        minute,
        (g_id * minute)
    );
}
