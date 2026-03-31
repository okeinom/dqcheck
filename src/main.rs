use clap::Parser;
use dqcheck::{cli::Cli, runner::run_checks};
use std::process;

fn main() {
    let cli = Cli::parse();

    match run_checks(&cli.input, &cli.rules) {
        Ok(report) => {
            // Human summary
            println!("File: {}", report.file);
            println!("Rows: {}", report.total_rows);
            println!("Passed: {}", report.passed);
            println!("Status: {}", report.status());
            println!(
                "Summary: missing_columns={}, missing_values={}, numeric_failures={}",
                report.summary.missing_required_columns,
                report.summary.missing_required_values,
                report.summary.numeric_parse_failures
            );

            // Write JSON report
            let json = serde_json::to_string_pretty(&report).expect("serialize report");
            if let Err(e) = std::fs::write(&cli.report, json) {
                eprintln!("Failed to write report {}: {}", cli.report, e);
                process::exit(1);
            }
            println!("Report written to {}", cli.report);

            process::exit(report.exit_code());
        }
        Err(e) => {
            eprintln!("dqcheck failed: {}", e);
            process::exit(1);
        }
    }
}
