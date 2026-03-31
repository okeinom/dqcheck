# 📦 dqcheck — Data Quality CLI in Rust

A lightweight, fast **data-quality validation CLI** for CSV files, built in Rust.

Designed for small teams and on-prem environments that need **reliable, privacy-friendly data checks** before ingestion.

---

## 🚀 Features

- ✅ Validate CSV files using configurable rules
- ✅ Detect:
  - Missing required columns
  - Missing required values
  - Invalid numeric fields
- ⚠️ Supports **warnings vs errors**
- 📊 Outputs:
  - Human-readable summary
  - JSON report for automation
- 🔁 Deterministic, testable, and fast
- 🦀 Built with Rust (safe, efficient, no runtime)

---

## 🛠️ Installation

### Prerequisites
- Rust (latest stable): https://www.rust-lang.org

### Build
```bash
git clone <your-repo-url>
cd dqcheck
cargo build --release
```
Binary will be in:

target/release/dqcheck

▶️ Usage
```bash
dqcheck <input.csv> --rules rules.json --report report.json
```
Example
```bash
cargo run -- data.csv --rules rules.json --report out.json
```
Example rules.json:
```json
{
  "required_columns": ["id", "amount", "name"],
  "required_values": ["id", "name"],
  "numeric_fields": ["amount"]
}
```
📊 Output
Console Output

File: data.csv
Rows: 4
Status: WARN
Summary: missing_columns=0, missing_values=0, numeric_failures=1
Report written to out.json

JSON Report (example)

```json
{
  "file": "data.csv",
  "total_rows": 4,
  "passed": true,
  "errors": [
    {
      "check": "numeric_fields",
      "message": "Value is not a valid integer",
      "row": 2,
      "column": "amount",
      "sample": "xyz",
      "severity": "Warning"
    }
  ],
  "summary": {
    "missing_required_columns": 0,
    "missing_required_values": 0,
    "numeric_parse_failures": 1
  }
}
```

🚦 Exit Codes
Code	Meaning
0	PASS (no issues)
2	WARN (warnings only)
1	FAIL (errors present)

This makes dqcheck easy to integrate into:

CI/CD pipelines
Airflow jobs
Cron-based batch workflows

🧪 Running Tests
```bash
cargo test
```
🧠 Design Philosophy
Fail fast, but informatively
No hidden behavior — explicit rules
Small-org friendly — no cloud dependency
Composable — designed to plug into ETL pipelines

🔜 Roadmap
 Duplicate key detection (--unique)
 Rejects output (rejects.csv)
 Folder input (batch processing)
 Warning vs error configuration
 Support for CSV + JSONL

📦 Tech Stack
clap — CLI parsing
csv — CSV processing
serde — JSON serialization
thiserror — error handling

💡 Use Cases
Pre-ingestion data validation
Data pipeline guardrails
Small org ETL workflows
Privacy-first data processing (on-prem)
