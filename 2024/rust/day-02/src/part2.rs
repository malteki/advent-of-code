use itertools::Itertools;

enum CheckedReport {
    /// report is save, content no longer needed
    Safe,
    /// report is not yet flagged as save, content will be checked again (Problem Dampener)
    Unsafe(Vec<i64>),
}

impl CheckedReport {
    fn from_report(report: Vec<i64>) -> Self {
        match report_is_safe(&report) {
            true => Self::Safe,
            false => Self::Unsafe(report),
        }
    }
}

fn report_is_safe(report: &[i64]) -> bool {
    match report.is_sorted_by(|a, b| a <= b) || report.is_sorted_by(|a, b| a >= b) {
        true => {
            // fancy stuff
            let mut iter = report.iter();
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
}

fn with_problem_dampener(mut report: Vec<i64>) -> bool {
    // .rev() for better average performance
    for i in (0..report.len()).rev() {
        let element = report.remove(i);

        if report_is_safe(&report) {
            return true;
        }

        report.insert(i, element);
    }
    false
}

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
        .map(|report| CheckedReport::from_report(report)) // first iter: all levels that work without removing something are CheckedReport::Safe
        // filter_map to get the report as owned value
        .filter_map(|checked_report| match checked_report {
            CheckedReport::Safe => Some(()),
            CheckedReport::Unsafe(report) => with_problem_dampener(report).then_some(()),
        }) // second iter: check the Unsafe reports with "Problem Dampener"
        .count();

    Ok(save_count.to_string())
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
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
