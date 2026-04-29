//! LEMON (MPI Leipzig Mind-Brain-Body) loader — skeleton.
//!
//! Real fetch + .set parsing: TODO. The archived
//! `_archive/v1_pre_2026-04-28/src/ze_bandwise.py` reads `.set` (EEGLAB) files
//! via MNE-Python; the Rust port will need a `.set`/`.fdt` reader (e.g.
//! `mne_io::eeglab` once that crate matures, or a hand-rolled parser).

use crate::{Dataset, DatasetError, DatasetSnapshot, Modality, Result, Subject, SubjectId};
use std::path::PathBuf;

pub struct Lemon {
    cache: PathBuf,
}

impl Lemon {
    pub fn new() -> Self {
        let cache = std::env::var("BIOSENSE_DATASETS_DIR")
            .map(|s| PathBuf::from(s).join("lemon"))
            .unwrap_or_else(|_| {
                home::home_dir()
                    .unwrap_or_else(|| PathBuf::from("/tmp"))
                    .join(".cache/biosense/datasets/lemon")
            });
        Self { cache }
    }
}

// Shim for `home::home_dir` — keep dependency-free; use HOME env.
mod home {
    use std::path::PathBuf;
    pub fn home_dir() -> Option<PathBuf> {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
}

impl Dataset for Lemon {
    fn id(&self) -> &'static str { "lemon" }
    fn license(&self) -> &'static str { "CC-BY-NC" }
    fn description(&self) -> &'static str {
        "LEMON (Leipzig Mind-Brain-Body) — N=227 EEG/MRI/cognitive aging cohort"
    }
    fn local_cache(&self) -> PathBuf { self.cache.clone() }

    fn snapshot(&self) -> Result<DatasetSnapshot> {
        if !self.cache.exists() {
            return Err(DatasetError::CacheMissing(
                self.cache.display().to_string(),
            ));
        }
        Ok(DatasetSnapshot {
            id: self.id(),
            n_subjects: 0, // TODO: scan cache
            modalities: vec![Modality::Eeg],
            local_path: self.cache.display().to_string(),
        })
    }

    fn subjects(&self) -> Result<Vec<Subject>> {
        if !self.cache.exists() {
            return Err(DatasetError::CacheMissing(self.cache.display().to_string()));
        }
        // TODO: enumerate sub-XXX directories
        Ok(Vec::new())
    }

    fn binary_signal(&self, _subject: &SubjectId, modality: Modality) -> Result<Vec<u8>> {
        if modality != Modality::Eeg {
            return Err(DatasetError::ModalityMissing(modality, "lemon".into()));
        }
        // TODO: load .set, pick EEG channel, apply symbolise::median_threshold
        Ok(Vec::new())
    }

    fn fetch(&self, allow_network: bool) -> Result<()> {
        if !allow_network {
            return Err(DatasetError::NetworkNotEnabled);
        }
        // TODO: download from openneuro.org/datasets/ds000221
        Err(DatasetError::Io(
            "LEMON network fetch not yet implemented (Phase 2 backlog)".into(),
        ))
    }
}
