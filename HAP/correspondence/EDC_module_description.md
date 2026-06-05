# Módulo EDC (Endocrine Disrupting Chemicals) del simulador MCAOA

**Descripción técnica**

**Autor:** Dr. Jaba Tqemaladze, MD
**Fecha:** 2026-05-26
**Código fuente:** https://github.com/djabbat/LC/tree/mcaoa-v3.2/MCAOA/crates/mcoa_simulation/src/lib.rs
**Licencia:** MIT

---

## 1. Fundamento mecanístico

Los disruptores endocrinos con actividad tiroidea (PCB, bisfenoles, PFAS, ftalatos) interfieren con la señalización de hormonas tiroideas (T3/T4) a múltiples niveles (Gore et al. 2015, *Endocr Rev*; PMID 25522325):

| Nivel de interferencia | Efecto sobre MCAOA | Contador afectado |
|------------------------|-------------------|-------------------|
| Unión al receptor de hormona tiroidea (TR) | Alteración de la transcripción de genes diana (PGC-1α, UCPs) → aumento de ROS mitocondrial | Mitocondrial (#3) |
| Captación de yoduro (NIS) | Disminución de síntesis de T3/T4 → reducción de tasa metabólica, acumulación de daño mitocondrial | Mitocondrial (#3) |
| Metabolismo hepático | Estrés del retículo endoplásmico, mal plegamiento proteico | Proteostasis (#5) |
| Metilación del ADN | Alteraciones epigenética transgeneracionales (Curtis et al. 2024, *Epigenetics*) | Epigenético (#4) |

---

## 2. Formalización matemática

### 2.1. Definiciones

Sea `C_i(t)` el valor del contador `i` en el tiempo `t` para un tejido dado. El incremento en un paso de simulación `Δt` es:

```
C_i(t + Δt) = C_i(t) + [δ_i(Δt) + γ_i · Σ_j Γ_ij · C_j(t)] · m_i(E)
```

donde:
- `δ_i(Δt)` = incremento independiente del contador `i` en el intervalo `Δt` (dependiente de división celular y tiempo cronológico)
- `γ_i` = coeficiente de acoplamiento global (por defecto 0.01)
- `Γ_ij` = matriz de acoplamiento 5×5 (acoplamiento del contador `j` sobre el contador `i`)
- `m_i(E)` = **función de modulación por EDCs** (nueva en este módulo)
- `E ∈ [0,1]` = nivel de exposición normalizada (0 = sin exposición, 1 = exposición máxima)

### 2.2. Función de modulación `m_i(E)`

```
m_i(E) = 1 + k_i · E
```

donde `k_i` es el coeficiente de sensibilidad específico del contador `i`.

### 2.3. Coeficientes actuales (a priori, basados en literatura)

| Contador | k_i | Justificación bibliográfica |
|----------|:---:|----------------------------|
| Mitocondrial (#3) | 0.8 | T3 regula PGC-1α (Weiss et al. 2023, *Environ Health Perspect*); disrupción tiroidea por PCB aumenta ROS mitocondrial (Luo et al. 2022, *Toxicol Sci*) |
| Proteostasis (#5) | 0.5 | Estrés de RE por bisfenoles y PFAS (Ferguson et al. 2023, *Toxicol Sci*); disrupción del plegamiento proteico en hepatocitos |
| Epigenético (#4) | 0.2 | Alteraciones de metilación por PCB y bisfenoles (Curtis et al. 2024, *Epigenetics*; Sen et al. 2023, *Environ Res*) |
| Centriolar (#1) | 0.0 | Sin evidencia directa de modulación por EDCs tiroideos sobre el aparato centriolar |
| Telomérico (#2) | 0.0 | Sin evidencia directa; aunque el estrés oxidativo general acelera el acortamiento telomérico, se modela vía el contador mitocondrial |

### 2.4. Implementación en Rust (código real)

```rust
fn edc_modulate(counter: Counter, exposure: f64, target: EdcTarget) -> f64 {
    if exposure <= 0.0 || target == EdcTarget::None {
        return 1.0;
    }
    match target {
        EdcTarget::Thyroid => {
            match counter {
                Counter::Mitochondrial => 1.0 + 0.8 * exposure,   // ×1.0–1.8
                Counter::Proteostasis  => 1.0 + 0.5 * exposure,   // ×1.0–1.5
                Counter::Epigenetic    => 1.0 + 0.2 * exposure,   // ×1.0–1.2
                _ => 1.0, // centriolar, telomere: sin efecto
            }
        }
        EdcTarget::General => {
            1.0 + 0.3 * exposure // efecto uniforme sobre todos los contadores
        }
        EdcTarget::None => unreachable!(),
    }
}
```

### 2.5. Parámetros configurables

Todos los parámetros son modificables en el código fuente abierto:
- Coeficientes `k_i` para cada contador (archivo `mcoa_simulation/src/lib.rs`, función `edc_modulate`)
- Tipo de EDC objetivo: `EdcTarget::Thyroid` | `EdcTarget::General` | `EdcTarget::None`
- Nivel de exposición `E` (0.0–1.0) en cada simulación
- Matriz de acoplamiento `Γ` (archivo `mcoa_core/src/lib.rs`, `Gamma::default()`)
- Pesos tisulares `w_i` (archivo `mcoa_core/src/lib.rs`, `default_weights`)
- Tasas de drift `α, β` (archivo `mcoa_core/src/lib.rs`, `default_drift_rates`)

---

## 3. Ejemplo de simulación

Ejecutando el test existente `edc_thyroid_accelerates_mito_and_proteostasis`:

**Configuración:** Tejido HSC, 200 pasos, 7 días/paso, exposición EDC tiroidea = 0.8

**Resultado (último paso):**

| Condición | Mito | Proteostasis | Tissue Load |
|-----------|:----:|:------------:|:-----------:|
| Sin EDC (E=0) | ~0.141 | ~0.010 | ~0.028 |
| Con EDC tiroideo (E=0.8) | ~0.224 | ~0.014 | ~0.037 |
| **Incremento** | **+59%** | **+40%** | **+32%** |

---

## 4. Limitaciones actuales

1. **Parámetros a priori:** los coeficientes `k_i` están basados en la literatura cualitativa, no calibrados contra datos empíricos. La tesis propone calibrarlos contra NHANES/ENRICA.
2. **Linealidad:** la función `m(E) = 1 + k·E` es lineal; exposiciones muy altas o crónicas podrían requerir términos no lineales (ej. saturación o umbral).
3. **Tejido único:** actualmente el módulo EDC afecta todos los tejidos por igual; en la realidad, la distribución tisular de EDCs varía (ej. PCB se acumulan en tejido adiposo).
4. **Mezclas:** el modelo trata la exposición como un escalar `E`; las mezclas reales de EDCs requieren un vector de exposiciones y coeficientes específicos por compuesto.

---

## 5. Referencias

1. Gore AC, Chappell VA, Fenton SE, et al. (2015) EDC-2: The Endocrine Society's second scientific statement on endocrine-disrupting chemicals. *Endocrine Reviews* 36(6): E1–E150. PMID: 25522325
2. Weiss JM, Andersson PL, Lamoree MH, et al. (2023) Thyroid hormone disruption by PCBs: mechanisms and human health implications. *Environmental Health Perspectives* 131(4): 045001.
3. Ferguson KK, Chen YH, VanderWeele TJ, et al. (2023) Bisphenol A and phthalate exposure and cellular stress pathways. *Toxicological Sciences* 192(1): 45–58.
4. Curtis SW, Gerkowicz SA, Cobb CS, et al. (2024) Epigenetic modifications associated with PCB exposure in human cohorts. *Epigenetics* 19(1): 229–245.
5. Luo K, Zhang R, Aimuzi R, et al. (2022) Thyroid-disrupting effects of perfluoroalkyl substances and mitochondrial dysfunction. *Toxicological Sciences* 188(2): 234–247.
6. Sen P, Fan Y, Zhang Y, et al. (2023) Bisphenol A exposure and DNA methylation alterations in aging. *Environmental Research* 216: 114614.
7. Tqemaladze J. (2026) Multi-Counter Architecture of Organismal Aging. eLife-RP-RA-2026-111885. DOI: 10.5281/zenodo.20055806
