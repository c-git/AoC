#[derive(Debug)]
enum Cell {
    Paper,
    PendingDelete,
    Empty,
}

impl Cell {
    /// Returns `true` if the cell is [`Empty`].
    ///
    /// [`Empty`]: Cell::Empty
    #[must_use]
    fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Returns `true` if the cell is [`Pending_Delete`].
    ///
    /// [`Pending_Delete`]: Cell::Pending_Delete
    #[must_use]
    fn is_pending_delete(&self) -> bool {
        matches!(self, Self::PendingDelete)
    }

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

    let row_count = grid.len();
    let col_count = grid[0].len();

    debug_assert!(
        grid.iter().all(|x| col_count == x.len()),
        "all rows are not the same length"
    );

    let mut is_changed = true;
    while is_changed {
        let result_before = result;
        clear_out_pending(&mut grid);
        for row in 0..row_count {
            for col in 0..col_count {
                if grid[row][col].is_paper() && neighbouring_paper_count(row, col, &grid) < 4 {
                    grid[row][col] = Cell::PendingDelete;
                    result += 1;
                }
            }
        }
        is_changed = result != result_before;
    }
    Ok(result.to_string())
}

fn clear_out_pending(grid: &mut Vec<Vec<Cell>>) {
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            if cell.is_pending_delete() {
                *cell = Cell::Empty;
            }
        }
    }
}

fn neighbouring_paper_count(row: usize, col: usize, grid: &[Vec<Cell>]) -> u8 {
    let mut result = 0;
    // Upper row
    if row > 0 {
        // Left Up
        if col > 0 && !grid[row - 1][col - 1].is_empty() {
            result += 1;
        }

        // Straight Up
        if !grid[row - 1][col].is_empty() {
            result += 1;
        }

        // Right Up
        if col < grid[0].len() - 1 && !grid[row - 1][col + 1].is_empty() {
            result += 1;
        }
    }

    // Left
    if col > 0 && !grid[row][col - 1].is_empty() {
        result += 1;
    }

    // Right
    if col < grid[0].len() - 1 && !grid[row][col + 1].is_empty() {
        result += 1;
    }

    // Lower Row
    if row < grid.len() - 1 {
        // Left Down
        if col > 0 && !grid[row + 1][col - 1].is_empty() {
            result += 1;
        }

        // Straight Down
        if !grid[row + 1][col].is_empty() {
            result += 1;
        }

        // Right Down
        if col < grid[0].len() - 1 && !grid[row + 1][col + 1].is_empty() {
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
        assert_eq!(process(input)?, "43");
        Ok(())
    }
}
