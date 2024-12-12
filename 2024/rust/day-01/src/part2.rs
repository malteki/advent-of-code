use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut elements = line.split_ascii_whitespace();
        left.push((elements.next().unwrap()).parse::<u128>().unwrap());
        right.push((elements.next().unwrap()).parse::<u128>().unwrap());
    }

    right.sort();
    let right_counts = right.into_iter().counts();

    let similarity_score: u128 = left
        .into_iter()
        .map(|value| value * (*right_counts.get(&value).unwrap_or(&0) as u128))
        .sum();

    Ok(dbg!(similarity_score).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
