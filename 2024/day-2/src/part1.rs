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

    let mut increasing = true;
    let mut decreasing = true;

    for window in report.windows(2) {
        let diff = window[1] as isize - window[0] as isize;
        if !(1..=3).contains(&diff.abs()) {
            return false;
        }
        if diff < 0 {
            increasing = false;
        }
        if diff > 0 {
            decreasing = false;
        }
    }

    increasing || decreasing
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
        assert_eq!("2", process(input)?);
        Ok(())
    }

    #[test]
    fn is_safe_report_works() {
        assert_eq!(is_safe_report(&[1, 2, 3, 4, 5]), true);
        assert_eq!(is_safe_report(&[5, 4, 3, 2, 1]), true);
        assert_eq!(is_safe_report(&[1, 2, 3, 3, 3]), false);
        assert_eq!(is_safe_report(&[1, 2, 3, 5, 5]), false);
        assert_eq!(is_safe_report(&[1, 2, 3, 5, 4]), false);
    }
}
