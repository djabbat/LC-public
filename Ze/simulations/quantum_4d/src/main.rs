//! Ze QMC v2.1 — CLI

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version = "2.1")]
struct Cli {
    #[arg(short='L', long, default_value = "4")] size: usize,
    #[arg(short='t', long, default_value = "6")] lt: usize,
    #[arg(short='m', long, default_value = "16")] trotter: usize,
    #[arg(long, default_value = "1.0")] jt: f64,
    #[arg(long, default_value = "0.0")] js: f64,
    #[arg(long, default_value = "0.0")] jnnn: f64,
    #[arg(short='G', long, default_value = "1.0")] gamma: f64,
    #[arg(short='H', long, default_value = "0.0")] h: f64,
    #[arg(short='b', long, default_value = "10.0")] beta: f64,
    #[arg(long, default_value = "500")] thermal: usize,
    #[arg(long, default_value = "2000")] samples: usize,
    #[arg(long, default_value = "10")] interval: usize,
    #[arg(long)] scan: Option<String>,
    #[arg(long)] fss: bool,
    #[arg(long, default_value = "42")] seed: u64,
    #[arg(long, default_value = "1")] pt_replicas: usize,
    #[arg(long)] json: bool,
    #[arg(long, default_value = "20")] n_bins: usize,
    #[arg(long)] auto_thermal: bool,
    #[arg(long)] trotter_extrap: bool,
    #[arg(long)] checkpoint: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let gammas: Vec<f64> = if let Some(ref s) = cli.scan {
        s.split(',').map(|x| x.trim().parse().unwrap()).collect()
    } else { vec![cli.gamma] };
    let ls: Vec<usize> = if cli.fss { vec![4,6,8] } else { vec![cli.size] };
    
    let p0 = ze_qmc_4d::Params{l:ls[0], lt:cli.lt, m:cli.trotter,
        jt:cli.jt, js:cli.js, jnnn:cli.jnnn, g:cli.gamma, h:cli.h, b:cli.beta};
    let n_spins = ze_qmc_4d::nspin(&p0);
    
    if !cli.json {
        let total = gammas.len() * ls.len();
        println!("Ze QMC v2.1 | {} spins (i8) | PT={} | {} simulations",
            n_spins, cli.pt_replicas, total);
        if cli.fss { println!("FSS: L = {:?}", ls); }
        println!("{:>4} {:>6} {:>10} {:>10} {:>10} {:>10} {:>7} {:>5}  W(1)/W(2)",
                 "L","Γ","|v|","v_stag","E/N","Binder/Cv","τ_int","Фаза");
        println!("{}","─".repeat(82));
    }
    
    let pb = if !cli.json { Some(ProgressBar::new((gammas.len()*ls.len()) as u64)
        .with_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:30.cyan}] {pos}/{len} {msg}")
            .unwrap())) } else { None };
    
    let mut all = vec![];
    for &l in &ls {
        for &g in &gammas {
            let p = ze_qmc_4d::Params { l, lt:cli.lt, m:cli.trotter,
                jt:cli.jt, js:cli.js, jnnn:cli.jnnn, g, h:cli.h, b:cli.beta };
            
            if let Some(ref pb) = pb { pb.set_message(format!("L={} Γ={:.2}", l, g)); }
            
            let m = if cli.trotter_extrap {
                let m1 = ze_qmc_4d::run_simulation(&p, cli.thermal, cli.samples, cli.interval,
                    cli.n_bins, cli.pt_replicas, cli.auto_thermal, &cli.checkpoint, cli.seed);
                let p_half = ze_qmc_4d::Params { m: cli.trotter/2, ..p };
                let m2 = ze_qmc_4d::run_simulation(&p_half, cli.thermal, cli.samples, cli.interval,
                    cli.n_bins, cli.pt_replicas, cli.auto_thermal, &cli.checkpoint, cli.seed);
                ze_qmc_4d::Meas { e: 2.0*m1.e-m2.e, e_err: (4.0*m1.e_err.powi(2)+m2.e_err.powi(2)).sqrt(),
                    v_abs: 2.0*m1.v_abs-m2.v_abs, v_abs_err: m1.v_abs_err,
                    v_stag: 2.0*m1.v_stag-m2.v_stag, v_stag_err: m1.v_stag_err,
                    binder: 2.0*m1.binder-m2.binder, binder_err: m1.binder_err,
                    tau_int_e: m1.tau_int_e, gamma: g, l, beta: m1.beta, m_trotter: 0,
                    n_thermal: m1.n_thermal, n_samples: m1.n_samples, n_spins: m1.n_spins,
                    wilson_1x1: 2.0*m1.wilson_1x1-m2.wilson_1x1, wilson_2x2: 2.0*m1.wilson_2x2-m2.wilson_2x2 }
            } else {
                ze_qmc_4d::run_simulation(&p, cli.thermal, cli.samples, cli.interval,
                    cli.n_bins, cli.pt_replicas, cli.auto_thermal, &cli.checkpoint, cli.seed)
            };
            
            let phase = if m.v_stag>0.3 {"АФМ"} else if m.v_abs<0.2 {"пара"} else {"крит"};
            let w = if m.wilson_1x1.is_finite() && m.wilson_2x2.is_finite() {
                if (m.wilson_2x2-m.wilson_1x1.powi(2)).abs() < (m.wilson_2x2-m.wilson_1x1.powi(4)).abs() {"deconf"} else {"conf"}
            } else {"?"};
            
            if cli.json { all.push(m.clone()); }
            else { println!("{:4} {:6.2} {:10.4} {:10.4} {:10.4} {:10.4} {:7.2} {:>5}  {:.3}/{:.3} {}",
                l,g,m.v_abs,m.v_stag,m.e,m.binder,m.tau_int_e,phase,m.wilson_1x1,m.wilson_2x2,w); }
            all.push(m);
            if let Some(ref pb) = pb { pb.inc(1); }
        }
    }
    if let Some(ref pb) = pb { pb.finish_with_message("Done"); }
    if cli.json { println!("{}", serde_json::to_string_pretty(&all).unwrap()); }
}
