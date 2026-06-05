# Propuesta de tesis v2 — MCAOA como hipótesis computacional a evaluar críticamente

**Programa:** Doctorado en Ciencias Biomédicas y Salud Pública (código 9620)
**Equipo 8, Línea 3:** Bases moleculares-celulares y fisiopatología de las enfermedades crónicas
**Directora propuesta:** Dra. Mónica Torres Ruiz (ISCIII)
**Codirector propuesto:** Dr. Antonio De la Vieja (ISCIII)
**Candidato:** Dr. Jaba Tqemaladze, MD (Georgia Longevity Alliance)
**Versión:** 2 (2026-05-26)

---

## Cambio fundamental respecto a la versión 1

En esta versión, **MCAOA se formula como una hipótesis computacional que debe ser evaluada críticamente** mediante comparación con modelos alternativos sobre datos reales, no como un modelo previamente validado. La tesis no asume que MCAOA es correcto — su objetivo es determinar si aporta valor predictivo más allá de modelos más simples.

---

## 1. Preguntas de investigación

**Pregunta principal:**
¿Puede un modelo multi-contador (MCAOA) con modulación por disruptores endocrinos tiroideos mejorar la predicción de trayectorias de envejecimiento patológico (mortalidad, fragilidad, enfermedades crónicas) en comparación con modelos de un solo contador (telómero, epigenético) o modelos de regresión logística tradicionales?

**Preguntas secundarias:**
1. ¿Qué contador(es) de MCAOA tienen mayor poder predictivo para cada tipo de patología crónica?
2. ¿La incorporación de exposición a EDCs tiroideos (PCB, bisfenol A, PFAS) como modulador externo mejora significativamente el ajuste del modelo?
3. ¿Existe un efecto sinérgico entre múltiples contadores de daño y la exposición a EDCs?

---

## 2. Hipótesis operativas (falsables)

**H1 (nula):** MCAOA completo (5 contadores) no predice significativamente mejor la mortalidad/trayectorias de fragilidad que un modelo de regresión que use solo edad, sexo y un solo biomarcador (longitud telomérica o edad epigenética).

**H2 (nula):** La inclusión de exposición a EDCs tiroideos como modulador de los contadores no reduce significativamente la desviación del modelo respecto a los datos observados (test de razón de verosimilitud, α = 0.05, corrección FDR).

---

## 3. Plan de análisis: validación contra NHANES

### 3.1. Fuente de datos

**NHANES (National Health and Nutrition Examination Survey)** — ciclos 1999–2018 con vinculación a mortalidad (NDI). Ventajas:
- Datos de exposición a EDCs (PCB, bisfenol A, PFAS) en suero
- Datos de función tiroidea (TSH, T4L, T3L)
- Datos longitudinales de mortalidad (NDI, hasta 2022)
- Datos de enfermedades crónicas, fragilidad (self-reported), biomarcadores
- Muestra representativa de EE.UU. (tamaño: ~50,000–80,000 según ciclo)

### 3.2. Variables

| Tipo | Variables | Fuente NHANES |
|------|-----------|---------------|
| Exposición | PCB (congéneres individuales), BPA, PFAS (PFOA, PFOS) | Laboratorio (suero) |
| Función tiroidea | TSH, T4L, T3L, T3 total | Laboratorio |
| Envejecimiento | Mortalidad (tiempo hasta muerte), índice de fragilidad (Rockwood 2005), enfermedades crónicas | NDI vinculado + cuestionario |
| Covariables | Edad, sexo, IMC, tabaco, alcohol, nivel socioeconómico | Cuestionario |

### 3.3. Modelos comparados

| Modelo | Descripción | Parámetros libres |
|--------|-------------|:-----------------:|
| **M0** | Nulo: solo edad + sexo | 2 |
| **M1** | Regresión logística: edad + sexo + un biomarcador (telómero o epigenético) | 3 |
| **M2** | Regresión logística: edad + sexo + exposiciones a EDCs | 3–5 |
| **M3** | MCAOA basal (5 contadores, parámetros a priori) + edad + sexo | 7 |
| **M4** | MCAOA con modulación EDC + edad + sexo | 7 + k_c × N_contadores |

### 3.4. Criterios de comparación

