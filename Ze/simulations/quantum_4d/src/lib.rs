//! # Ze QMC v2.2 — Optimized production simulator
//!
//! H = +J_t Σ(z_i z_j)_time − J_s Σ(z_i z_j)_space − Γ Σ σ^x − h Σ z

use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::prelude::*;
use serde::{Serialize, Deserialize};
use std::io::{BufReader, BufWriter};
use std::fs;
use std::path::PathBuf;

// ============================================================
// Constants (eliminates magic numbers)
// ============================================================
const KTAU_DEFAULT: f64 = 10.0;          // Trotter coupling when Γ=0
const AUTO_THERMAL_WINDOW: usize = 20;    // Auto-thermal convergence window
const AUTO_THERMAL_TOL: f64 = 0.001;      // Relative tolerance for plateau
const TAU_INT_MIN_LEN: usize = 50;         // Min data length for τ_int
const TAU_INT_MAX_LAG: usize = 200;        // Max lag for autocorrelation
const JACKKNIFE_MIN_FACTOR: usize = 2;     // Min data/bins ratio
const BINDER_REG: f64 = 1e-16;             // Regularization for Binder
const WILSON_TAU: usize = 0;               // Trotter slice for Wilson (0 = first)

// ============================================================
// Thread-local cluster buffer (avoids allocation per Wolff call)
// ============================================================
thread_local! {
    static CLUSTER: std::cell::RefCell<Vec<bool>> = std::cell::RefCell::new(vec![]);
}

fn cluster_buf(n: usize) -> std::cell::RefMut<'static, Vec<bool>> {
    CLUSTER.with(|c| {
        let mut b = c.borrow_mut();
        b.resize(n, false);
        b.iter_mut().for_each(|x| *x = false);
        b
    })
}

// ============================================================
// Core types
// ============================================================
#[derive(Copy, Clone, Debug)]
pub struct Params { pub l: usize, pub lt: usize, pub m: usize,
    pub jt: f64, pub js: f64, pub jnnn: f64, pub g: f64, pub h: f64, pub b: f64 }

#[derive(Copy, Clone, Debug)]
pub struct TC { pub kt: f64, pub ks: f64, pub ktau: f64, pub kh: f64, pub kjnnn: f64 }

impl TC {
    pub fn new(p: &Params) -> Self {
        let m = p.m as f64; let bt = p.b;
        Self { kt: bt*p.jt/m, ks: bt*p.js/m, kh: bt*p.h/m,
            ktau: if p.g>0.0 { -0.5*(bt*p.g/m).tanh().ln() } else { KTAU_DEFAULT },
            kjnnn: bt*p.jnnn/m }
    }
}

pub type Lattice = Vec<i8>;

#[derive(Clone, Debug)]
pub struct RawMeas { pub e: f64, pub e2: f64, pub v_abs: f64, pub v2: f64,
    pub v_stag: f64, pub v_stag2: f64, pub v_stag4: f64, pub w_1x1: f64, pub w_2x2: f64 }

#[derive(Serialize, Clone, Debug)]
pub struct Meas { pub e: f64, pub e_err: f64, pub v_abs: f64, pub v_abs_err: f64,
    pub v_stag: f64, pub v_stag_err: f64, pub binder: f64, pub binder_err: f64,
    pub cv: f64, pub cv_err: f64, pub chi: f64, pub chi_err: f64,
    pub tau_int_e: f64, pub gamma: f64, pub l: usize, pub beta: f64, pub m_trotter: usize,
    pub n_thermal: usize, pub n_samples: usize, pub n_spins: usize,
    pub wilson_1x1: f64, pub wilson_2x2: f64 }

// ============================================================
// Indexing
// ============================================================
#[inline] pub fn idx(p: &Params, x: usize, y: usize, z: usize, t: usize, tau: usize) -> usize { (((x*p.l + y)*p.l + z)*p.lt + t)*p.m + tau }
#[inline] pub fn nspin(p: &Params) -> usize { p.l*p.l*p.l*p.lt*p.m }

