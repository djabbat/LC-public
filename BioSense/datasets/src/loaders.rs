//! Per-dataset loaders. Each loader implements the `Dataset` trait.
//! Phase 2 of TODO. Skeletons land first; real fetch logic later.

pub mod lemon;
pub mod cuban;

pub use lemon::Lemon;
pub use cuban::Cuban;
