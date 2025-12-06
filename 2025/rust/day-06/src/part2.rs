#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    todo!("day-06 - part 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "not on second part yet"]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert_eq!(process(input)?, "");
        Ok(())
    }
}
