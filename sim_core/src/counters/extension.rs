/// Слоты расширения для будущих счётчиков
///
/// #6 (piRNA) — исключён из v1.0. Оставлен как слот.
/// При добавлении нового счётчика:
/// 1. Добавить вариант в CounterType
/// 2. Создать файл с default_params() и new_state()
/// 3. Обновить coupling.rs (добавить строку/столбец)
/// 4. Обновить tissue::TissueConfig (добавить вес)

/// Зарезервировано для счётчика #6 (piRNA)
/// PMID 38142432 (Parambil 2023) — piRNA в стволовых клетках
/// Статус: эксплораторный, не включён в v1.0
pub const SLOT_PIRNA: &str = "Counter #6 (piRNA) — exploratory, not in v1.0";

/// Все зарезервированные слоты
pub const RESERVED_SLOTS: &[&str] = &[
    SLOT_PIRNA,
    "Counter #7 — TBD",
    "Counter #8 — TBD",
];
