// Ontogenesis v4.1 — src/lib.rs
// Human development simulation 0–25 years (0–300 months)
// Stack: Rust (data pipeline + transition detection) + Three.js (3D render)
//
// Stage 0: Data harmonization layer
// Stage 1: Transition detection (CV/Range algorithm)
// Stage 2: 3D visualization

pub mod data;
pub mod analysis;
pub mod params;

pub use data::ingestion::{DataRecord, DataIngestion, DataType};
pub use data::normalization::AgeGrid;
pub use analysis::transition_detection::{TransitionDetector, Transition, TransitionType};
pub use params::OntogenesisParams;