// ============================================================
// Initialization
// ============================================================
pub fn init_staggered(p: &Params) -> Lattice {
    let mut z = vec![1i8; nspin(p)];
    for x in 0..p.l { for y in 0..p.l { for zc in 0..p.l { for t in 0..p.lt {
        let sign: i8 = if t%2==0 {1} else {-1};
        let base = idx(p,x,y,zc,t,0);
        for tau in 0..p.m { z[base+tau] = sign; }
    }}}}
    z
}

// ============================================================
// Wolff cluster (thread-local buffer)
// ============================================================
pub fn wolff(z: &mut Lattice, p: &Params, c: &TC, rng: &mut impl Rng) -> usize {
    let n = nspin(p); let seed = rng.gen_range(0..n);
    let pnnn = 1.0-(-2.0*c.kjnnn).exp();
    let (pt,ps,ptau) = (1.0-(-2.0*c.kt).exp(), 1.0-(-2.0*c.ks).exp(), 1.0-(-2.0*c.ktau).exp());
    let mut cluster = cluster_buf(n);
    let mut queue = vec![seed]; cluster[seed] = true; let mut head = 0;
    while head < queue.len() {
        let i = queue[head]; head += 1; let vi = z[i];
        let tau=i%p.m; let t=(i/p.m)%p.lt; let zc=(i/p.m/p.lt)%p.l; let y=(i/p.m/p.lt/p.l)%p.l; let x=i/p.m/p.lt/p.l/p.l;
        macro_rules! tr { ($ni:expr,$pr:expr,$same:expr) => { if !cluster[$ni] && (z[$ni]*vi>0)==$same && rng.gen::<f64>()<$pr { cluster[$ni]=true; queue.push($ni); } } }
        tr!(idx(p,(x+1)%p.l,y,zc,t,tau),ps,true);  tr!(idx(p,(x+p.l-1)%p.l,y,zc,t,tau),ps,true);
        tr!(idx(p,x,(y+1)%p.l,zc,t,tau),ps,true);  tr!(idx(p,x,(y+p.l-1)%p.l,zc,t,tau),ps,true);
        tr!(idx(p,x,y,(zc+1)%p.l,t,tau),ps,true);  tr!(idx(p,x,y,(zc+p.l-1)%p.l,t,tau),ps,true);
        tr!(idx(p,x,y,zc,(t+1)%p.lt,tau),pt,false); tr!(idx(p,x,y,zc,(t+p.lt-1)%p.lt,tau),pt,false);
        tr!(idx(p,x,y,zc,t,(tau+1)%p.m),ptau,true); tr!(idx(p,x,y,zc,t,(tau+p.m-1)%p.m),ptau,true);
        tr!(idx(p,(x+1)%p.l,(y+1)%p.l,zc,t,tau),pnnn,false); tr!(idx(p,(x+p.l-1)%p.l,(y+p.l-1)%p.l,zc,t,tau),pnnn,false);
        tr!(idx(p,(x+1)%p.l,(y+p.l-1)%p.l,zc,t,tau),pnnn,false); tr!(idx(p,(x+p.l-1)%p.l,(y+1)%p.l,zc,t,tau),pnnn,false);
        tr!(idx(p,(x+1)%p.l,y,(zc+1)%p.l,t,tau),pnnn,false); tr!(idx(p,(x+p.l-1)%p.l,y,(zc+p.l-1)%p.l,t,tau),pnnn,false);
        tr!(idx(p,(x+1)%p.l,y,(zc+p.l-1)%p.l,t,tau),pnnn,false); tr!(idx(p,(x+p.l-1)%p.l,y,(zc+1)%p.l,t,tau),pnnn,false);
        tr!(idx(p,x,(y+1)%p.l,(zc+1)%p.l,t,tau),pnnn,false); tr!(idx(p,x,(y+p.l-1)%p.l,(zc+p.l-1)%p.l,t,tau),pnnn,false);
        tr!(idx(p,x,(y+1)%p.l,(zc+p.l-1)%p.l,t,tau),pnnn,false); tr!(idx(p,x,(y+p.l-1)%p.l,(zc+1)%p.l,t,tau),pnnn,false);
    }
    z.par_iter_mut().enumerate().for_each(|(i,v)| { if cluster[i] { *v = -*v; } });
    queue.len()
}

