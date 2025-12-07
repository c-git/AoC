use miette::Context;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut lines = input.lines();

    let mut beams_counts: Vec<_> = lines
        .next()
        .wrap_err("first row must be present")?
        .chars()
        .map(|c| if c == 'S' { 1 } else { 0 })
        .collect();

    // Iterate through the lines and see where we have splitters
    for line in lines {
        let mut new_beam_counts = beams_counts.clone();
        // Get indices that have splitters
        let splitter_positions: Vec<_> = line
            .char_indices()
            .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
            .collect();
        for splitter_position in splitter_positions {
            new_beam_counts[splitter_position] = 0;
            new_beam_counts[splitter_position - 1] += beams_counts[splitter_position];
            new_beam_counts[splitter_position + 1] += beams_counts[splitter_position];
        }
        std::mem::swap(&mut beams_counts, &mut new_beam_counts);
    }
    let result: u64 = beams_counts.iter().sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
        assert_eq!(process(input)?, "40");
        Ok(())
    }
}
