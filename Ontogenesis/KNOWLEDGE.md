# KNOWLEDGE.md — Ontogenesis
## Domain Corpus: Human Ontogenesis 0–25 Years

---

## 1. Developmental Phases Overview

Human ontogenesis 0–25 years encompasses five major phases:

| Phase | Age range | Key characteristics |
|-------|-----------|---------------------|
| Infancy | 0–2 yr | Rapid growth, brain volume doubles, synaptic proliferation |
| Early childhood | 2–6 yr | Motor refinement, language acquisition, cognitive scaffolding |
| Middle childhood | 6–12 yr | Steady growth, cognitive consolidation, social network expansion |
| Puberty / Adolescence | 10–18 yr | HPG axis activation, rapid somatic growth, sex differentiation |
| Late adolescence / Early adulthood | 18–25 yr | Prefrontal maturation, skeletal consolidation, social independence |

The upper boundary of 25 years is defined by:
- Prefrontal cortex myelination completion (~24–25 yr)
- Epiphyseal plate fusion (skeletal maturity)
- HPA axis reaching adult set-point
- This is also the **lower boundary for CDATA simulation** in the AIM ecosystem

---

## 2. Anatomical Domain Parameters

### 2.1 Height and Weight (Parameters 1–2)
- Source: WHO Growth Charts (CC BY-NC-SA); percentiles 3/10/25/50/75/90/97
- Rapid growth velocities: infancy (0–2 yr) and pubertal growth spurt (F: 10–14 yr; M: 12–16 yr)
- Adult height reached: females ~16 yr, males ~18 yr

### 2.2 Body Proportions — Sitting Height/Stature (Parameter 3)
- Newborn ratio ~0.68; adult ratio ~0.52
- Reflects differential growth of trunk vs. limbs across development

### 2.3 Organ Volumes (Parameters 4–8): Brain, Heart, Liver, Kidneys, Lungs

| Organ | Birth volume | Adult volume | Peak growth period |
|-------|-------------|-------------|-------------------|
| Brain | ~350 mL | ~1350 mL | 0–5 yr (80% adult by age 3) |
| Heart | ~20 mL | ~200 mL | Linear + pubertal spurt |
| Liver | ~150 mL | ~1500 mL | Proportional to body mass |
| Kidneys (pair) | ~30 mL | ~300 mL | Proportional to body mass |
| Lungs (pair) | ~80 mL | ~1200 mL | Pubertal spurt more prominent in males |

Data sources: MRI volumetrics from published meta-analyses; NHANES body composition data.

---

## 3. Endocrinological Domain Parameters

### 3.1 Growth Hormone / IGF-1 (Parameters 9–10)
- GH secretion is pulsatile; peak amplitude increases through puberty
- IGF-1 peaks at mid-puberty (Tanner III–IV): females ~12–13 yr, males ~13–15 yr
- IGF-1 reference values approximate: prepubertal 100–300 ng/mL; pubertal peak 350–600 ng/mL; adult 100–300 ng/mL
- Key axis: hypothalamus (GHRH/somatostatin) → pituitary → liver (IGF-1) → bone/muscle

### 3.2 Sex Steroids (Parameter 11)
- **Testosterone (males):** prepubertal <10 ng/dL; adult 300–1000 ng/dL; pubertal rise starts ~11–12 yr (Tanner II)
- **Estradiol (females):** prepubertal <10 pg/mL; adult follicular 30–120 pg/mL; rise starts ~9–10 yr (Tanner II)
- Tanner stages (puberty staging standard, Marshall & Tanner 1969):
  - Tanner I = prepubertal; Tanner V = adult
  - Each stage ~1–2 years duration; entry age varies ±2 SD

### 3.3 Cortisol (Parameter 12)
- Diurnal rhythm: peaks at 06:00–08:00, nadir at midnight
- Morning cortisol reference: 5–25 µg/dL (adult); similar in children after ~3 yr
- HPA axis matures by ~3 years; stress reactivity peaks in adolescence
- Chronic elevation signals allostatic load — tracked as quality marker (Parameter 15)

---

## 4. Psychological Domain Parameters

### 4.1 CBCL T-score (Parameter 13)
- Child Behavior Checklist (ASEBA): normed for ages 1.5–18 yr
- T-score 50 = population mean; T ≥ 70 = clinically significant (98th percentile)
- Two broad-band scales: Internalizing (anxiety, withdrawal) and Externalizing (aggression, rule-breaking)
- Developmental trajectory: externalizing peaks in toddlerhood, declines; internalizing increases in adolescence

### 4.2 Temperament — EAS (Parameter 14)
- EAS (Emotionality, Activity, Sociability) — Buss & Plomin model
- Relatively stable trait from infancy; biological basis in limbic-prefrontal circuits
- Emotionality inversely correlated with social network size in adolescence