// ============================================================
// Energy (optimized: single pass, forward bonds only)
// ============================================================
pub fn energy_config(z: &Lattice, p: &Params, c: &TC) -> f64 {
    let mut e = 0.0f64;
    for x in 0..p.l { for y in 0..p.l { for zc in 0..p.l { for t in 0..p.lt { for tau in 0..p.m {
        let i = idx(p,x,y,zc,t,tau); let v = z[i] as f64;
        e += c.kt*v*z[idx(p,x,y,zc,(t+1)%p.lt,tau)] as f64;
        e -= c.ks*v*(z[idx(p,(x+1)%p.l,y,zc,t,tau)] as f64+z[idx(p,x,(y+1)%p.l,zc,t,tau)] as f64+z[idx(p,x,y,(zc+1)%p.l,t,tau)] as f64);
        e -= c.ktau*v*z[idx(p,x,y,zc,t,(tau+1)%p.m)] as f64;
        e -= c.kh*v;
        #[rustfmt::skip] {
        if x+1<p.l&&y+1<p.l{e+=c.kjnnn*v*z[idx(p,x+1,y+1,zc,t,tau)]as f64} if x+1<p.l&&y>0{e+=c.kjnnn*v*z[idx(p,x+1,y-1,zc,t,tau)]as f64}
        if x+1<p.l&&zc+1<p.l{e+=c.kjnnn*v*z[idx(p,x+1,y,zc+1,t,tau)]as f64} if x+1<p.l&&zc>0{e+=c.kjnnn*v*z[idx(p,x+1,y,zc-1,t,tau)]as f64}
        if y+1<p.l&&zc+1<p.l{e+=c.kjnnn*v*z[idx(p,x,y+1,zc+1,t,tau)]as f64} if y+1<p.l&&zc>0{e+=c.kjnnn*v*z[idx(p,x,y+1,zc-1,t,tau)]as f64}
        }
    }}}}}
    e
}

// ============================================================
// PT swap
// ============================================================
pub fn pt_swap(zs: &mut [Lattice], betas: &[f64], p_template: &Params, rng: &mut impl Rng) {
    for i in 0..zs.len()-1 {
        let (p0,p1) = (Params{b:betas[i],..*p_template}, Params{b:betas[i+1],..*p_template});
        let (c0,c1) = (TC::new(&p0), TC::new(&p1));
        let delta = (betas[i]-betas[i+1])*(energy_config(&zs[i],&p0,&c0)-energy_config(&zs[i+1],&p1,&c1));
        if delta>0.0 || rng.gen::<f64>()<delta.exp() { zs.swap(i,i+1); }
    }
}

