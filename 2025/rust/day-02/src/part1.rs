use miette::{Context, IntoDiagnostic};

/// After watching solution in https://www.youtube.com/watch?v=LTT93lHogRM
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let ranges = input.split(",").map(|pair| {
        let mut boundary_iter = pair.split("-").map(|x| {
            x.trim()
                .parse::<u64>()
                .into_diagnostic()
                .wrap_err_with(|| format!("failed to parse {x:?}"))
                .unwrap()
        });
        let start = boundary_iter.next().expect("failed to get start");
        let end = boundary_iter.next().expect("failed to get end");
        assert!(boundary_iter.next().is_none());
        start..=end
    });

    let mut result = 0u64;
    for id in ranges.flatten() {
        if is_invalid(id) {
            result += id;
        }
    }
    Ok(result.to_string())
}

fn is_invalid(id: u64) -> bool {
    let id = id.to_string();
    if !id.len().is_multiple_of(2) {
        return false;
    }
    let half = id.len() / 2;
    id[..half] == id[half..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(process(input)?, "1227775554");
        Ok(())
    }
}
