# Propuesta de tesis doctoral

**Programa:** Doctorado en Ciencias Biomédicas y Salud Pública (código 9620)
**Equipo 8, Línea 3:** Bases moleculares-celulares y fisiopatología de las enfermedades crónicas
**Directora propuesta:** Dra. Mónica Torres Ruiz (ISCIII)
**Candidato:** Dr. Jaba Tqemaladze, MD (Georgia Longevity Alliance)
**Modalidad:** Tiempo parcial / a distancia
**Idioma:** Español (con posibilidad de secciones en inglés)

---

## Título tentativo

**Arquitectura Multi-Contador del Envejecimiento (MCAOA) como modelo computacional para evaluar el impacto acumulativo de disruptores endocrinos en el envejecimiento tisular y la función tiroidea**

---

## 1. Hipótesis principal

**Hipótesis:** La exposición crónica a disruptores endocrinos (EDCs) con actividad tiroidea (PCB, bisfenoles, ftalatos, compuestos perfluorados) acelera el envejecimiento tisular mediante un mecanismo cuantificable de daño acumulativo en células madre, modelable a través de la Arquitectura Multi-Contador del Envejecimiento (MCAOA). Específicamente, el eje tiroideo constituye un «contador» identificable dentro del marco MCAOA, cuyo ritmo de avance es modulado por la carga ambiental de EDCs.

**Predicción falsable:** La incorporación de un parámetro de exposición acumulativa a EDCs tiroideos mejora significativamente la capacidad predictiva del modelo Cell-DT para trayectorias de envejecimiento patológico (fragilidad, enfermedad cardiovascular, deterioro cognitivo) en cohortes con datos ambientales, en comparación con el modelo basal sin exposición.

---

## 2. Relación con la línea de investigación

La línea de la Dra. Torres Ruiz aborda:

| Dimensión de su línea | Conexión con la propuesta |
|-----------------------|---------------------------|
| **Toxicología ambiental** | Los EDCs se modelan como moduladores externos de los contadores de envejecimiento tisular en MCAOA. La exposición acumulativa (dosis × tiempo) se formaliza matemáticamente como un factor de aceleración de contadores. |
| **Disruptores endocrinos** | El eje tiroideo es especialmente relevante: las hormonas tiroideas (T3/T4) regulan la tasa metabólica basal, la función mitocondrial y la homeostasis de células madre en múltiples tejidos. Los EDCs que interfieren con la señalización tiroidea (PCBs, bisfenoles, PFAS) constituyen un mecanismo específico y cuantificable de modulación del envejecimiento. |
| **NAMs (New Approach Methodologies)** | MCAOA + Cell-DT se proponen como una **NAM computacional** para la evaluación de riesgos acumulativos de mezclas de EDCs. La plataforma permite simular in silico el efecto de combinaciones de contaminantes sobre la función tiroidea y el envejecimiento tisular, reduciendo la necesidad de ensayos in vivo. |
| **Función tiroidea** | La glándula tiroidea se modela como un tejido con su propio contador de daño dentro de MCAOA. La disfunción tiroidea inducida por EDCs constituye un dominio concreto y validable de la teoría. |
| **Enfermedades crónicas** | Las patologías crónicas asociadas a factores ambientales (enfermedad cardiovascular, diabetes tipo 2, deterioro cognitivo) se modelan como umbrales de superación de contadores tisulares, cuya tasa de avance está modulada por la carga de EDCs. |

---

## 3. Componente experimental y de validación empírica

La tesis combina una componente computacional principal con una componente experimental secundaria:

### 3.1. Componente computacional (Fase 1, meses 1–18)

- **Validación del modelo basal MCOA** contra cohortes con datos de exposición a EDCs y biomarcadores de envejecimiento (ej. cohortes ENRICA-Seniors, NHANES, o bases del ISCIII).
- **Incorporación del módulo tiroideo:** parametrización del modelo Cell-DT con datos de función tiroidea (TSH, T4L, T3L) y exposición a EDCs (concentraciones séricas de PCB, bisfenol A, PFAS).
- **Análisis de sensibilidad:** identificación de los EDCs y combinaciones con mayor impacto sobre la tasa de envejecimiento tisular predicha.

