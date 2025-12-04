#[derive(Debug)]
enum Cell {
    Paper,
    Empty,
}

impl Cell {
    /// Returns `true` if the cell is [`Paper`].
    ///
    /// [`Paper`]: Cell::Paper
    #[must_use]
    fn is_paper(&self) -> bool {
        matches!(self, Self::Paper)
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '@' => Self::Paper,
            '.' => Self::Empty,
            other => unreachable!("unexpected cell value found: {other:?}"),
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0;
    let mut grid: Vec<Vec<Cell>> = vec![];

    for line in input.lines() {
        grid.push(vec![]);
        let row = grid.last_mut().expect("just inserted one");
        for c in line.chars() {
            row.push(c.into());
        }
    }

    debug_assert!(
        grid.iter().all(|x| x.len() == grid[0].len()),
        "all rows are not the same length"
    );

    for (row, row_values) in grid.iter().enumerate() {
        for (col, cell) in row_values.iter().enumerate() {
            if cell.is_paper() && neighbouring_paper_count(row, col, &grid) < 4 {
                result += 1;
            }
        }
    }
    Ok(result.to_string())
}

fn neighbouring_paper_count(row: usize, col: usize, grid: &[Vec<Cell>]) -> u8 {
    let mut result = 0;
    // Upper row
    if row > 0 {
        // Left Up
        if col > 0 && grid[row - 1][col - 1].is_paper() {
            result += 1;
        }

        // Straight Up
        if grid[row - 1][col].is_paper() {
            result += 1;
        }

        // Right Up
        if col < grid[0].len() - 1 && grid[row - 1][col + 1].is_paper() {
            result += 1;
        }
    }

    // Left
    if col > 0 && grid[row][col - 1].is_paper() {
        result += 1;
    }

    // Right
    if col < grid[0].len() - 1 && grid[row][col + 1].is_paper() {
        result += 1;
    }

    // Lower Row
    if row < grid.len() - 1 {
        // Left Down
        if col > 0 && grid[row + 1][col - 1].is_paper() {
            result += 1;
        }

        // Straight Down
        if grid[row + 1][col].is_paper() {
            result += 1;
        }

        // Right Down
        if col < grid[0].len() - 1 && grid[row + 1][col + 1].is_paper() {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(process(input)?, "13");
        Ok(())
    }
}
