use std::{collections::VecDeque, fmt::Debug};

use miette::Context;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0;
    let machines = parse_machines(input);
    for machine in machines.iter() {
        result += min_presses_for_machine(machine);
    }
    Ok(result.to_string())
}

fn min_presses_for_machine(machine: &Machine) -> u32 {
    let mut queue = VecDeque::new();

    for i in 0..machine.buttons.len() {
        queue.push_back(PressTracker {
            state: vec![0; machine.joltage.len()],
            press_count: 0,
            next_button: i,
        });
    }

    loop {
        let mut tracker = queue
            .pop_front()
            .wrap_err_with(|| {
                format!(
                    "Error: no more buttons to press but desired pattern not found. {machine:?}"
                )
            })
            .unwrap();

        // Do press
        tracker.press_count += 1;
        for &counter in machine.buttons[tracker.next_button].iter() {
            tracker.state[counter] += 1;
        }

        // Check if this branch is a match or if it can continue
        let mut match_count = 0;
        for (state_val, target_val) in tracker.state.iter().zip(machine.joltage.iter()) {
            match state_val.cmp(target_val) {
                std::cmp::Ordering::Less => {} // Allowed just continue
                std::cmp::Ordering::Equal => match_count += 1,
                std::cmp::Ordering::Greater => continue, // No point the branch is dead
            }
        }
        if match_count == machine.joltage.len() {
            return tracker.press_count;
        }

        for i in 0..machine.buttons.len() {
            let mut new_tracker = tracker.clone();
            new_tracker.next_button = i;
            queue.push_back(new_tracker);
        }
    }
}

#[derive(Debug, Clone)]
struct PressTracker {
    state: Vec<u16>,
    press_count: u32,
    next_button: usize,
}

#[derive(Debug)]
struct Machine {
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u16>,
}

fn parse_machines(input: &str) -> Vec<Machine> {
    input.lines().map(parse_machine).collect()
}

fn parse_machine(line: &str) -> Machine {
    let (_indicators, rest) = line.split_once("]").unwrap();
    let mut buttons = vec![];
    let mut joltage = vec![];
    for part in rest.split(")") {
        let part = &part[2..]; // Remove space and opening bracket
        if *part.as_bytes().last().unwrap() == b'}' {
            // Process joltage info
            for str_value in part[..part.len() - 1].split(",") {
                joltage.push(str_value.parse().unwrap());
            }
        } else {
            // Extract info for button
            buttons.push(part.split(",").map(|x| x.parse().unwrap()).collect());
        }
    }

    Machine { buttons, joltage }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
        assert_eq!(process(input)?, "33");
        Ok(())
    }
}