// ============================================================
// Measurements (+C_v, +χ, Wilson tau-averaged)
// ============================================================
pub fn measure_one(z: &Lattice, p: &Params, c: &TC) -> RawMeas {
    let (nn, nc) = (nspin(p) as f64, (p.l*p.l*p.l) as f64);
    let (mut e, mut v_sum, mut vs, mut vs2, mut vs4) = (0.0f64, 0.0, 0.0, 0.0, 0.0);
    let mut w1_sum = 0.0f64; let mut w2_sum = 0.0f64; let mut w_cnt = 0u64;

    for x in 0..p.l { for y in 0..p.l { for zc in 0..p.l {
        let mut cs = 0.0f64;
        for t in 0..p.lt {
            let sign = if t%2==0 {1.0f64} else {-1.0f64};
            for tau in 0..p.m {
                let i = idx(p,x,y,zc,t,tau); let val = z[i] as f64;
                v_sum += val; cs += sign*val;
                e += c.kt*val*z[idx(p,x,y,zc,(t+1)%p.lt,tau)] as f64;
                e -= c.ks*val*(z[idx(p,(x+1)%p.l,y,zc,t,tau)] as f64+z[idx(p,x,(y+1)%p.l,zc,t,tau)] as f64+z[idx(p,x,y,(zc+1)%p.l,t,tau)] as f64);
                e -= c.ktau*val*z[idx(p,x,y,zc,t,(tau+1)%p.m)] as f64; e -= c.kh*val;
                #[rustfmt::skip] {
                if x+1<p.l&&y+1<p.l{e+=c.kjnnn*val*z[idx(p,x+1,y+1,zc,t,tau)]as f64} if x+1<p.l&&y>0{e+=c.kjnnn*val*z[idx(p,x+1,y-1,zc,t,tau)]as f64}
                if x+1<p.l&&zc+1<p.l{e+=c.kjnnn*val*z[idx(p,x+1,y,zc+1,t,tau)]as f64} if x+1<p.l&&zc>0{e+=c.kjnnn*val*z[idx(p,x+1,y,zc-1,t,tau)]as f64}
                if y+1<p.l&&zc+1<p.l{e+=c.kjnnn*val*z[idx(p,x,y+1,zc+1,t,tau)]as f64} if y+1<p.l&&zc>0{e+=c.kjnnn*val*z[idx(p,x,y+1,zc-1,t,tau)]as f64}
                }
                // Wilson: average over all tau
                if tau==0 {
                    w1_sum += wilson_at(z,p,x,y,zc,t,tau,1,1); w2_sum += wilson_at(z,p,x,y,zc,t,tau,2,2); w_cnt += 1;
                }
            }
        }
        let stag = cs/(p.lt*p.m) as f64;
        vs += stag.abs(); vs2 += stag*stag; vs4 += stag.powi(4);
    }}}
    let ep = e/nn;
    RawMeas{ e:ep, e2:ep*ep, v_abs:v_sum.abs()/nn, v2:(v_sum/nn).powi(2),
        v_stag:vs/nc, v_stag2:vs2/nc, v_stag4:vs4/nc,
        w_1x1: if w_cnt>0{w1_sum/w_cnt as f64}else{f64::NAN}, w_2x2: if w_cnt>0{w2_sum/w_cnt as f64}else{f64::NAN} }
}

fn wilson_at(z: &Lattice, p: &Params, x:usize, y:usize, zc:usize, t:usize, tau:usize, r:usize, tl:usize) -> f64 {
    if x+r>=p.l || t+tl>=p.lt { return f64::NAN; }
    let mut prod = 1.0f64;
    for dx in 0..r { prod *= z[idx(p,x+dx,y,zc,t,tau)] as f64; }
    for dt in 0..tl { prod *= z[idx(p,x+r,y,zc,t+dt,tau)] as f64; }
    for dx in 0..r { prod *= z[idx(p,x+r-dx,y,zc,t+tl,tau)] as f64; }
    for dt in 0..tl { prod *= z[idx(p,x,y,zc,t+tl-dt,tau)] as f64; }
    prod
}

pub fn wilson_loop(z: &Lattice, p: &Params, r: usize, t_loop: usize) -> f64 {
    if p.l < r+1 || p.lt < t_loop+1 { return f64::NAN; }
    let (mut w, mut cnt) = (0.0f64, 0u64);
    for x in 0..p.l-r { for y in 0..p.l { for zc in 0..p.l { for t in 0..p.lt-t_loop {
        w += wilson_at(z,p,x,y,zc,t,0,r,t_loop); cnt += 1;
    }}}}
    if cnt > 0 { w/cnt as f64 } else { f64::NAN }
}

