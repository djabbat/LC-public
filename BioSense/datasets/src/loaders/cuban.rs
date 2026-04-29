//! Cuban Human Normative EEG (Olmos-Cabrera et al. 2020) — skeleton.
//!
//! Article cohort (Tkemaladze 2026 §4.1, N=196). Archived
//! `_archive/v1_pre_2026-04-28/src/ze_cuban_analysis.py` did the original reduction.

use crate::{Dataset, DatasetError, DatasetSnapshot, Modality, Result, Subject, SubjectId};
use std::path::PathBuf;

pub struct Cuban {
    cache: PathBuf,
}

impl Cuban {
    pub fn new() -> Self {
        let cache = std::env::var("BIOSENSE_DATASETS_DIR")
            .map(|s| PathBuf::from(s).join("cuban"))
            .unwrap_or_else(|_| {
                std::env::var("HOME")
                    .map(|h| PathBuf::from(h).join(".cache/biosense/datasets/cuban"))
                    .unwrap_or_else(|_| PathBuf::from("/tmp/biosense/cuban"))
            });
        Self { cache }
    }
}

impl Dataset for Cuban {
    fn id(&self) -> &'static str { "cuban" }
    fn license(&self) -> &'static str { "Open with DUA" }
    fn description(&self) -> &'static str {
        "Cuban Human Normative EEG — N=196 resting EC/EO; Olmos-Cabrera et al. 2020"
    }
    fn local_cache(&self) -> PathBuf { self.cache.clone() }

    fn snapshot(&self) -> Result<DatasetSnapshot> {
        if !self.cache.exists() {
            return Err(DatasetError::CacheMissing(self.cache.display().to_string()));
        }
        Ok(DatasetSnapshot {
            id: self.id(),
            n_subjects: 0,
            modalities: vec![Modality::Eeg],
            local_path: self.cache.display().to_string(),
        })
    }

    fn subjects(&self) -> Result<Vec<Subject>> {
        if !self.cache.exists() {
            return Err(DatasetError::CacheMissing(self.cache.display().to_string()));
        }
        Ok(Vec::new())
    }

    fn binary_signal(&self, _subject: &SubjectId, modality: Modality) -> Result<Vec<u8>> {
        if modality != Modality::Eeg {
            return Err(DatasetError::ModalityMissing(modality, "cuban".into()));
        }
        Ok(Vec::new())
    }

    fn fetch(&self, allow_network: bool) -> Result<()> {
        if !allow_network {
            return Err(DatasetError::NetworkNotEnabled);
        }
        Err(DatasetError::License(
            "Cuban dataset requires DUA — automated fetch disabled. Sign DUA at synapse.org first.",
        ))
    }
}
