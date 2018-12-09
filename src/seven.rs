pub fn part_one(file_contents: &String) -> () {
    let mut num_directions: usize = 0;

    let direction_tuples: Vec<(String, String)> = file_contents
        .clone()
        .as_mut_str()
        .split("\n")
        .map(|d_str| {
            let lower_step = d_str.to_lowercase();
            num_directions += 1;
            let steps: Vec<&str> = lower_step.split("step ").collect();
            let blocker = steps[1][0..1].to_string();
            let blockee = steps[2][0..1].to_string();

            return (blocker, blockee);
        }).collect();

    println!("{} directions found", num_directions);
}
