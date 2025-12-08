#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0;
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert_eq!(process(input)?, "");
        Ok(())
    }
}
