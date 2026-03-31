use crate::{
    checks,
    errors::DqError,
    report::Report,
    rules::Rules,
};
use std::collections::HashMap;
use std::fs;

pub fn run_checks(csv_path: &str, rules_path: &str) -> Result<Report, DqError> {
    let rules_json = fs::read_to_string(rules_path)?;
    let rules: Rules = serde_json::from_str(&rules_json)?;

    let mut reader = csv::Reader::from_path(csv_path)?;
    let header = reader
        .headers()
        .map_err(|e| DqError::Csv(e))?
        .clone();

    // Build column name -> index map
    let mut col_index: HashMap<String, usize> = HashMap::new();
    for (i, name) in header.iter().enumerate() {
        col_index.insert(name.to_string(), i);
    }

    let mut report = Report {
        file: csv_path.to_string(),
        ..Default::default()
    };

    // 1) required columns check (header-level)
    checks::required_columns::run(&mut report, &header, &rules.required_columns);

    // If required columns missing, we can still continue (useful), but checks referencing
    // missing columns will just skip those columns.
    for (row_i0, result) in reader.records().enumerate() {
        let record = result?;
        let row_index_1based = row_i0 + 1;
        report.total_rows += 1;

        // 2) required values
        checks::required_values::run(
            &mut report,
            row_index_1based,
            &record,
            &col_index,
            &rules.required_values,
        );

        // 3) numeric fields
        checks::numeric::run(
            &mut report,
            row_index_1based,
            &record,
            &col_index,
            &rules.numeric_fields,
        );
    }

    report.passed = !report.has_errors();
    Ok(report)
}