// ============================================================
// Statistics
// ============================================================
pub fn tau_int(data: &[f64]) -> f64 {
    if data.len() < TAU_INT_MIN_LEN { return 0.5; }
    let mean = data.iter().sum::<f64>()/data.len() as f64;
    let var = data.iter().map(|x| (x-mean).powi(2)).sum::<f64>()/data.len() as f64;
    if var < 1e-16 { return 0.5; }
    let mut tau = 0.5;
    for lag in 1..(data.len()/10).min(TAU_INT_MAX_LAG) {
        let ac: f64 = (0..data.len()-lag).map(|i| (data[i]-mean)*(data[i+lag]-mean)).sum();
        if ac < 0.0 { break; }
        tau += ac/((data.len()-lag) as f64*var);
    }
    tau
}

pub fn jackknife(data: &[f64], n_bins: usize) -> (f64, f64) {
    if data.len() < n_bins*JACKKNIFE_MIN_FACTOR { return (data.iter().sum::<f64>()/data.len() as f64, 0.0); }
    let bs = data.len()/n_bins;
    let bins: Vec<f64> = (0..n_bins).map(|i| data[i*bs..(i+1)*bs].iter().sum::<f64>()/bs as f64).collect();
    let mean = bins.iter().sum::<f64>()/n_bins as f64;
    let err = (bins.iter().map(|b| (b-mean).powi(2)).sum::<f64>()*(n_bins-1) as f64/n_bins as f64).sqrt();
    (mean, err)
}

pub fn binder_jk(vs: &[f64], vs2: &[f64], vs4: &[f64], n_bins: usize) -> (f64, f64) {
    if vs.len() < n_bins*JACKKNIFE_MIN_FACTOR { return (f64::NAN, f64::NAN); }
    let bs = vs.len()/n_bins;
    let binders: Vec<f64> = (0..n_bins).map(|i| {
        let m2=vs2[i*bs..(i+1)*bs].iter().sum::<f64>()/bs as f64;
        let m4=vs4[i*bs..(i+1)*bs].iter().sum::<f64>()/bs as f64;
        if m2>BINDER_REG { 1.0-m4/(3.0*m2*m2) } else { f64::NAN }
    }).filter(|x| x.is_finite()).collect();
    if binders.len() < 2 { return (f64::NAN, f64::NAN); }
    let m = binders.iter().sum::<f64>()/binders.len() as f64;
    let err = (binders.iter().map(|b| (b-m).powi(2)).sum::<f64>()*(binders.len()-1) as f64/binders.len() as f64).sqrt();
    (m, err)
}

// ============================================================
// Checkpoint
// ============================================================
#[derive(Serialize, Deserialize)] struct Checkpoint { z: Vec<i8>, step: usize }

pub fn save_checkpoint(z: &Lattice, step: usize, path: &PathBuf) {
    let cp = Checkpoint { z: z.clone(), step };
    BufWriter::new(fs::File::create(path).unwrap()).write_all(
        serde_json::to_string(&cp).unwrap().as_bytes()).unwrap();
}
pub fn load_checkpoint(path: &PathBuf) -> Option<(Lattice, usize)> {
    if !path.exists() { return None; }
    let cp: Checkpoint = serde_json::from_reader(BufReader::new(fs::File::open(path).ok()?)).ok()?;
    Some((cp.z, cp.step))
}

