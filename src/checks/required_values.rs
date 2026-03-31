use crate::report::{CheckError, Report, Severity};
use std::collections::HashMap;

pub fn run(
    report: &mut Report,
    row_index_1based: usize,
    record: &csv::StringRecord,
    col_index: &HashMap<String, usize>,
    required_values: &[String],
) {
    for col in required_values {
        if let Some(&idx) = col_index.get(col) {
            let value = record.get(idx).unwrap_or("").trim();
            if value.is_empty() {
                report.errors.push(CheckError {
                    check: "required_values".to_string(),
                    message: "Required value is empty".to_string(),
                    row: Some(row_index_1based),
                    column: Some(col.clone()),
                    sample: None,
                    severity: Severity::Error,
                });
                report.summary.missing_required_values += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests { 
    use super::*;
  
    #[test]
    fn missing_value_is_reported() {
        let record = csv::StringRecord::from(vec!["1", "Alice", ""]);
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

        assert_eq!(report.summary.missing_required_values, 1);
        assert_eq!(report.errors.len(), 1);
        assert!(matches!(report.errors[0].severity, Severity::Error));
    }
}