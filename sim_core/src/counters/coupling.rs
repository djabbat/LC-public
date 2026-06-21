/// Γ-матрица связей между счётчиками (coupling matrix)
///
/// Γ_{ij} — скорость, с которой повреждение счётчика j ускоряет повреждение счётчика i.
/// Каноническое значение: 0 (независимость). Отклонение требует статистического обоснования.
///
/// Источник: MCAOA preprint (DOI 10.5281/zenodo.20055806)

use crate::counters::CounterState;

/// 4×4 матрица связей (без центриоли — она обрабатывается отдельно)
#[derive(Debug, Clone)]
pub struct GammaMatrix {
    /// Индексы: [i][j] — влияние счётчика j на счётчик i
    pub matrix: [[f64; 4]; 4],
}

impl Default for GammaMatrix {
    /// Априорная Γ-матрица. Известные не-нулевые связи:
    /// - Γ_{mito→telo} = 0.30: АФК ускоряет укорочение теломер (Parrinello 2003)
    /// - Γ_{mito→epi}  = 0.30: NAD⁺/NADH влияет на эпигенетику (Schultz & Sinclair 2019)
    /// Остальные — гипотетические, канонически = 0.
    fn default() -> Self {
        Self {
            matrix: [
                // telo  mito  epi   prot
                [ 0.00, 0.30, 0.05, 0.00 ], // → telomere
                [ 0.00, 0.00, 0.10, 0.10 ], // → mito
                [ 0.00, 0.30, 0.00, 0.00 ], // → epi
                [ 0.00, 0.05, 0.10, 0.00 ], // → proteo
            ],
        }
    }
}

impl GammaMatrix {
    /// Влияние других счётчиков на счётчик i
    pub fn influence(&self, i: usize, states: &[CounterState; 4]) -> f64 {
        (0..4).map(|j| self.matrix[i][j] * states[j].damage).sum()
    }

    /// Нулевая матрица (проверка гипотезы независимости)
    pub fn null() -> Self {
        Self { matrix: [[0.0; 4]; 4] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::counters::{CounterType, CounterParams};

    #[test]
    fn null_matrix_yields_zero_influence() {
        let gamma = GammaMatrix::null();
        let states = [
            CounterState::new(CounterType::Telomere, CounterParams::telomere()),
            CounterState::new(CounterType::Mitochondrial, CounterParams::mitochondrial()),
            CounterState::new(CounterType::Epigenetic, CounterParams::epigenetic()),
            CounterState::new(CounterType::Proteostatic, CounterParams::proteostatic()),
        ];
        for i in 0..4 {
            assert!((gamma.influence(i, &states) - 0.0).abs() < 1e-10);
        }
    }

    #[test]
    fn default_matrix_has_nonzero_mito_telo() {
        let gamma = GammaMatrix::default();
        assert!(gamma.matrix[0][1] > 0.0); // mito → telo
    }
}
