use std::fmt::Debug;

use z3::{Solver, ast::Int};

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
    // Create mapping from counter to the buttons that control it
    let mut counter_buttons = vec![vec![]; machine.joltage.len()];
    for (i, button) in machine.buttons.iter().enumerate() {
        for &controller in button {
            counter_buttons[controller].push(i);
        }
    }

    // instantiate a Solver
    let solver = Solver::new();

    // Create ints for the number of button presses
    let buttons_ints: Vec<_> = (0..machine.buttons.len())
        .map(|i| Int::new_const(i as i32))
        .collect();

    // encode the constraints of the problem as Bool-valued Asts
    // and assert them in the solver
    for button_int in buttons_ints.iter() {
        solver.assert(button_int.ge(0));
    }
    for (i, &joltage) in machine.joltage.iter().enumerate() {
        if let Some(&first) = counter_buttons[i].first() {
            let mut sum = buttons_ints[first].clone();
            for &button_index in counter_buttons[i].iter().skip(1) {
                sum += buttons_ints[button_index].clone();
            }
            solver.assert(sum.eq(joltage));
        }
    }

    for solution in solver.solutions(&buttons_ints, false).take(2) {
        let solution: Vec<u64> = solution
            .iter()
            .map(Int::as_u64)
            .map(Option::unwrap)
            .collect();
        dbg!(format!("{solution:?}"));
    }
    1
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
