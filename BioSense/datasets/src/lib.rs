//! BioSense datasets — loaders behind a unified `Dataset` trait.
//!
//! All loaders are *opt-in network*: by default they read from a local cache directory
//! (`~/.cache/biosense/datasets/<id>/`). Network fetch is a separate, explicit method.
//!
//! See `MIGRATION_NOTES.md` for the symbolisation conventions discovered during
//! the 2026-04-28 audit of the archived Python pipelines.

pub mod registry;
pub mod loaders;
pub mod symbolise;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatasetError {
    #[error("cache directory not found at {0} — run `fetch()` or set BIOSENSE_DATASETS_DIR")]
    CacheMissing(String),
    #[error("subject `{0}` not found in dataset `{1}`")]
    SubjectMissing(String, &'static str),
    #[error("modality `{0:?}` not available for subject `{1}`")]
    ModalityMissing(Modality, String),
    #[error("network fetch is opt-in — pass `allow_network=true` to enable")]
    NetworkNotEnabled,
    #[error("io: {0}")]
    Io(String),
    #[error("license: {0}")]
    License(&'static str),
}

pub type Result<T> = std::result::Result<T, DatasetError>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Modality {
    Eeg,
    Hrv,
    Respiration,
    Sleep,
    Actigraphy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub id: SubjectId,
    pub age: Option<f64>,
    pub sex: Option<&'static str>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetSnapshot {
    pub id: &'static str,
    pub n_subjects: usize,
    pub modalities: Vec<Modality>,
    pub local_path: String,
}

pub trait Dataset {
    fn id(&self) -> &'static str;
    fn license(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn local_cache(&self) -> std::path::PathBuf;

    /// Read what's locally cached. Returns `Err(CacheMissing)` if nothing is there.
    fn snapshot(&self) -> Result<DatasetSnapshot>;

    /// List subjects available locally.
    fn subjects(&self) -> Result<Vec<Subject>>;

    /// Read raw signal samples for `subject` and `modality`. Already symbolised
    /// to `{0, 1}` by the loader — see `symbolise::*` for the canonical rules.
    fn binary_signal(&self, subject: &SubjectId, modality: Modality) -> Result<Vec<u8>>;

    /// Opt-in network fetch.
    fn fetch(&self, allow_network: bool) -> Result<()>;
}
