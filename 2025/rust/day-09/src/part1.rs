#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0;

    // Collect input
    let red_squares = input
        .lines()
        .map(|line| {
            let pair: Vec<i64> = line
                .split(",")
                .map(|val| val.parse().expect("failed to parse number"))
                .collect();
            debug_assert_eq!(pair.len(), 2);
            (pair[0], pair[1])
        })
        .collect::<Vec<_>>();

    // Check each pair (Room to optimize but may not be worth it)
    for (i, first) in red_squares.iter().enumerate().take(red_squares.len() - 1) {
        for second in red_squares.iter().skip(i + 1) {
            result = result.max(get_area(first, second));
        }
    }

    Ok(result.to_string())
}

fn get_area(first: &(i64, i64), second: &(i64, i64)) -> u64 {
    let length = (first.0 - second.0).abs() + 1;
    let width = (first.1 - second.1).abs() + 1;
    length as u64 * width as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
        assert_eq!(process(input)?, "50");
        Ok(())
    }
}
