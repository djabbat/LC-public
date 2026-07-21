//! mcara-compare-cedar — MANDATORY MCARA vs CEDAR comparison harness.
//!
//! Rust port of `scripts/compare_mcara_cedar.py`.
//!
//! Per project rule (`feedback_mcara_cedar_comparison`): every MCARA simulation run
//! MUST be paired with an analogous CEDAR run and a Δ report filed to
//! `docs/comparisons/YYYY-MM-DD_label.md`.

use clap::Parser;
use mcara_compare::{compare_mcara_cedar, CompareArgs};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "mcara-compare-cedar", about = "MCARA vs CEDAR comparison harness")]
struct Cli {
    #[arg(long)]
    mcara_csv: PathBuf,
    #[arg(long)]
    cedar_csv: PathBuf,
    #[arg(long)]
    tissue: String,
    #[arg(long)]
    label: String,
    #[arg(long, default_value = "../docs/comparisons/")]
    out_dir: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let c = Cli::parse();
    let args = CompareArgs {
        mcara_csv: &c.mcara_csv,
        cedar_csv: &c.cedar_csv,
        tissue: &c.tissue,
        label: &c.label,
        out_dir: &c.out_dir,
    };
    compare_mcara_cedar(args)?;
    Ok(())
}
