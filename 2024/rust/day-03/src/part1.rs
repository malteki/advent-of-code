fn try_get_digits(input: &str) -> Option<(usize, i64)> {
    for i in (1..4).rev() {
        if let Ok(parsed) = (&input[..i]).parse::<i64>() {
            return Some((i, parsed));
        }
    }
    None
}

// this is big
fn find_next(input: &str, mut detected: Vec<i64>) -> Vec<i64> {
    // look for next "mul(" in code
    match input.find("mul(") {
        Some(index) => {
            let next_input = &input[(index + 4)..];
            if let Some((len, x)) = try_get_digits(next_input) {
                let next_input = &next_input[len..];
                if next_input
                    .chars()
                    .nth(0)
                    .map_or(false, |first| first == ',')
                {
                    let next_input = &next_input[1..];
                    if let Some((len, y)) = try_get_digits(next_input) {
                        let next_input = &next_input[len..];
                        if next_input
                            .chars()
                            .nth(0)
                            .map_or(false, |first| first == ')')
                        {
                            detected.push(x * y);
                            find_next(next_input, detected)
                        } else {
                            find_next(next_input, detected)
                        }
                    } else {
                        find_next(next_input, detected)
                    }
                } else {
                    find_next(next_input, detected)
                }
            } else {
                find_next(next_input, detected)
            }
        }
        None => detected,
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let sum: i64 = find_next(input, vec![]).into_iter().sum();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