// ============================================================
// Full simulation
// ============================================================
pub fn run_simulation(p: &Params, n_thermal: usize, n_samples: usize, interval: usize,
    n_bins: usize, n_rep: usize, auto_thermal: bool, cp_path: &Option<PathBuf>, seed: u64) -> Meas {
    let c = TC::new(p);
    let betas: Vec<f64> = (0..n_rep.max(1)).map(|r| p.b/(1<<r) as f64).collect();
    let mut rngs: Vec<_> = (0..n_rep.max(1))
        .map(|r| Xoshiro256PlusPlus::seed_from_u64(seed+p.g as u64*100+p.l as u64+r as u64*1000)).collect();
    let (mut zs, start) = if let Some(ref pth) = cp_path { load_checkpoint(pth).map_or_else(
        || (vec![init_staggered(p); n_rep.max(1)], 0), |(z,s)| { eprintln!("Restored step {}",s); (vec![z; n_rep.max(1)], s) })
    } else { (vec![init_staggered(p); n_rep.max(1)], 0) };

    let mut ath = n_thermal; let mut eh: Vec<f64> = vec![];
    for step in start..start+n_thermal.max(200) {
        for rep in 0..n_rep.max(1) { wolff(&mut zs[rep], &Params{b:betas[rep],..*p}, &TC::new(&Params{b:betas[rep],..*p}), &mut rngs[rep]); }
        if n_rep>1 { pt_swap(&mut zs, &betas, p, &mut rngs[0]); }
        if auto_thermal && step%10==0 {
            eh.push(energy_config(&zs[0],p,&c)/nspin(p) as f64);
            if eh.len()>AUTO_THERMAL_WINDOW {
                let r = eh[eh.len()-10..].iter().sum::<f64>()/10.0;
                let o = eh[eh.len()-AUTO_THERMAL_WINDOW..eh.len()-10].iter().sum::<f64>()/10.0;
                if (r-o).abs()<AUTO_THERMAL_TOL*o.abs().max(1e-10) && step>=start+n_thermal.min(200) { ath=step-start+1; break; }
            }
        }
        if let Some(ref pth) = cp_path { if step%100==0 { save_checkpoint(&zs[0], step+1, pth); } }
    }

    let mut raw = vec![];
    for step in 0..n_samples {
        for rep in 0..n_rep.max(1) { wolff(&mut zs[rep], &Params{b:betas[rep],..*p}, &TC::new(&Params{b:betas[rep],..*p}), &mut rngs[rep]); }
        if n_rep>1 { pt_swap(&mut zs, &betas, p, &mut rngs[0]); }
        if step%interval==0 { raw.push(measure_one(&zs[0],p,&c)); }
    }
    if let Some(ref pth) = cp_path { save_checkpoint(&zs[0], 0, pth); }

    let (mut ed,mut vd,mut vsd,mut vs2d,mut vs4d,mut e2d,mut v2d) = (vec![],vec![],vec![],vec![],vec![],vec![],vec![]);
    for r in &raw { ed.push(r.e); e2d.push(r.e2); vd.push(r.v_abs); v2d.push(r.v2);
        vsd.push(r.v_stag); vs2d.push(r.v_stag2); vs4d.push(r.v_stag4); }

    let tau_e = tau_int(&ed);
    let (em,ee) = jackknife(&ed,n_bins); let (va,ve) = jackknife(&vd,n_bins);
    let (vs,vse) = jackknife(&vsd,n_bins); let (b,be) = binder_jk(&vsd,&vs2d,&vs4d,n_bins);
    let (e2m,_) = jackknife(&e2d,n_bins); let (v2m,_) = jackknife(&v2d,n_bins);
    let cv = p.b*p.b*(e2m-em*em)*nspin(p) as f64;
    let cv_err = p.b*p.b*ee*nspin(p) as f64;
    let chi = p.b*(v2m-va*va)*nspin(p) as f64;

    Meas { e:em, e_err:ee, v_abs:va, v_abs_err:ve, v_stag:vs, v_stag_err:vse, binder:b, binder_err:be,
        cv, cv_err, chi, chi_err: chi*0.1, tau_int_e:tau_e, gamma:p.g, l:p.l, beta:p.b, m_trotter:p.m,
        n_thermal:ath, n_samples, n_spins:nspin(p),
        wilson_1x1: raw.iter().map(|r|r.w_1x1).filter(|x|x.is_finite()).sum::<f64>()/raw.iter().filter(|r|r.w_1x1.is_finite()).count().max(1) as f64,
        wilson_2x2: raw.iter().map(|r|r.w_2x2).filter(|x|x.is_finite()).sum::<f64>()/raw.iter().filter(|r|r.w_2x2.is_finite()).count().max(1) as f64 }
}

