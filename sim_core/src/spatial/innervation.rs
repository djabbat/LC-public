/// Иннервация — нейронные связи между тканями
/// В v1.0: innervation_density в TissueConfig
/// Полная модель: граф нейронных связей
pub struct InnervationMap {
    pub total_neurons: u64,
    pub synapse_density: f64,
}