### 4.3 Cortisol as Stress Marker (Parameter 15)
- Derived from Parameter 12 (cortisol); categorical scale: Normal / Elevated / High
- >2 SD above age norm = Elevated; >3 SD = High
- Integration point for AIM treatment_recommender.py (stress-related dosage adjustments)

---

## 5. Cognitive Domain Parameters

### 5.1 IQ Indices — WISC-V / WAIS-IV (Parameters 16–19)
- Verbal IQ (VCI): language comprehension, reasoning; peaks ~20–25 yr
- Spatial IQ (VSI): visual-spatial processing; peaks ~18–22 yr, then stable
- Working Memory (WMI): rapid development 6–16 yr; adult capacity ~7±2 items
- Processing Speed (PSI): fastest growth 6–12 yr; plateaus 18–22 yr

Developmental trajectories:
| Index | Rapid growth | Peak | Notes |
|-------|-------------|------|-------|
| Verbal IQ | 4–16 yr | ~22 yr | Crystallized intelligence; continues growing |
| Spatial IQ | 4–14 yr | ~20 yr | Fluid intelligence; peaks earlier |
| Working Memory | 6–16 yr | ~20 yr | Frontal lobe dependent |
| Processing Speed | 6–12 yr | ~18 yr | Peaks earliest; myelination-dependent |

---

## 6. Social Domain Parameters

### 6.1 Life Events (Parameters 20–23)
Binary markers (0/1):
- **Kindergarten entry:** 3–5 yr (country-dependent); first major peer social environment
- **School entry:** 5–7 yr; structuring of daily time, authority relationships
- **Puberty (Tanner II):** 9–11 yr (F), 11–13 yr (M); social role shift, peer group reorganization
- **School / University graduation:** 17–22 yr; transition to autonomy

### 6.2 Social Network Size (Parameter 24)
- Infancy: family only (~5 people)
- Kindergarten age: ~10–15 peers
- Middle childhood: ~20–30 stable relationships (Dunbar inner circle)
- Adolescence: peak of acquaintances (~100–150); stable close network ~15
- Data source: GSS/ESS adapted questionnaires

---

## 7. Transition Detection Algorithm Summary

### Longitudinal Data
- Anatomical: transition if change > **2 SD** from individual rolling trajectory
- Endocrine: transition if change > **3 SD** from individual rolling trajectory

### Cross-Sectional Data
- Anatomical: transition if `CV(A,t) > mean_CV + 2×SD_CV` in age cohort
- Endocrine: transition if `Range_90_10(t) > mean(Range_90_10) + 2×SD(Range_90_10)`

### Clustering
- Radius: 6 months
- Min stable period: 3 months
- Min transitions per cluster: 30
- Only clusters are visualized as critical periods on the timeline

---

## 8. Regeneration Module — Developmental Context

Regeneration efficiency is age-modulated relative to the 25-yr baseline:

| Age group | Speed multiplier | Biological rationale |
|-----------|-----------------|---------------------|
| 0–12 yr | ×1.2 | Higher stem cell reserves, stronger growth factor milieu |
| 12–25 yr | ×1.0 | Baseline adult norm |
| 25+ yr | ×0.8 | CDATA domain: centriolar damage accumulation begins |

**Boundary with CDATA:** The 25+ coefficient feeds directly into AIM/treatment_recommender.py for age-adjusted protocols.

Tissue types:
1. **Superficial** (skin, epithelium): 7–14 days, 100% recovery
2. **Deep** (muscle, bone): 4–8 weeks, 70–90% recovery (possible residual fibrosis)
3. **Parenchymal** (liver): 4–6 months, 100%* (if no cirrhosis)
4. **Irreversible** (cardiomyocytes, CNS neurons): 0% recovery → fibrotic replacement

---

## 9. AIM Ecosystem Integration Points

| AIM module | Data received from Ontogenesis | Parameter(s) |
|-----------|-------------------------------|-------------|
| `lab_reference.py` | Hormone age norms (0–25 yr) | GH, IGF-1, testosterone/estradiol, cortisol |
| `treatment_recommender.py` | Regeneration age multipliers | ×1.2 (0–12), ×1.0 (12–25) |
| CDATA (Cell-DT) | Lower boundary = 25 yr | All parameters hand-off at 25 yr |

---

## 10. Key Validated Facts with Sources

| Fact | Source |
|------|--------|
| Brain reaches 80% adult volume by age 3 | Knickmeyer et al. 2008, PMID 18971494 |
| Prefrontal cortex myelination completes ~24–25 yr | Lebel et al. 2012, PMID 22306083 |
| IGF-1 peak at Tanner III–IV | Juul et al. 1997, PMID 9262477 |
| Tanner staging standard | Marshall & Tanner 1969, PMID 5785179 |
| Working memory capacity ~7±2 | Miller 1956 (classic); developmental norms WISC-V |
| NK cell efficiency at age 70 = 50% | PMID 12803352 (relevant for regen boundary) |
| CHIP onset acceleration >25 yr | Jaiswal 2017, PMID 28792876 |
