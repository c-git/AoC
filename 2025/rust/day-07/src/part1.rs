use miette::Context;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0;
    let mut lines = input.lines();

    let mut beams: Vec<_> = lines
        .next()
        .wrap_err("first row must be present")?
        .chars()
        .map(|c| if c == 'S' { Cell::Beam } else { Cell::Empty })
        .collect();

    // Iterate through the lines and see where we have splitters
    for line in lines {
        // Get indices that have splitters
        let splitter_positions: Vec<_> = line
            .char_indices()
            .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
            .collect();
        for splitter_position in splitter_positions {
            if beams[splitter_position].is_beam() {
                // A beam has hit a splitter
                result += 1;
                beams[splitter_position - 1] = Cell::Beam;
                beams[splitter_position] = Cell::Empty;
                beams[splitter_position + 1] = Cell::Beam;
            }
        }
    }

    Ok(result.to_string())
}

enum Cell {
    Beam,
    Empty,
}

impl Cell {
    /// Returns `true` if the cell is [`Beam`].
    ///
    /// [`Beam`]: Cell::Beam
    #[must_use]
    fn is_beam(&self) -> bool {
        matches!(self, Self::Beam)
    }
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
        assert_eq!(process(input)?, "21");
        Ok(())
    }
}