| Criterio | Método |
|----------|--------|
| Bondad de ajuste | AIC, BIC |
| Discriminación | AUC-ROC, C-statistic |
| Calibración | Test de Hosmer-Lemeshow, diagramas de calibración |
| Significancia incremental | Test de razón de verosimilitud (modelos anidados) |
| Validación cruzada | 10-fold cross-validation |
| Corrección por múltiples comparaciones | FDR (Benjamini-Hochberg, q < 0.05) |

### 3.5. Criterio de éxito pre-definido

MCAOA se considera útil si **M4 (MCAOA + EDC)** supera a **M1** y **M2** en AUC-ROC con una diferencia ≥ 0.05 y un test de razón de verosimilitud significativo (p < 0.05 con corrección FDR).

Si ningún modelo MCAOA supera a modelos más simples, se concluirá que **el marco multi-contador no aporta valor predictivo adicional** para los datos analizados, lo cual es un resultado igualmente válido desde el punto de vista científico.

---

## 4. Interpretación de resultados (escenarios posibles)

| Escenario | Interpretación | Implicación para la línea |
|-----------|---------------|---------------------------|
| M4 mejor que M1, M2, M3 | MCAOA con EDCs mejora la predicción — la hipótesis multi-contador+ambiental es plausible | Desarrollar como NAM computacional |
| M3 mejor que M1, M2, pero M4 no | MCAOA basal es útil, pero la modulación por EDCs no mejora — la vía tiroidea no es el mecanismo dominante | Refinar el módulo EDC o buscar otros moduladores |
| M4 similar a M2 | La exposición a EDCs explica la variabilidad, los contadores no añaden información | Marco teórico no refutado pero no superior — publicar como hallazgo negativo |
| Ningún modelo mejora M0 | Ni MCAOA ni EDCs predicen envejecimiento patológico — las variables no son las adecuadas | Repetir con otras cohortes o biomarcadores |

---

## 5. Viabilidad a distancia y tiempo parcial

| Actividad | Modalidad | Dedicación estimada |
|-----------|-----------|:-------------------:|
| Análisis de datos NHANES (extracción, limpieza, modelado) | Remota (R/Python) | 6 meses (15 h/sem) |
| Desarrollo de interfaz MCAOA-NHANES (calibración) | Remota (Rust/R) | 3 meses (15 h/sem) |
| Redacción de artículos (3) y tesis | Remota | 6 meses (15 h/sem) |
| Videoconferencias quincenales con directores | Remota | 1 h/sem |
| Estancia presencial en Madrid (si necesaria) | Presencial | 2 semanas × 1–2 veces |

**Plan temporal total:** 3–4 años.

---

## 6. Publicaciones previstas (compendio)

1. **Artículo metodológico:** «MCAOA-EDC: a multi-counter computational model for evaluating cumulative EDC risk on tissue aging» → target: *Toxicology in Vitro* o *ALTEX* (factor de impacto ~3–5).
2. **Artículo de validación:** «Comparative evaluation of MCAOA vs. single-counter models for predicting mortality in NHANES (1999–2018): the role of thyroid-disrupting EDCs» → target: *Environmental Research* o *Environmental Health Perspectives* (FI ~6–8).
3. **Artículo de revisión:** «Thyroid-disrupting chemicals and tissue aging: a quantitative framework using MCAOA» → target: *Endocrine Reviews* o *Molecular and Cellular Endocrinology* (FI ~8–15).

---

## 7. Referencias clave

1. Tqemaladze J. (2023) Centriolar Damage Accumulation Theory of Aging. *Mol Biol Rep*. PMID: 36583780.
2. Tqemaladze J. (2026) Multi-Counter Architecture of Organismal Aging. eLife-RP-RA-2026-111885. DOI: 10.5281/zenodo.20055806.
3. Gore AC et al. (2015) EDC-2: Endocrine Society scientific statement. *Endocr Rev*. PMID: 25522325.
4. Rockwood K, Mitnitski A. (2005) Frailty index as a measure of biological age. *J Gerontol A Biol Sci Med Sci*.
5. Searle SD et al. (2008) A standard procedure for creating a frailty index. *BMC Geriatrics*.
6. López-Otín C et al. (2023) Hallmarks of aging (3rd edition). *Cell*. PMID: 36893778.
7. Kortenkamp A. (2014) Low dose mixture effects of endocrine disrupters. *Environ Int*. PMID: 25454207.
8. National Health and Nutrition Examination Survey (NHANES). CDC/NCHS. https://www.cdc.gov/nchs/nhanes/

---

**Candidato:** Dr. Jaba Tqemaladze, MD
**Fecha:** 2026-05-26
