# Cover Letter — Resubmission of BIOSYS-D-26-00760

**Date:** 2026-06-02
**To:** Dr. Robert Prinz, Editor, BioSystems
**Re:** Resubmission of previously rejected manuscript BIOSYS-D-26-00760
**Title:** Beyond Calendar Time: A Multilayer Age Model for Physical Swarm Hierarchy
**Author:** Jaba Tqemaladze, MD

---

Dear Dr. Prinz,

Thank you for your detailed and constructive review of our manuscript BIOSYS-D-26-00760. We have carefully addressed each of the five concerns raised by the reviewer. Below is a point-by-point response.

---

## Response to Reviewer Comments

### 1. Insufficient biological depth

**Reviewer:** "The manuscript lacks sufficiently deep explanations related to biology and molecular systems known to be involved in aging."

**Response:** We have substantially expanded the biological motivation section (now §1.3, ~4 pages). The new section includes:
- Molecular mechanisms of tissue-specific aging: epigenetic clocks (Horvath multi-tissue clock, GrimAge, PhenoAge), cellular senescence (Campisi, Muñoz-Espín), mitochondrial dysfunction, autophagy decline, and inflammaging
- The τ_sick–disease connection, including the thymus paradox, HSC somatic mutation burden, and CDATA centriolar damage connection
- Intra-individual epigenetic age variance (IEAV) as the direct biological analogue of τ_sick, with supporting evidence (Horvath & Raj 2018: IEAV >5 years → 2.3× mortality risk)
- A new table (Table 1) explicitly mapping each bristlebot age layer to its biological analogue and molecular mechanism
- 34 new references added (18→52 total), including López-Otín 2023, Horvath 2013, Lu 2019 (GrimAge), Levine 2018 (PhenoAge), Campisi 2019, Franceschi 2000, and others

### 2. Unclear data sources

**Reviewer:** "It is unclear where the values for the brain and other organs were derived from or what they are based upon."

**Response:** Every parameter now has an explicit, verifiable data source:
- **V_cap:** Panasonic EECF5R5U105 supercapacitor datasheet + bench measurements (n=30, R²=0.994)
- **LED brightness:** Kingbright L-934GD datasheet + Thorlabs photodiode measurements
- **ESP32 aging:** Espressif AN-2022-01 reliability report + Black's equation for electromigration
- **Motor aging:** Mabuchi RC-280 datasheet + brush wear model
- **Supercapacitor ESR:** Empirically fitted degradation model from 10 prototype bots over 8 weeks
- **τ_sick:** Formal entropy definition with computation example
- **τ_atom:** Stellar nucleosynthesis models (Woosley 2002, Karakas 2016) + MC-ICP-MS protocol for δ³⁰Si
- **Resource costs:** Supplier names and catalog numbers (Mouser, Digikey) with current pricing

### 3. Role of experimental protocols

**Reviewer:** "The manuscript does not clarify what role the experimental protocols play in the study or how they contribute to the overall conclusions. They are too short anyhow and lack necessary explanation."

**Response:** The experimental protocol section (§5) has been expanded from 1 page to 4 pages:
- §5.1: Robot design and component tracking (YAML τ_profile format for each bot)
- §5.2: Telemetry protocol (10-byte ESP-NOW packet specification)
- §5.3: Five-point calibration protocol (V_cap, LED, motor, buzzer, CRC)
- §5.4: Complete test sequence (4 phases, 75–85 runs, 2.5–3 hours)
- §5.5: Safety risk matrix (5 risks × mitigation × verification)
- §5.6: Resource requirements table with 12 line items and total cost (€1,580)
- All protocols are directly linked to specific test outcomes in the decision matrix (§3.1, Table 5a)

### 4. Reading flow

**Reviewer:** "The manuscript does not maintain a coherent reading flow across sections."

**Response:** We have restructured the manuscript with:
- A roadmap (§1.1) that explicitly states the organization of all sections
- Cross-references between sections (e.g., "see §1.3.3 for limitations" in the Abstract)
- A decision tree (Table 7) that maps experiment progression step by step
- A Decision Matrix (Table 5a) showing which test covers each layer combination
- Formal definitions at the start of each major section

### 5. Limited references

**Reviewer:** "The low number of references does not indicate uniqueness, but rather suggests a lack of embeddedness in contemporary theories and the broader scientific literature."

**Response:** References have been expanded from 18 to 52, including:
- Swarm robotics: Bonabeau 1999, Sahin 2004, Hamann 2018, Floreano 2010
- Aging biology: López-Otín 2013 & 2023, Horvath 2013 & 2018, Gladyshev 2021, Campisi 2019, Franceschi 2000 & 2018
- Epigenetic clocks: Lu 2019 (GrimAge), Levine 2018 (PhenoAge), Benayoun 2015
- Senescence: Muñoz-Espín 2014, Demaria 2017, Zhu 2015, Nikichuk 2022
- Autophagy and inflammaging: Rubinsztein 2011, Salminen 2012, Fulop 2017
- Geroscience: Kennedy 2014, Kaeberlein 2017
- Electronics aging: Bhardwaj 2022, Hamilton 2021, Zhang 2022, Andrady 2020
- Stellar nucleosynthesis: Woosley 2002, Karakas 2016, Seitenzahl 2019, Savage 2017

---

## Summary of Changes

| Metric | Original Submission | Revised Version |
|--------|:------------------:|:---------------:|
| Word count | ~3,500 | ~7,500 |
| References | 18 | 52 |
| Tables | 3 | 9 |
| Biological depth | 1 paragraph | 4 pages |
| Experimental protocol | 1 page | 4 pages |
| Data sources | Implicit | Explicit for all parameters |

We believe the revised manuscript addresses all reviewer concerns and is now suitable for publication in BioSystems. We look forward to your consideration.

Sincerely,

**Jaba Tqemaladze, MD**
Kutaisi International University & Georgia Longevity Alliance
jaba@longevity.ge
ORCID: 0000-0001-8651-7243
