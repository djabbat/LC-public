/// Миграция кода из старых подпроектов (CEDAR, MCARA)
///
/// Файлы: mcoa_core (MCARA/crates/mcoa_core) → ReferenceScales, DriftRates, Gamma
///        cell_dt   (MCARA/CEDAR/crates/cell_dt_core) → inflammaging, asymmetric_division
///
/// Всего мигрировано: 360+ файлов → 2 модуля с ключевыми структурами
/// Остальные файлы — в _archive/subprojects_concepts/
///
/// Технический долг: ~12 месяцев на полную интеграцию

pub mod mcoa_core;
pub mod cell_dt;

pub use mcoa_core::{ReferenceScales, DriftRates, independent_drift, default_scales};
