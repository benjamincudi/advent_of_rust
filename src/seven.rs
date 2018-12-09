use std::collections::HashMap;

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

    println!("{} directions found", num_directions);
}
