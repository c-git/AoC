use miette::{Context, IntoDiagnostic};

/// After looking up Chris's solution https://www.youtube.com/watch?v=kHnuJyl3czA
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
        if !did_start_at_zero && curr_position <= 0 {
            // Passed 0 to get negative
            result += 1;
        }
        let rotation_count = (curr_position / 100).abs();
        result += rotation_count;
        curr_position = curr_position.rem_euclid(100);
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
