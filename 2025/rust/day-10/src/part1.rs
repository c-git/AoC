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

fn min_presses_for_machine(machine: &Machine) -> u8 {
    let mut queue = VecDeque::new();
    for i in 0..machine.buttons.len() {
        queue.push_back(PressTracker {
            next_button: i,
            ..Default::default()
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
        // eprintln!("{tracker:?}");
        tracker.press_count += 1;
        tracker.state ^= machine.buttons[tracker.next_button];

        if tracker.state == machine.target {
            return tracker.press_count;
        }
        tracker.pressed += 1 << tracker.next_button; // Mark button as pressed
        // eprintln!("{tracker:?}\n");
        for i in 0..machine.buttons.len() {
            let button_bit_pos = 1 << i;
            if (button_bit_pos & tracker.pressed) == 0 {
                // Button not pressed enqueue to be pressed
                queue.push_back(PressTracker {
                    next_button: i,
                    ..tracker
                });
            }
        }
    }
}

#[derive(Default, Clone, Copy)]
struct PressTracker {
    state: u16,
    press_count: u8,
    pressed: u16,
    next_button: usize,
}

struct Machine {
    /// Bit map of target lights
    target: u16,

    /// Bit map of which lights each button toggles
    buttons: Vec<u16>,
    joltage: Vec<u16>,
}

fn parse_machines(input: &str) -> Vec<Machine> {
    input.lines().map(parse_machine).collect()
}

fn parse_machine(line: &str) -> Machine {
    let (indicators, rest) = line.split_once("]").unwrap();
    let target = indicators[1..]
        .char_indices()
        .fold(0, |acc, (i, c)| if c == '#' { acc + (1 << i) } else { acc });
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
            buttons.push(
                part.split(",")
                    .map(|x| x.parse().unwrap())
                    .fold(0, |acc, x: usize| acc + (1 << x)),
            );
        }
    }

    Machine {
        target,
        buttons,
        joltage,
    }
}

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buttons_iter = self.buttons.iter();
        let mut buttons = format!("{:b}", buttons_iter.next().unwrap());
        for button in buttons_iter {
            buttons.push_str(&format!(", {:b}", button));
        }
        f.debug_struct("Machine")
            .field("target", &format!("{:b}", self.target))
            .field("buttons", &buttons)
            .field("joltage", &self.joltage)
            .finish()
    }
}

impl Debug for PressTracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PressTracker")
            .field("state", &format!("{:b}", self.state))
            .field("press_count", &self.press_count)
            .field("pressed", &self.pressed)
            .field("next_button", &self.next_button)
            .finish()
    }
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
        assert_eq!(process(input)?, "7");
        Ok(())
    }
}
