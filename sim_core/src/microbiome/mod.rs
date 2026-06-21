// Микробиом — расширяемый модуль
// Моделирует микробные сообщества кишечника, кожи, ротовой полости.
// INFOGEST-совместимость: Brodkorb 2019 (DOI 10.1038/s41596-018-0119-1)
// В v1.0: базовая структура. Полная реализация — Phase 3.

pub mod gut;
pub mod skin;
pub mod oral;

pub use gut::GutMicrobiome;
pub use skin::SkinMicrobiome;
pub use oral::OralMicrobiome;