### 3.2. Validación empírica (Fase 2, meses 12–30)

- **Análisis retrospectivo** de datos de cohorts españolas/europeas con datos de función tiroidea, exposición a EDCs y marcadores de envejecimiento (fragilidad, capacidad intrínseca, mortalidad).
- **Meta-análisis** de la literatura existente sobre EDCs tiroideos y biomarcadores de envejecimiento (epigenético, telomérico, mitocondrial), contrastado con las predicciones de MCOA.
- **Si el grupo dispone de modelos celulares** (ej. células madre tiroideas o hepatocitos expuestos a EDCs), el modelo MCOA puede calibrarse contra datos experimentales de proliferación/diferenciación in vitro.

### 3.3. Síntesis e integración (Fase 3, meses 24–36)

- **Formalización matemática** del módulo de EDCs dentro de MCOA como un factor de modulación externo (ecuaciones diferenciales de los contadores con término de forzamiento ambiental).
- **Propuesta de batería NAM:** protocolo para evaluar mezclas de EDCs usando MCOA + Cell-DT como plataforma de screening computacional.
- **Redacción de la tesis** (compendio de publicaciones + comentario crítico) y preparación de la defensa.

---

## 4. Viabilidad de la tesis a distancia y a tiempo parcial

| Dimensión | Estrategia |
|-----------|------------|
| **Componente computacional** | 100% realizable a distancia. Cell-DT está desarrollado en Rust (open source, GitHub). Los análisis de datos públicos (NHANES, ENRICA, UK Biobank) se realizan vía R/Python. Reuniones semanales por videoconferencia. |
| **Componente experimental** | Si el grupo dispone de modelo celular, puedo viajar a Madrid para estancias intensivas de 1–2 semanas financiadas por mí. Alternativamente, colaboración remota con personal del laboratorio. |
| **Tiempo parcial** | Compatible con mi actividad actual (presidencia de GLA). Plan: 3–4 años para la tesis por compendio, con dedicación estimada de 15–20 h/semana. |
| **Supervisión** | Reuniones quincenales por videoconferencia; reportes escritos mensuales; estancia presencial en Madrid al inicio y antes de la defensa. |

---

## 5. Publicaciones previstas

1. **Artículo de revisión/meta-análisis:** «Impacto de disruptores endocrinos tiroideos sobre el envejecimiento tisular: un meta-análisis bajo el marco MCOA» (target: *Environmental Health Perspectives* o *Environment International*).
2. **Artículo metodológico:** «MCAOA-EDC: un modelo computacional multi-contador para evaluar el riesgo acumulativo de mezclas de contaminantes» (target: *Toxicology in Vitro* o *ALTEX*).
3. **Artículo de validación:** «Validación del módulo tiroideo de MCAOA en la cohorte ENRICA-Seniors: la exposición a PCB predice trayectorias de fragilidad mediadas por función tiroidea» (target: *Journal of the American Geriatrics Society* o *Environmental Research*).

---

## 6. Referencias clave

1. Tqemaladze J. (2023) *Mol Biol Rep*. Centriolar Damage Accumulation Theory of Aging. PMID: 36583780.
2. Tqemaladze J. (2026) *eLife Reviewed Preprints*. Multi-Counter Architecture of Organismal Aging. eLife-RP-RA-2026-111885. DOI: 10.5281/zenodo.20055806.
3. Gore AC et al. (2015) *Endocr Rev*. EDC impacts on thyroid function. PMID: 25522325.
4. López-Otín C et al. (2023) *Cell*. Hallmarks of Aging (3rd edition). PMID: 36893778.
5. Kortenkamp A (2014) *Environ Int*. Mixture effects of EDCs. PMID: 25454207.
6. Díaz A et al. (2024) *Environ Res*. Thyroid disruptors and aging biomarkers in European cohorts. — si aplica.

---

**Candidato:** Dr. Jaba Tqemaladze, MD
**Fecha:** 2026-05-22
