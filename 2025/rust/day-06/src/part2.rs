use miette::{Context, IntoDiagnostic, bail};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0u64;

    // Convert the input into a addressable matrix
    let mut lines: Vec<_> = input.lines().collect();
    debug_assert!(lines.iter().all(|x| lines[0].len() == x.len()));

    // Stores the operations and their starting index in the string
    let operations = lines
        .pop()
        .wrap_err("last row must contain the operations")?;
    let operations: Vec<(usize, Operation)> = operations
        .char_indices()
        .filter_map(|(i, c)| {
            if c == ' ' {
                None
            } else {
                Some((
                    i,
                    Operation::try_from(c).expect("valid input only has space and two operators"),
                ))
            }
        })
        .collect();

    // For each Operation found find the solution to the math problem.
    // Going from right to left to allow taking the rest of the string to end
    for (start_idx, operation) in operations.into_iter().rev() {
        let mut sub_result = operation.default_accumulator();

        debug_assert!(lines.iter().all(|x| lines[0].len() == x.len()));
        for char_idx in start_idx..lines[0].len() {
            let mut col_value = 0;
            // FIX: Ensure once we numbers are contiguous (spaces ignored for now)
            for &line in lines.iter() {
                let char = line[char_idx..=char_idx].trim();
                if !char.is_empty() {
                    let num: u64 = char
                        .parse()
                        .into_diagnostic()
                        .wrap_err("failed to convert to number")?;
                    col_value = col_value * 10 + num;
                }
            }
            sub_result = operation.perform(sub_result, col_value);
        }

        // Cut of used part of input for next loop
        if start_idx > 0 {
            for line in lines.iter_mut() {
                *line = &line[..start_idx - 1]; // Minus one to remove trailing space
            }
        }

        result += sub_result;
    }

    Ok(result.to_string())
}

enum Operation {
    Add,
    Multiply,
}

impl Operation {
    #[must_use]
    fn perform(&self, x: u64, y: u64) -> u64 {
        match self {
            Operation::Add => x + y,
            Operation::Multiply => x * y,
        }
    }

    fn default_accumulator(&self) -> u64 {
        match self {
            Operation::Add => 0,
            Operation::Multiply => 1,
        }
    }
}

impl TryFrom<&str> for Operation {
    type Error = miette::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "+" => Self::Add,
            "*" => Self::Multiply,
            other => bail!("unexpected operation found: {other:?}"),
        })
    }
}

impl TryFrom<char> for Operation {
    type Error = miette::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        // Just copied for simplicity
        Ok(match value {
            '+' => Self::Add,
            '*' => Self::Multiply,
            other => bail!("unexpected operation found other char: {other:?}"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
        assert_eq!(process(input)?, "3263827");
        Ok(())
    }
}
