# Respuesta a Dra. Mónica Torres Ruiz — 2026-05-26

**De:** Dr. Jaba Tqemaladze, MD <djabbat@gmail.com>
**Para:** Mónica Lucía Torres Ruiz <mtorres@isciii.es>
**CC:** Antonio De la Vieja <adelavieja@isciii.es>, PALOMA COLLADO GUIRAO <pcollado@psi.uned.es>
**Asunto:** RE: Solicitud de dirección de tesis — Programa 9620 — Equipo 8 Línea 3 — Documentación solicitada

---

Estimada Dra. Torres Ruiz,

Muchísimas gracias por su respuesta del 22 de mayo y por la valoración tan cuidadosa de mi propuesta. Agradezco especialmente que haya considerado la posible codirección del Dr. Antonio De la Vieja, así como la oportunidad de reformular los aspectos que usted señala.

A continuación respondo punto por punto a su solicitud de información adicional.

---

## 1. Contacto con la UNED — viabilidad administrativa

Ya he contactado con la UNED. Mi solicitud de preinscripción (Nº **712356513**) fue presentada el 19 de mayo de 2026, dentro del plazo ordinario del Programa de Doctorado en Ciencias Biomédicas y Salud Pública (código 9620). Los documentos requeridos (pasaporte, CV, título de médico, certificado académico) han sido cargados en el portal. Mi titulación de MD (Tbilisi State Medical University, 1996) está reconocida como equivalente EHEA/Bolonia (6 años, ≥300 ECTS) desde 2005.

Estoy a la espera de la resolución administrativa sobre la modalidad a distancia/tiempo parcial. En caso de que surgiera cualquier obstáculo administrativo para la inscripción, les informaré de inmediato.

Número de solicitud: **712356513** (disponible para verificación si lo precisan).

---

## 2. Estado exacto del manuscrito MCOA/MCAOA

Aclaro el estado actual con toda transparencia:

**a) Manuscrito:** «The Multi-Counter Architecture of Organismal Aging»
- **Estado:** eLife Reviewed Preprint (ID: **eLife-RP-RA-2026-111885**), depositado en Zenodo con DOI: **10.5281/zenodo.20055806**.
- **Trayectoria editorial:**
  1. Enviado a *Nature Aging* en abril 2026 → desk reject.
  2. Transferido a **eLife** (abril 2026). La editora sénior (Yamini Dalal) invitó a transferir a **Review Commons** — procedimiento estándar de eLife para evaluar si el manuscrito es adecuado para revisión por pares.
  3. **Review Commons (mayo 2026, #RC-2026-03569)** declinó proceder, indicando que el manuscrito «does not align sufficiently well with the type of research studies that Review Commons is designed to evaluate» — específicamente, porque se trata de un **trabajo teórico**, no experimental. Review Commons aclaró expresamente que esta decisión «is independent of the journals affiliated to Review Commons and is not communicated to any journal».
  4. **Apelación presentada a eLife (22 de mayo de 2026)** , solicitando que el manuscrito sea revisado directamente por un Reviewing Editor de eLife, dado que el filtro de Review Commons es inapropiado para marcos teóricos formales. Estamos pendientes de respuesta.

**b) El manuscrito no ha recibido revisión por pares en ninguna revista hasta la fecha.** La decisión de Review Commons fue exclusivamente por género (teoría vs. experimento), no por mérito científico.

**c) Enlace directo al preprint:** https://doi.org/10.5281/zenodo.20055806

---

## 3. Evaluación de 86.5/100 — procedimiento

La evaluación de 86.5/100 corresponde a un **procedimiento interno de revisión** denominado **TBPR (Tqemaladze Biomedical Peer Review)**, desarrollado para la preparación y verificación de manuscritos antes de su envío a revistas. No es una revisión editorial ni institucional pública.

**Descripción del procedimiento:**

- **Formato:** tres rondas independientes de revisión anónima por pares (22 de mayo de 2026), simulando un comité de tesis doctoral.
- **Comité revisor:** grupo de tres revisores anónimos con experiencia en biología del envejecimiento, modelos computacionales y estadística.
- **Criterios evaluados:** novedad teórica, rigor metodológico, base empírica, implementación computacional, honestidad en citación.
- **Resultado:** 86.5/100 global, con recomendación de aprobación para defensa de tesis doctoral. Las cinco correcciones obligatorias identificadas fueron implementadas en un solo ciclo de revisión (inclusión de datos refutadores para el contador de piRNA, calibración de L_tissue contra el Frailty Index, compromiso de pre-registro, etc.).

**No se trata de una revisión pública** ni de un artículo aceptado. Es un proceso interno de aseguramiento de calidad que forma parte de mi preparación doctoral. Comprendo que esto no sustituye una revisión por pares externa, y lo menciono con total transparencia.

---

## 4. Documento técnico: módulo EDC del simulador MCAOA

Adjunto a este correo el documento **`EDC_module_description.md`** con la descripción técnica completa del módulo de disruptores endocrinos. A continuación un resumen:

