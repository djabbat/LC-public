//! In-process dataset registry — list known datasets without instantiating loaders.
//!
//! Mirror of `biosense-web/lib/biosense_web_web/live/datasets_live.ex` so the
//! UI and the Rust backend stay in sync on which datasets exist and what their
//! status is.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LoaderStatus {
    Available,
    Pending,
    Placeholder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetEntry {
    pub id: &'static str,
    pub name: &'static str,
    pub modalities: &'static str,
    pub size: &'static str,
    pub license: &'static str,
    pub relevance: &'static str,
    pub status: LoaderStatus,
    pub link: &'static str,
}

pub fn known_datasets() -> Vec<DatasetEntry> {
    vec![
        DatasetEntry {
            id: "lemon",
            name: "LEMON (MPI Leipzig Mind-Brain-Body)",
            modalities: "EEG (resting), ECG, MRI",
            size: "N=227, age 20–77",
            license: "CC-BY-NC",
            relevance: "Direct EEG → χ_Ze(EEG) testbed; aging-correlated cohort",
            status: LoaderStatus::Pending,
            link: "https://www.openneuro.org/datasets/ds000221",
        },
        DatasetEntry {
            id: "cuban",
            name: "Cuban Human Normative EEG (Olmos-Cabrera et al.)",
            modalities: "EEG (resting EC/EO)",
            size: "N=196",
            license: "Open with DUA",
            relevance: "Original article cohort (r=−0.61 with age)",
            status: LoaderStatus::Pending,
            link: "https://www.synapse.org",
        },
        DatasetEntry {
            id: "dortmund",
            name: "Dortmund Vital Study",
            modalities: "EEG (resting), cognitive",
            size: "N≈600",
            license: "Open with DUA",
            relevance: "Aging cohort; was processed by archived `ze_dortmund_pipeline.py`",
            status: LoaderStatus::Pending,
            link: "https://openneuro.org/datasets/ds005385",
        },
        DatasetEntry {
            id: "nhats",
            name: "NHATS",
            modalities: "Wearable accelerometry",
            size: "N≈62k (subset)",
            license: "Public sign-in DUA",
            relevance: "Largest aging-wearable cohort",
            status: LoaderStatus::Pending,
            link: "https://www.nhatsdata.org",
        },
        DatasetEntry {
            id: "allofus_fitbit",
            name: "All of Us — Fitbit subset",
            modalities: "Fitbit",
            size: "N=2,222",
            license: "Researcher Workbench DUA",
            relevance: "External validation (PhenoAge r=0.67 in article)",
            status: LoaderStatus::Pending,
            link: "https://www.researchallofus.org",
        },
        DatasetEntry {
            id: "ukbb",
            name: "UK Biobank — wearable",
            modalities: "AX3 accelerometer",
            size: "N≈103k",
            license: "Full DUA + cost",
            relevance: "Population-scale wearable",
            status: LoaderStatus::Placeholder,
            link: "https://www.ukbiobank.ac.uk",
        },
        DatasetEntry {
            id: "physionet_eeg",
            name: "PhysioNet — resting-state EEG",
            modalities: "EEG resting",
            size: "varies",
            license: "Open with citation",
            relevance: "Multi-cohort EEG benchmarks",
            status: LoaderStatus::Pending,
            link: "https://physionet.org",
        },
        DatasetEntry {
            id: "shhs",
            name: "SHHS",
            modalities: "Polysomnography",
            size: "N≈6,400",
            license: "DUA via NSRR",
            relevance: "Multi-modal night-time data",
            status: LoaderStatus::Pending,
            link: "https://sleepdata.org/datasets/shhs",
        },
        DatasetEntry {
            id: "mesa",
            name: "MESA Sleep",
            modalities: "PSG + actigraphy + HRV",
            size: "N≈2,200",
            license: "DUA via NSRR",
            relevance: "Cross-modality test",
            status: LoaderStatus::Pending,
            link: "https://sleepdata.org/datasets/mesa",
        },
        DatasetEntry {
            id: "dreamer",
            name: "DREAMER",
            modalities: "EEG (Emotiv) + ECG (Empatica)",
            size: "N=23",
            license: "Open with citation",
            relevance: "Joint EEG+HRV pipeline",
            status: LoaderStatus::Pending,
            link: "https://zenodo.org/record/546113",
        },
        DatasetEntry {
            id: "stress_predict",
            name: "Stress-Predict (Empatica E4)",
            modalities: "HRV + EDA + temperature",
            size: "N=35",
            license: "Open",
            relevance: "HRV-only χ_Ze",
            status: LoaderStatus::Pending,
            link: "https://physionet.org/content/stresspredict/1.0/",
        },
        DatasetEntry {
            id: "pmdata",
            name: "PMData",
            modalities: "Fitbit + lifelog",
            size: "N=16, 5 months",
            license: "CC-BY-4.0",
            relevance: "Long-N=1 dense per-participant",
            status: LoaderStatus::Pending,
            link: "https://datasets.simula.no/pmdata",
        },
    ]
}
