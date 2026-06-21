/// Анатомическая карта тканей — 3D координаты
/// В v1.0: нормализованные координаты [0,1]³ в TissueConfig
/// Полная модель: Visible Human Project или анатомический атлас
pub struct AnatomicalMap {
    pub resolution_mm: f64,
    pub total_volume_liters: f64,
}
