use std::collections::BTreeMap;

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

    // Calculate ranges of green / red
    let allowed_ranges = AllowedRanges::new(&red_squares);

    // Check each pair (Room to optimize but may not be worth it)
    for (i, first) in red_squares.iter().enumerate().take(red_squares.len() - 1) {
        for second in red_squares.iter().skip(i + 1) {
            if allowed_ranges.included(first, second) {
                result = result.max(get_area(first, second));
            }
        }
    }

    Ok(result.to_string())
}

type Start = i64;
type End = i64;

struct AllowedRanges {
    x_ranges: BTreeMap<Start, End>,
    y_ranges: BTreeMap<Start, End>,
}

impl AllowedRanges {
    fn new(red_squares: &[(i64, i64)]) -> Self {
        let mut result = Self {
            x_ranges: Default::default(),
            y_ranges: Default::default(),
        };
        result.add_range(red_squares.first().unwrap(), red_squares.last().unwrap());
        for i in 0..red_squares.len() - 1 {
            result.add_range(&red_squares[i], &red_squares[i + 1]);
        }
        result
    }

    /// Points must be either in the same column or row otherwise this function panics
    fn add_range(&mut self, point1: &(i64, i64), point2: &(i64, i64)) {
        if point1.0 == point2.0 {
            // Same row
            let (start, end) = if point1.1 < point2.1 {
                (point1.1, point2.1)
            } else {
                (point2.1, point1.1)
            };
            let y = self.y_ranges.entry(start).or_default();
            *y = end.max(*y);
        } else if point1.1 == point2.1 {
            // Same column
            let (start, end) = if point1.0 < point2.0 {
                (point1.0, point2.0)
            } else {
                (point2.0, point1.0)
            };
            let x = self.x_ranges.entry(start).or_default();
            *x = end.max(*x);
        } else {
            unreachable!("input should always be either same column or same row")
        }
    }

    fn included(&self, point1: &(i64, i64), point2: &(i64, i64)) -> bool {
        let (x1, x2, y1, y2) = (point1.0, point2.0, point1.1, point2.1);
        println!(
            "({x1}, {y1}), ({x2}, {y2}) - {}",
            self.test_x(x1, x2) && self.test_y(y1, y2)
        );
        self.test_x(x1, x2) && self.test_y(y1, y2)
    }

    fn test_x(&self, mut x1: i64, mut x2: i64) -> bool {
        if x2 < x1 {
            std::mem::swap(&mut x1, &mut x2);
        }
        Self::test(&self.x_ranges, x1, x2)
    }

    fn test_y(&self, mut y1: i64, mut y2: i64) -> bool {
        if y2 < y1 {
            std::mem::swap(&mut y1, &mut y2);
        }
        Self::test(&self.y_ranges, y1, y2)
    }

    fn test(ranges: &BTreeMap<Start, End>, start: i64, end: i64) -> bool {
        debug_assert!(start <= end);
        for (&range_start, &range_end) in ranges.range(..=start).rev() {
            // Keep going until we hit ranges that will never work
            if start < range_start {
                // Cannot work as not included anymore
                break;
            }
            if end <= range_end {
                return true;
            }
        }
        false
    }
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
