//! mcara-sim — run an MCARA simulation and write per-step records to CSV.
//!
//! Per the mandatory comparison rule (see ~/Desktop/LC/MCARA/CLAUDE.md), every simulation
//! output MUST be paired with an analogous CEDAR run via `scripts/compare_mcara_cedar.py`.

use clap::Parser;
use mcara_core::{Gamma, Tissue};
use mcara_simulation::{run, EdcTarget};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "mcara-sim",
    version = "3.2.0",
    about = "Run a Multi-Counter Architecture simulation with optional EDC modulation.",
    long_about = "Runs a discrete-time MCARA simulation for a given tissue, writing per-step counter \
                   states and tissue load to CSV. Includes optional EDC (endocrine disrupting chemical) \
                   modulation for modelling environmental impacts on aging rates.\n\
                   Reference: Tqemaladze J. (2026) DOI 10.5281/zenodo.20055806"
)]
struct Cli {
    /// Tissue name: fibroblast | hsc | neuron | hepatocyte | beta_cell | cd8_t_memory
    #[arg(long, default_value = "hsc")]
    tissue: String,

    /// Number of division-equivalent steps
    #[arg(long, default_value_t = 100)]
    divisions: usize,

    /// Seconds per division-equivalent (default: 7 days = 604800)
    #[arg(long, default_value_t = 604800.0)]
    seconds_per_division: f64,

    /// EDC exposure level (0.0–1.0). 0 = none, 1 = maximum
    #[arg(long, default_value_t = 0.0)]
    edc_exposure: f64,

    /// EDC target: none | thyroid | general
    #[arg(long, default_value = "none")]
    edc_target: String,

    /// Output CSV path
    #[arg(long, default_value = "mcara_run.csv")]
    output: PathBuf,
}

fn parse_tissue(s: &str) -> Result<Tissue, String> {
    match s {
        "fibroblast" => Ok(Tissue::Fibroblast),
        "hsc" => Ok(Tissue::Hsc),
        "neuron" => Ok(Tissue::Neuron),
        "hepatocyte" => Ok(Tissue::Hepatocyte),
        "beta_cell" => Ok(Tissue::BetaCell),
        "cd8_t_memory" => Ok(Tissue::CD8TMemory),
        other => Err(format!("unknown tissue '{other}'")),
    }
}

fn parse_edc_target(s: &str) -> Result<EdcTarget, String> {
    match s {
        "none" => Ok(EdcTarget::None),
        "thyroid" => Ok(EdcTarget::Thyroid),
        "general" => Ok(EdcTarget::General),
        other => Err(format!("unknown EDC target '{other}'. Use: none | thyroid | general")),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let tissue = parse_tissue(&cli.tissue)?;
    let edc_target = parse_edc_target(&cli.edc_target)?;
    let gamma = Gamma::default();

    let records = run(
        tissue,
        cli.divisions,
        cli.seconds_per_division,
        &gamma,
        cli.edc_exposure,
        edc_target,
    );

    let mut writer = csv::Writer::from_path(&cli.output)?;
    for r in &records {
        writer.serialize(r)?;
    }
    writer.flush()?;

    let last = records.last().unwrap();
    let l_crit = if last.tissue_load >= 0.60 { "⚠️ ABOVE L_crit" } else { "below L_crit" };
    eprintln!(
        "mcara-sim v3.2: {} steps, tissue={}, EDC={}/{}, final L_tissue={:.4} ({})",
        records.len() - 1,
        cli.tissue,
        cli.edc_exposure,
        cli.edc_target,
        last.tissue_load,
        l_crit
    );
    eprintln!("  Output: {}", cli.output.display());
    Ok(())
}
