use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Warning,
    Error,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Report {
    pub file: String,
    pub total_rows: usize,
    pub passed: bool,

    pub errors: Vec<CheckError>,
    pub summary: Summary,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Summary {
    pub missing_required_columns: usize,
    pub missing_required_values: usize,
    pub numeric_parse_failures: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckError {
    pub check: String,
    pub message: String,
    pub row: Option<usize>, // 1-based data row index (excluding header)
    pub column: Option<String>,
    pub sample: Option<String>,
    pub severity: Severity,
}



impl Report {
    pub fn has_errors(&self) -> bool {
        self.errors.iter().any(|e| matches!(e.severity, Severity::Error))
    }
    pub fn has_warnings(&self) -> bool {
        self.errors.iter().any(|e| matches!(e.severity, Severity::Warning))
    }
    pub fn exit_code(&self) -> i32 {
        if self.has_errors() {
            1
        } else if self.has_warnings() {
            2
        } else {
            0
        }
    }

    pub fn status(&self) -> &'static str {
    if self.has_errors() {
        "FAIL"
    } else if self.has_warnings() {
        "WARN"
    } else {
        "PASS"
    }
}
}