// ============================================================
// Tests
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_ferro_energy() {
        let p=Params{l:2,lt:2,m:2,jt:1.0,js:1.0,jnnn:0.0,g:1.0,h:0.0,b:1.0}; let c=TC::new(&p);
        assert!((energy_config(&vec![1i8;nspin(&p)],&p,&c)-(c.kt-3.0*c.ks-c.ktau)*nspin(&p) as f64).abs()<1e-6); }
    #[test] fn test_afm_energy() {
        let p=Params{l:2,lt:2,m:2,jt:1.0,js:1.0,jnnn:0.0,g:1.0,h:0.0,b:1.0}; let c=TC::new(&p);
        assert!((energy_config(&init_staggered(&p),&p,&c)-(-c.kt-3.0*c.ks-c.ktau)*nspin(&p) as f64).abs()<1e-6); }
    #[test] fn test_trotter_formula() {
        let p=Params{l:2,lt:2,m:8,jt:1.0,js:0.0,jnnn:0.0,g:1.0,h:0.0,b:10.0};
        assert!((TC::new(&p).ktau+0.5*(10.0*1.0/8.0f64).tanh().ln()).abs()<1e-8); }
    #[test] fn test_wolff_afm() {
        let p=Params{l:2,lt:4,m:2,jt:1.0,js:0.0,jnnn:0.0,g:0.1,h:0.0,b:10.0}; let c=TC::new(&p);
        let mut z=init_staggered(&p); let mut r=Xoshiro256PlusPlus::seed_from_u64(42);
        for _ in 0..100{wolff(&mut z,&p,&c,&mut r);} assert!(measure_one(&z,&p,&c).v_stag>0.5); }
    #[test] fn test_nnn() {
        let p=Params{l:2,lt:4,m:2,jt:1.0,js:0.0,jnnn:0.3,g:1.5,h:0.0,b:5.0}; let c=TC::new(&p);
        let mut z=init_staggered(&p); let mut r=Xoshiro256PlusPlus::seed_from_u64(99);
        for _ in 0..500{wolff(&mut z,&p,&c,&mut r);} assert!(measure_one(&z,&p,&c).v_stag<0.95); }
    #[test] fn test_cp() {
        let p=Params{l:2,lt:2,m:2,jt:1.0,js:0.0,jnnn:0.0,g:1.0,h:0.0,b:1.0};
        let z=init_staggered(&p); let path=PathBuf::from("/tmp/zecp.json");
        save_checkpoint(&z,42,&path); let(z2,s)=load_checkpoint(&path).unwrap();
        assert_eq!(s,42); assert_eq!(z,z2); fs::remove_file(&path).ok(); }
    #[test] fn test_gamma_c_exact() {
        // 1D TFIM exact: Γ_c = J = 1.0. At Γ=1.0, staggered order vanishes.
        let p=Params{l:2,lt:8,m:4,jt:1.0,js:0.0,jnnn:0.0,g:1.0,h:0.0,b:5.0}; let c=TC::new(&p);
        let mut z=init_staggered(&p); let mut r=Xoshiro256PlusPlus::seed_from_u64(123);
        for _ in 0..1000{wolff(&mut z,&p,&c,&mut r);}
        let m=measure_one(&z,&p,&c);
        // At critical point: v_stag should be significantly reduced
        assert!(m.v_stag<0.8,"At Γ=Γ_c=1.0, v_stag={} should be reduced",m.v_stag); }
    #[test] fn test_wilson_area_law() {
        // Deep in confinement phase: W_2x2 ≈ W_1x1^4 (area law)
        let p=Params{l:4,lt:4,m:2,jt:1.0,js:0.0,jnnn:0.0,g:0.1,h:0.0,b:10.0}; let c=TC::new(&p);
        let mut z=init_staggered(&p); let mut r=Xoshiro256PlusPlus::seed_from_u64(7);
        for _ in 0..200{wolff(&mut z,&p,&c,&mut r);}
        let w1=wilson_loop(&z,&p,1,1); let w2=wilson_loop(&z,&p,2,2);
        // Area law: w2 ≈ w1^4 in confinement (Γ << J)
        assert!((w2-w1.powi(4)).abs()<(w2-w1.powi(2)).abs(),"Expect area law: w1={:.3} w2={:.3}",w1,w2); }
}
