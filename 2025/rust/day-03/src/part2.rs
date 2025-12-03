use miette::Context;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    const BATTERY_COUNT: usize = 12;
    let mut result = 0u64;
    let mut batteries = vec![];
    for line in input.lines() {
        for c in line.chars() {
            let curr_num = c.to_digit(10).wrap_err("conversion failed")?;

            if batteries.len() == BATTERY_COUNT {
                // See if we need to remove any small numbers
                let remove_idx =
                    (0..batteries.len() - 1).find(|&i| batteries[i] < batteries[i + 1]);
                if let Some(index) = remove_idx {
                    batteries.remove(index);
                }
            }

            if batteries.len() < BATTERY_COUNT {
                batteries.push(curr_num);
            } else if &curr_num
                > batteries
                    .last()
                    .expect("must exist as it's not less than count")
            {
                // Add new bigger number to the end
                batteries.pop();
                batteries.push(curr_num);
            }
        }
        assert_eq!(batteries.len(), BATTERY_COUNT);
        result += batteries.drain(..).fold(0u64, |acc, x| acc * 10 + x as u64);
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
        assert_eq!(process(input)?, "3121910778619");
        Ok(())
    }
}
