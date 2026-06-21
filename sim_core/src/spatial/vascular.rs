/// Сосудистая сеть — граф перфузии тканей
/// В v1.0: vascular_density в TissueConfig
/// Полная модель: граф сосудов с сопротивлением/потоком
pub struct VascularNetwork {
    pub total_blood_volume_liters: f64,
    pub cardiac_output: f64,
}