**Fundamento mecanístico:** Los disruptores endocrinos con acción tiroidea (PCB, bisfenoles, PFAS, ftalatos) interfieren con la señalización de hormonas tiroideas (T3/T4), que regulan:
- La tasa metabólica basal y la producción mitocondrial de ROS → **contador mitocondrial (#3)**.
- La homeostasis de proteínas y el plegamiento en el retículo endoplásmico → **contador de proteostasis (#5)**.
- La metilación del ADN y las modificaciones de histonas → **contador epigenético (#4)**, efecto menor.

**Formalización matemática:** La exposición a EDCs se modela como un **multiplicador adimensional** `m(c, E)` del incremento de daño del contador `c` para un nivel de exposición `E ∈ [0,1]`:

```
m(c, E) = 1 + k_c · E
```

donde `k_c` es el coeficiente de sensibilidad específico del contador:

| Contador | k_c | Fundamento bibliográfico |
|----------|:---:|--------------------------|
| Mitocondrial (#3) | 0.8 | T3 regula PGC-1α y biogénesis mitocondrial; EDCs tiroideos aumentan ROS (Weiss et al. 2023, *Environ Health Perspect*) |
| Proteostasis (#5) | 0.5 | Estrés del RE inducido por EDCs (Ferguson et al. 2023, *Toxicol Sci*) |
| Epigenético (#4) | 0.2 | Alteraciones de metilación por PCB (Curtis et al. 2024, *Epigenetics*) |
| Centriolar (#1) | 0.0 | Sin evidencia directa |
| Telomérico (#2) | 0.0 | Sin evidencia directa |

**Parámetros configurables:** Todos los coeficientes `k_c`, así como el nivel de exposición `E`, son parámetros configurables en el código abierto, no valores fijos.

**Calibración:** Actualmente los parámetros son *a priori* (basados en la literatura). La tesis propone calibrarlos contra datos poblacionales (NHANES, ENRICA-Seniors).

---

## 5. Ejemplo mínimo reproducible

Adjunto el anexo **`MCAOA_minimal_example.md`** con instrucciones paso a paso para:

1. Clonar el repositorio: `git clone -b mcaoa-v3.2 https://github.com/djabbat/LC.git`
2. Compilar: `cd LC/MCAOA && cargo build --release`
3. Ejecutar el test EDC: `cargo test --package mcoa_simulation edc_thyroid -- --nocapture`
4. O bien ejecutar la simulación completa: `cargo run --package mcoa_cli --example run_edc`

**Licencia:** MIT — el código es completamente abierto, sin restricciones de uso, modificación o redistribución. Solo se solicita la cita académica (DOI del preprint).

---

## 6. Propuesta reformulada: MCAOA como hipótesis computacional a evaluar críticamente

Entiendo perfectamente su planteamiento: la tesis no debe asumir que MCAOA está validado, sino someterlo a **evaluación crítica objetiva** comparándolo con modelos alternativos sobre datos reales. Adjunto la propuesta revisada **`Propuesta_v2_MCOA_Hipotesis.md`** con este enfoque.

**Cambios fundamentales respecto a la versión anterior:**

1. **MCAOA se formula como hipótesis computacional**, no como modelo validado. Las preguntas de investigación son:
   - ¿Puede un modelo multi-contador mejorar la predicción de trayectorias de envejecimiento patológico frente a modelos de un solo contador (telómero, epigenético)?
   - ¿La incorporación de exposición a EDCs tiroideos como modulador externo mejora significativamente el ajuste del modelo?

2. **Validación contra NHANES:** se propone un plan concreto de análisis de datos de NHANES (ciclos 1999–2018) con:
   - Exposición a PCB, bisfenol A, PFAS (variables séricas)
   - Biomarcadores de función tiroidea (TSH, T4L, T3L)
   - Variables de envejecimiento (mortalidad, fragilidad, enfermedades crónicas)
   - Comparación de modelos: MCAOA completo vs. modelo nulo (sin contadores) vs. modelo de un solo contador (telómero) vs. regresión logística estándar

3. **Criterio de éxito pre-definido:** mejora significativa en AIC/BIC y AUC-ROC del modelo MCAOA frente a alternativas más simples (ANOVA de modelos anidados, α = 0.05, con corrección FDR).

---

Quedo a su entera disposición para cualquier aclaración adicional. Si lo considera oportuno, puedo organizar una videoconferencia en cuanto la UNED confirme la viabilidad administrativa y ustedes hayan revisado los documentos adjuntos.

Le agradezco de nuevo su tiempo, su franqueza y la oportunidad de reformular la propuesta.

Atentamente,

Dr. Jaba Tqemaladze, MD
Presidente, Georgia Longevity Alliance
ORCID: 0000-0001-8651-7243
Tel. / WhatsApp: +995 555 185 161
Email: djabbat@gmail.com

---

**Documentos adjuntos:**
1. `EDC_module_description.md` — descripción técnica del módulo EDC (ecuaciones, parámetros, fuentes)
2. `MCAOA_minimal_example.md` — instrucciones para clonar, compilar y ejecutar el simulador
3. `Propuesta_v2_MCOA_Hipotesis.md` — propuesta de tesis reformulada: MCAOA como hipótesis a evaluar críticamente
