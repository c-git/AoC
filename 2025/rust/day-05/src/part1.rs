use miette::{Context, IntoDiagnostic};
use std::collections::BTreeMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0;
    // Start end pairs
    let mut fresh_ranges: BTreeMap<u64, u64> = BTreeMap::new();

    let mut lines = input.lines();

    // Read ranges
    loop {
        let line = lines
            .next()
            .wrap_err("should always have ranges at the top and a empty line in valid input")?;
        if line.trim().is_empty() {
            // Remaining lines are queries
            break;
        }
        let (mut start, mut end) = parse_range(line)?;
        let mut remove_list = vec![];
        for (&existing_start, &existing_end) in fresh_ranges.range(..=end).rev() {
            if start <= existing_end {
                remove_list.push(existing_start);
            } else {
                break;
            }
        }
        for old_start in remove_list {
            let old_end = fresh_ranges
                .remove(&old_start)
                .wrap_err("we know this key must exists we just found it from the map")?;
            start = start.min(old_start);
            end = end.max(old_end);
        }
        fresh_ranges.insert(start, end);
    }

    // Read queries
    for line in lines {
        let query_id: u64 = line
            .parse()
            .into_diagnostic()
            .wrap_err("failed to parse id")?;
        for (&start, &end) in fresh_ranges.range(..=query_id).rev() {
            if (start..=end).contains(&query_id) {
                result += 1;
                break;
            } else if end < query_id {
                // This is not fresh it didn't match any
                break;
            } else {
                // Not a match check the next range
            }
        }
    }

    Ok(result.to_string())
}

fn parse_range(line: &str) -> miette::Result<(u64, u64)> {
    let mut split = line.split("-");
    let start = split
        .next()
        .wrap_err("failed to get first part of range")?
        .parse()
        .into_diagnostic()
        .wrap_err("failed to parse start of range")?;
    let end = split
        .next()
        .wrap_err("failed to get second part of range")?
        .parse()
        .into_diagnostic()
        .wrap_err("failed to parse end of range")?;

    debug_assert!(split.next().is_none(), "unexpected input found in range");
    debug_assert!(start <= end);
    Ok((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        assert_eq!(process(input)?, "3");
        Ok(())
    }
}
