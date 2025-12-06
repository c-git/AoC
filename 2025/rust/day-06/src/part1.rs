use miette::{Context, IntoDiagnostic, bail};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0u64;
    let mut rows = vec![];
    for line in input.lines() {
        rows.push(line.split_whitespace().collect::<Vec<_>>());
    }
    let row_count = rows.len();
    let col_count = rows[0].len();
    debug_assert!(
        rows.iter().all(|x| col_count == x.len()),
        "all rows should have same number of columns"
    );
    for col in 0..col_count {
        let operation = Operation::try_from(rows[row_count - 1][col])?;
        let mut sub_result = operation.default_accumulator();
        for row in rows.iter().take(row_count - 1) {
            let value = row[col]
                .parse()
                .into_diagnostic()
                .wrap_err("failed to convert to number")?;
            sub_result = operation.perform(sub_result, value);
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
        assert_eq!(process(input)?, "4277556");
        Ok(())
    }
}
