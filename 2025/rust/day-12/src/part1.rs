/// Just couldn't come up with an approach so went to watch Chris' video and...
/// yeah still no "approach" but going to follow after what I understood from
/// the video and see how it works out for me
/// https://www.youtube.com/watch?v=QX7w2oJzX9Y
///
/// NOTE: Does not work on the test input given
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0;

    let (shapes, regions) = parse(input);
    for region in regions {
        if can_fit(region, &shapes) {
            result += 1;
        }
    }
    Ok(result.to_string())
}

fn can_fit(region: Region, shapes: &[Shape]) -> bool {
    let needed = region
        .requirements
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &x)| acc + shapes[i].expand(x));
    region.size >= needed
}

fn parse(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let mut shapes = vec![];
    let mut regions = vec![];
    let mut region_start = 0;
    let mut partial_shape_count = 0;
    for (i, c) in input.char_indices().skip(2) {
        if c.is_numeric() {
            region_start = i;
            shapes.push(Shape(partial_shape_count));
            partial_shape_count = 0;
            continue;
        }
        if c == 'x' {
            // We are no in the regions area
            break;
        }
        if c == '#' {
            partial_shape_count += 1;
        }
    }

    for line in input[region_start..].lines() {
        let (first_dimension, rest) = line.split_once("x").unwrap();
        let (second_dimension, rest) = rest.split_once(":").unwrap();
        let size =
            first_dimension.parse::<usize>().unwrap() * second_dimension.parse::<usize>().unwrap();
        let requirements = rest
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        regions.push(Region { size, requirements });
    }
    (shapes, regions)
}

#[derive(Debug)]
struct Shape(u8);
impl Shape {
    fn expand(&self, x: u8) -> usize {
        self.0 as usize * x as usize
    }
}

#[derive(Debug)]
struct Region {
    size: usize,
    requirements: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";
        assert_eq!(process(input)?, "2");
        Ok(())
    }
}
