use crate::report::{CheckError, Report, Severity};

pub fn run(report: &mut Report, header: &csv::StringRecord, required: &[String]) {
    for col in required {
        if header.iter().all(|h| h != col) {
            report.errors.push(CheckError {
                check: "required_columns".to_string(),
                message: format!("Missing required column '{}'", col),
                row: None,
                column: Some(col.clone()),
                sample: None,
                severity: Severity::Error,
            });
            report.summary.missing_required_columns += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
  
    #[test]
    fn missing_column_is_reported() {
        let header = csv::StringRecord::from(vec!["id", "name"]);
        let mut report = Report::default();

        run(&mut report, &header, &vec!["amount".to_string()]);

        assert_eq!(report.summary.missing_required_columns, 1);
        assert_eq!(report.errors.len(), 1);
        assert!(matches!(report.errors[0].severity, Severity::Error));

    }
}
