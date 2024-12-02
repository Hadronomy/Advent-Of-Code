use miette::*;
use tracing::*;

use crate::parser::*;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let reports = parse_reports(input).expect("failed to parse reports").1;
    let safe_reports = reports.iter().filter(|r| is_safe_report(r)).count();
    Ok(safe_reports.to_string())
}

fn is_safe_report(report: &[usize]) -> bool {
    if report.len() < 2 {
        return true;
    }

    if is_strictly_increasing_or_decreasing(report, None) {
        return true;
    }

    for i in 0..report.len() {
        if is_strictly_increasing_or_decreasing(report, Some(i)) {
            return true;
        }
    }

    false
}

fn is_strictly_increasing_or_decreasing(report: &[usize], skip_index: Option<usize>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    let mut fail_count = 0;

    for i in 0..report.len() - 1 {
        if Some(i) == skip_index {
            continue;
        }
        let next_index = if Some(i + 1) == skip_index {
            i + 2
        } else {
            i + 1
        };
        if next_index >= report.len() {
            break;
        }
        let diff = report[next_index] as isize - report[i] as isize;
        if !(1..=3).contains(&diff.abs()) {
            return false;
        }
        if diff < 0 {
            increasing = false;
        }
        if diff > 0 {
            decreasing = false;
        }
        if !increasing && !decreasing {
            fail_count += 1;
            if fail_count > 1 {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
