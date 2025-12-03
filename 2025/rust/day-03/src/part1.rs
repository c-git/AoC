use miette::Context;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0;
    for line in input.lines() {
        let mut first_num = None;
        let mut second_num = None;
        for c in line.chars() {
            let curr_num = c.to_digit(10).wrap_err("conversion failed")?;
            match (first_num, second_num) {
                (None, None) => first_num = Some(curr_num),
                (None, Some(_)) => unreachable!("always fill first number first"),
                (Some(_), None) => {
                    second_num = Some(curr_num);
                }
                (Some(first), Some(second)) => {
                    if first < second {
                        first_num = second_num;
                        second_num = Some(curr_num)
                    } else if second < curr_num {
                        second_num = Some(curr_num)
                    }
                }
            }
        }
        if let (Some(first), Some(second)) = (first_num, second_num) {
            result += first * 10 + second;
        } else {
            unreachable!("we should always get at least two numbers");
        }
    }
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111
";
        assert_eq!(process(input)?, "357");
        Ok(())
    }
}
