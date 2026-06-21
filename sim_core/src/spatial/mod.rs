// Пространственная 3D-модель
// Анатомическая карта тканей, сосудистая сеть, иннервация.
// В v1.0: базовая структура. Полная реализация — Phase 4.
// Источник: Visible Human Project, анатомические атласы.

pub mod anatomy;
pub mod vascular;
pub mod innervation;

pub use anatomy::AnatomicalMap;
pub use vascular::VascularNetwork;
pub use innervation::InnervationMap;
