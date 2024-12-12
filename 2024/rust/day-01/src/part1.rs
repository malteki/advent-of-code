#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut left = vec![];
    let mut right = vec![];

    // TODO: make this fast
    for line in input.lines() {
        let mut elements = line.split_ascii_whitespace();
        left.push((elements.next().unwrap()).parse::<i64>().unwrap());
        right.push((elements.next().unwrap()).parse::<i64>().unwrap());
    }

    left.sort();
    right.sort();

    let result: i64 = left
        .into_iter()
        .zip(right)
        .map(|(a, b)| {
            let dif = (a - b).abs();
            dif
        })
        .sum();

    Ok(result.to_string())
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
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
