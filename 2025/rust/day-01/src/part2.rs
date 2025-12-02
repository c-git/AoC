use miette::{Context, IntoDiagnostic};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0;
    let mut curr_position = 50;
    for line in input.lines() {
        let did_start_at_zero = curr_position == 0;
        let sign = if line.starts_with("L") { -1 } else { 1 };
        let clicks: i16 = line[1..]
            .parse()
            .into_diagnostic()
            .wrap_err_with(|| format!("failed parsing of number clicks {line:?}"))?;
        curr_position += sign * clicks;
        let rotation_count = curr_position / 100;
        result += rotation_count
            - if rotation_count != 0 && curr_position % 100 == 0 {
                1
            } else {
                0
            };
        curr_position %= 100;
        if curr_position < 0 {
            curr_position += 100;
            if !did_start_at_zero && curr_position != 0 {
                result += 1;
            }
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
L82";
        assert_eq!(process(input)?, "6");
        Ok(())
    }

    #[test]

    fn second_example() -> miette::Result<()> {
        let input = "R1000";
        assert_eq!(process(input)?, "10");
        Ok(())
    }
}
