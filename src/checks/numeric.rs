use crate::report::{CheckError, Report, Severity};
use std::collections::HashMap;

pub fn run(
    report: &mut Report,
    row_index_1based: usize,
    record: &csv::StringRecord,
    col_index: &HashMap<String, usize>,
    numeric_fields: &[String],
) {
    for col in numeric_fields {
        if let Some(&idx) = col_index.get(col) {
            let raw = record.get(idx).unwrap_or("").trim();
            if raw.is_empty() {
                // If it's empty, required_values check should catch it (if configured).
                // We'll ignore empties here.
                continue;
            }
            if raw.parse::<i64>().is_err() {
                report.errors.push(CheckError {
                    check: "numeric_fields".to_string(),
                    message: "Value is not a valid integer".to_string(),
                    row: Some(row_index_1based),
                    column: Some(col.clone()),
                    sample: Some(raw.to_string()),
                    severity: Severity::Warning,
                });
                report.summary.numeric_parse_failures += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
  
    #[test]
    fn invalid_numeric_value_is_reported() {
        let record = csv::StringRecord::from(vec!["1", "Alice", "abc"]);
        let mut report = Report::default();
        let col_index: HashMap<String, usize> = vec![
            ("id".to_string(), 0),
            ("name".to_string(), 1),
            ("amount".to_string(), 2),
        ]
        .into_iter()
        .collect();

        run(
            &mut report,
            1,
            &record,
            &col_index,
            &vec!["amount".to_string()],
        );

        assert_eq!(report.summary.numeric_parse_failures, 1);
        assert_eq!(report.errors.len(), 1);
        assert!(matches!(report.has_warnings, Severity::Warning));
    }
}