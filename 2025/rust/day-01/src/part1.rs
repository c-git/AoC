use miette::{Context, IntoDiagnostic};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0;
    let mut curr_position = 50;
    for line in input.lines() {
        let sign = if line.starts_with("L") { -1 } else { 1 };
        let clicks: i16 = line[1..]
            .parse()
            .into_diagnostic()
            .wrap_err("failed parsing of number clicks")?;
        curr_position += sign * clicks;
        if curr_position < 0 {
            curr_position += 100;
        }
        if curr_position > 99 {
            curr_position -= 100;
        }
        if curr_position == 0 {
            result += 1;
        }
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        assert_eq!(process(input)?, "3");
        Ok(())
    }
}
