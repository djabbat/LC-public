//! STOP / NEED regime for pipeline stages — mirror of the Demon Core
//! `GO / ASK / BLOCK / NEED` and the STOP CARD (DEMON_CORE_SPEC §7–8).
//!
//! A pipeline stage that cannot proceed lawfully returns a classified
//! `StageStopCard` instead of fabricating an output. Stopping at the right
//! place is a successful behavior, not a failure.

use serde::{Deserialize, Serialize};

/// The four states of a pipeline run's regime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RegimeState {
    /// Enough structure to continue lawfully.
    Go,
    /// An external fact (a dataset, a measured value) is missing, but a precise
    /// permitted query is possible.
    Ask,
    /// An acting rule/constraint forbids this path.
    Block,
    /// The pipeline's own working structure is missing; a correct query cannot
    /// yet be formed.
    Need,
}

/// Classification of what is missing (open taxonomy, not a closed ontology).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum NeedClass {
    MissingFact,
    MissingRule,
    MissingAddress,
    MissingRelation,
    MissingPattern,
    MissingScope,
    MissingProvenance,
    MissingOperation,
    AmbiguousMatch,
    Conflict,
    ClosureUnknown,
    CreditUnresolved,
}

/// A STOP CARD — a stuck stage returns this instead of a fabricated result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StageStopCard {
    pub stop_id: String,
    pub where_: String,
    pub wanted: String,
    pub available: String,
    pub missing: String,
    pub stop_class: NeedClass,
    /// Whether a precise, permitted external query exists (ASK vs NEED).
    pub can_ask_outside: bool,
    pub query: Option<String>,
}

impl StageStopCard {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        where_: impl Into<String>,
        wanted: &str,
        available: &str,
        missing: &str,
        stop_class: NeedClass,
        can_ask_outside: bool,
        query: Option<String>,
    ) -> Self {
        let where_ = where_.into();
        Self {
            stop_id: format!("stop-{}", hex_fingerprint(&format!("{where_}{missing}"))),
            where_,
            wanted: wanted.to_string(),
            available: available.to_string(),
            missing: missing.to_string(),
            stop_class,
            can_ask_outside,
            query,
        }
    }
}

fn hex_fingerprint(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{:08x}", hash)
}
