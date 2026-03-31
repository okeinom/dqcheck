use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "dqcheck", version, about = "Data quality checks for CSV files")]
pub struct Cli {
    /// Path to CSV file to check
    pub input: String,

    /// Path to rules JSON
    #[arg(long)]
    pub rules: String,

    /// Where to write JSON report
    #[arg(long, default_value = "dq_report.json")]
    pub report: String,
}
