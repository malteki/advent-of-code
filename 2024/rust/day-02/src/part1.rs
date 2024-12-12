/*
The unusual data (your puzzle input) consists of many reports, one report per line. Each report is a list of numbers called levels that are separated by spaces. For example:

The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:

    The levels are either all increasing or all decreasing.
    Any two adjacent levels differ by at least one and at most three.

In the example above, the reports can be found safe or unsafe by checking those rules:

    7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
    1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
    9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
    1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
    8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
    1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.

So, in this example, 2 reports are safe.
 */

use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let reports: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|entry| entry.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect();

    let save_count = reports
        .into_iter()
        .filter(|levels| {
            // decide wether levels is safe or not
            match levels.is_sorted_by(|a, b| a <= b) || levels.is_sorted_by(|a, b| a >= b) {
                true => {
                    // fancy stuff
                    let mut iter = levels.iter();
                    let mut last = iter.next().unwrap();
                    while let Some(next) = iter.next() {
                        let diff = (last - next).abs();
                        if diff < 1 || diff > 3 {
                            return false;
                        }
                        last = next;
                    }

                    return true;
                }
                false => return false,
            }
        })
        .count();

    Ok(dbg!(save_count).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
