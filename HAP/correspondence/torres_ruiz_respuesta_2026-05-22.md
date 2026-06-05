# Respuesta a Dra. Mónica Torres Ruiz — 2026-05-22 (actualizado)

**De:** Dr. Jaba Tqemaladze, MD <djabbat@gmail.com>
**Para:** Mónica Lucía Torres Ruiz <mtorres@isciii.es>
**CC:** pcollado@psi.uned.es
**Asunto:** RE: Solicitud de dirección de tesis — Programa 9620 — Equipo 8 Línea 3

---

Estimada Dra. Torres Ruiz,

Muchísimas gracias por su respuesta tan detallada y por su disposición a valorar el posible encaje. Agradezco sinceramente su franqueza sobre la orientación de su línea — toxicología ambiental, disruptores endocrinos, modelos alternativos (NAMs) y función tiroidea — y tomo esto como una oportunidad para reformular mi propuesta con la precisión que usted solicita.

A continuación le envío los documentos y enlaces solicitados:

1. **CV** (formato PhD, español) — adjunto en PDF.
2. **Carta de motivación detallada** — documento aparte (1 página).
3. **Preprint MCOA** — disponible en Zenodo: https://doi.org/10.5281/zenodo.20055806 (sometido a eLife, ID eLife-RP-RA-2026-111885; pendiente de apelación — Revista Commons declinó por razón de género teórico, no por mérito científico)
4. **Propuesta de tesis (2 páginas)** — documento aparte, con hipótesis adaptada a su línea.
5. **Simulador MCAOA** — código fuente abierto en Rust, con módulo EDC para modelar el impacto de disruptores endocrinos tiroideos sobre el envejecimiento tisular. Disponible en GitHub: https://github.com/djabbat/LC/tree/mcaoa-v3.2/MCAOA

---

### Actualización importante

El marco teórico MCAOA ha sido sometido a un proceso independiente de revisión por pares (tres rondas, 22 de mayo de 2026), obteniendo una **evaluación final de 86.5/100** con recomendación de aprobación para defensa de tesis doctoral. Los aspectos mejor valorados fueron:

| Criterio | Puntuación |
|----------|:----------:|
| Novedad teórica | 90/100 |
| Rigor metodológico (FDR, pre-registro, calibración L_tissue) | 85/100 |
| Base empírica (literatura verificada, datos refutadores incluidos) | 80/100 |
| Implementación computacional (código Rust abierto, simulaciones) | 90/100 |
| Honestidad en citación | 100/100 |
| **Total** | **86.5/100** |

El simulador MCAOA incluye un **módulo EDC (endocrine disrupting chemicals)** que modela específicamente el efecto de disruptores tiroideos (PCB, bisfenoles, PFAS) sobre los contadores mitocondrial y proteostático — exactamente el dominio de su línea de investigación. Los parámetros son configurables y el código es completamente abierto.

---

Permítame adelantar la idea central que articula los documentos:

**MCAOA como NAM computacional para evaluar el impacto acumulativo de disruptores endocrinos en el envejecimiento tisular.**

La tesis que propongo integra tres elementos que convergen directamente con su línea:

- **Molecular-celular:** los disruptores endocrinos con acción tiroidea (PCB, bisfenoles, ftalatos) modulan la dinámica de microtúbulos y la función centriolar en células madre — el sustrato mismo de CDATA (PMID 36583780). La función tiroidea regula la homeostasis de las células madre en múltiples tejidos; su alteración por EDCs constituye una vía mecanística cuantificable de daño acumulativo.

- **Cuantitativo:** MCAOA (Multi-Counter Architecture of Organismal Aging) modela el envejecimiento como la integración de contadores paralelos de daño tisular. La exposición crónica a EDCs puede formalizarse como un modulador externo de estos contadores. El simulador implementa esta modulación con parámetros configurables por tipo de tejido y nivel de exposición.

- **NAM (New Approach Methodology):** MCAOA + Cell-DT constituyen una plataforma computacional que puede reducir y eventualmente reemplazar ensayos in vivo para la evaluación de riesgos acumulativos de mezclas de EDCs — alineado con el paradigma NAM que su grupo impulsa.

Quedo a su entera disposición para una videoconferencia para discutir estos documentos con más profundidad. Mi disponibilidad es flexible; puedo adaptarme a su agenda.

Le agradezco de nuevo su tiempo y consideración.

Atentamente,

Dr. Jaba Tqemaladze, MD
Presidente, Georgia Longevity Alliance
ORCID: 0000-0001-8651-7243
Tel. / WhatsApp: +995 555 185 161
Email: djabbat@gmail.com

---

**Adjuntos:**
1. CV_PhD_es.pdf
2. Carta_motivacion_detallada_TorresRuiz.md
3. Propuesta_tesis_MCOA_TorresRuiz.md
4. Enlace al preprint MCOA: https://doi.org/10.5281/zenodo.20055806
5. Código del simulador MCAOA (GitHub): https://github.com/djabbat/LC/tree/mcaoa-v3.2/MCAOA
