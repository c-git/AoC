/// Based on https://www.youtube.com/watch?v=RyLuE5xFLxw
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

    // Get list of lines
    let mut lines = vec![(red_squares.first().unwrap(), red_squares.last().unwrap())];
    for i in 0..red_squares.len() - 1 {
        lines.push((&red_squares[i], &red_squares[i + 1]));
    }

    // Check each pair
    for (i, first) in red_squares.iter().enumerate().take(red_squares.len() - 1) {
        for second in red_squares.iter().skip(i + 1) {
            // Check for overlap
            if !overlaps_line(first, second, &lines) {
                result = result.max(get_area(first, second));
            }
        }
    }

    Ok(result.to_string())
}

#[expect(clippy::type_complexity)]
fn overlaps_line(
    first: &(i64, i64),
    second: &(i64, i64),
    lines: &[(&(i64, i64), &(i64, i64))],
) -> bool {
    // Overlaps a line if it is both above and below and to the left and to the
    // right. That means it must be crossing the line
    for (line_start, line_end) in lines {
        let is_to_left = first.0.max(second.0) <= line_start.0.min(line_end.0);
        let is_to_right = first.0.min(second.0) >= line_start.0.max(line_end.0);
        let is_above = first.1.max(second.1) <= line_start.1.min(line_end.1);
        let is_below = first.1.min(second.1) >= line_start.1.max(line_end.1);
        if !is_to_left && !is_to_right && !is_above && !is_below {
            return true;
        }
    }

    false
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
        assert_eq!(process(input)?, "24");
        Ok(())
    }
}
