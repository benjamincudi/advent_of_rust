use std::cmp::Ordering;
use std::collections::HashMap;
use std::usize;

pub fn part_one(file_contents: &String) -> () {
    let mut num_directions: usize = 0;

    let mut step_to_blockers: HashMap<String, Vec<String>> = HashMap::new();

    file_contents
        .clone()
        .as_mut_str()
        .split("\n")
        .for_each(|d_str| {
            let lower_step = d_str.to_lowercase();
            num_directions += 1;
            let steps: Vec<&str> = lower_step.split("step ").collect();
            let blocker = steps[1][0..1].to_string();
            let blockee = steps[2][0..1].to_string();

            step_to_blockers.entry(blocker.clone()).or_insert(vec![]);
            let v = step_to_blockers.entry(blockee).or_insert(vec![]);
            v.push(blocker);
        });

    let mut order_for_steps: Vec<String> = vec![];

    while step_to_blockers.len() > 0 {
        let mut ready_steps: Vec<String> = step_to_blockers
            .clone()
            .into_iter()
            .filter(|(_, blockers)| blockers.is_empty())
            .map(|(c, _)| c)
            .collect();

        ready_steps.sort_unstable();
        // Only perform the first available step, in alpha order
        let c = ready_steps.first().unwrap();

        order_for_steps.push(c.clone());
        step_to_blockers.remove_entry(c);

        for blockers in step_to_blockers.values_mut() {
            blockers.retain(|v| v != c);
        }
    }

    println!("order for directions: {}", order_for_steps.join(""));
}

struct Job {
    step: String,
    time_remaining: usize,
}

impl Job {
    fn decrease_time(&mut self, less: usize) {
        self.time_remaining -= less;
    }
}

pub fn part_two(file_contents: &String) -> () {
    let mut num_directions: usize = 0;
    let mut step_to_blockers: HashMap<String, Vec<String>> = HashMap::new();

    file_contents
        .clone()
        .as_mut_str()
        .split("\n")
        .for_each(|d_str| {
            let lower_step = d_str.to_lowercase();
            num_directions += 1;
            let steps: Vec<&str> = lower_step.split("step ").collect();
            let blocker = steps[1][0..1].to_string();
            let blockee = steps[2][0..1].to_string();

            step_to_blockers.entry(blocker.clone()).or_insert(vec![]);
            let v = step_to_blockers.entry(blockee).or_insert(vec![]);
            v.push(blocker);
        });
    let total_steps = step_to_blockers.len();
    let mut order_for_steps: Vec<String> = vec![];

    let num_workers: usize = 4;
    let bonus_time: usize = 60;

    let mut current_time: usize = 0;

    let mut worker_available: HashMap<usize, bool> = HashMap::new();
    for id in 0..num_workers {
        worker_available.insert(id, true);
    }

    let mut queue_map: HashMap<usize, Job> = HashMap::new();

    while order_for_steps.len() < total_steps {
        // println!("beginning loop iteration, time is {}", current_time);

        if queue_map.len() > 0 {
            // println!("queue has jobs");
            let mut next_to_finish: usize = usize::MAX;
            let mut time_remaining: usize = usize::MAX;
            for (worker_id, job) in queue_map.iter() {
                match job.time_remaining.cmp(&time_remaining) {
                    Ordering::Less => {
                        time_remaining = job.time_remaining;
                        next_to_finish = worker_id.clone();
                    }
                    _ => (),
                }
            }

            match next_to_finish.cmp(&usize::MAX) {
                Ordering::Equal => panic!("didn't find a worker"),
                _ => (),
            }

            let job = queue_map.remove(&next_to_finish).unwrap();

            current_time += job.time_remaining;
            // println!(
            //     "finished job for {}, now at time {}",
            //     job.step, current_time
            // );

            worker_available.insert(next_to_finish, true);
            // println!("worker {} made available", next_to_finish);

            step_to_blockers.remove_entry(&job.step);
            // println!("removed {} from unfinished steps", job.step);

            order_for_steps.push(job.step.clone());

            for blockers in step_to_blockers.values_mut() {
                blockers.retain(|v| v != &job.step.clone());
            }

            if queue_map.len() > 0 {
                // println!(
                //     "queue still has {} jobs, decreasing remaining job times",
                //     queue_map.len()
                // );
                for j in queue_map.values_mut() {
                    // println!("{} has {} left", j.step, j.time_remaining);
                    match j.time_remaining.cmp(&time_remaining) {
                        Ordering::Less => {
                            panic!("job had less remaining time, queue pulling is wrong")
                        }
                        _ => j.decrease_time(time_remaining),
                    }
                    // println!("{} now has {} left", j.step, j.time_remaining);
                }
            }
        }

        if order_for_steps.len() == total_steps {
            // println!("steps complete after queue processing");
            break;
        }

        // println!("finished queue processing, proceeding to queue generation");

        let mut available_workers: usize = worker_available
            .iter()
            .map(|(_, available)| if *available { 1 } else { 0 })
            .sum();
        // println!("there are {} available workers", available_workers);

        let mut ready_steps: Vec<String> = step_to_blockers
            .clone()
            .into_iter()
            .filter(|(_, blockers)| blockers.is_empty())
            .map(|(c, _)| c)
            .collect();

        ready_steps.sort_unstable();
        // println!(
        //     "time {} has {} steps ready to start",
        //     current_time,
        //     ready_steps.len()
        // );

        if ready_steps.len() == 0 && queue_map.len() == 0 {
            panic!("why are we still running???");
        }

        while available_workers > 0 && ready_steps.len() > 0 {
            let c = ready_steps.remove(0);
            let cost_of_step = time_for_step(&c) + bonus_time;

            step_to_blockers.insert(c.clone(), vec![c.clone()]);
            // println!("{} blocked on itself pending completion", c);

            let worker_id = worker_available
                .clone()
                .into_iter()
                .filter(|(_, available)| *available)
                .map(|(w_id, _)| w_id)
                .collect::<Vec<usize>>()
                .first()
                .unwrap()
                .clone();
            // println!("queuing {} with step {} for {}", worker_id, c, cost_of_step);
            queue_map.insert(
                worker_id.clone(),
                Job {
                    time_remaining: cost_of_step,
                    step: c,
                },
            );
            worker_available.insert(worker_id.clone(), false);
            available_workers -= 1;
        }
    }

    println!("time to complete: {}", current_time);
}

fn time_for_step(step: &String) -> usize {
    match step.to_lowercase().as_str() {
        "a" => 1,
        "b" => 2,
        "c" => 3,
        "d" => 4,
        "e" => 5,
        "f" => 6,
        "g" => 7,
        "h" => 8,
        "i" => 9,
        "j" => 10,
        "k" => 11,
        "l" => 12,
        "m" => 13,
        "n" => 14,
        "o" => 15,
        "p" => 16,
        "q" => 17,
        "r" => 18,
        "s" => 19,
        "t" => 20,
        "u" => 21,
        "v" => 22,
        "w" => 23,
        "x" => 24,
        "y" => 25,
        "z" => 26,
        _ => panic!("step was more than one"),
    }
}
