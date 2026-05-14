<!-- AUTO-TRANSLATED via DeepSeek 2026-05-13. Source language: russian. Original preserved at NEWS.ru.md. -->

# LongevityCommon — News & Science Feed

**Date of Update:** 2026-04-10 
**Status:** CORE · do not publish in public git 
**Purpose:** aggregator of recent scientific news for enriching the entire ecosystem

---

## How to Use This File

- Update at each session (search for recent articles)
- Findings are thematically duplicated in KNOWLEDGE.md of the corresponding subproject
- Format: date · source · thesis · connection to LongevityCommon

---

## April 2026

---

### [GENERAL] Citizen Science and Health in 2026

**Sources:**
- [The Rise of Citizen Science in Health Research — Tandfonline](https://www.tandfonline.com/doi/full/10.1080/15265161.2019.1619859)
- [Citizen Health Science: Foundations of a New Data Science Arena — PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC7299478/)
- [Patient-Powered Digital Health 2026 — DCI Network](https://www.dcinetwork.org/patients2026)
- [Digital Citizen Science Observatory — Frontiers](https://www.frontiersin.org/journals/digital-health/articles/10.3389/fdgth.2024.1399992/full)

**Key Theses:**
- Citizen science is gaining recognition in public health, urban health, and policy-making as a "bottom-up" approach
- Digital Citizen Science Observatory (DiScO): goal — transformation of healthcare systems through ethically collected citizen data → support for decision-making at both policy and patient levels simultaneously
- 08.04.2026: UNC Chapel Hill + UNC Health launched **SHIRE** (Secure Health Informatics Research Environment) — a cloud platform for responsible AI with real-world clinical data
- OpenAI and Anthropic announced the possibility of synchronizing personal health data with ChatGPT/Claude — an indicator of growing demand for a "personal medical assistant"

**Connection to LongevityCommon:**
> LongevityCommon is built on the same concept — the patient as a subject of research, not an object. DiScO and SHIRE are institutional confirmation of the correctness of the vector. Ze·Guide + Lab = implementation of citizen science within the ecosystem.

---

### [GENERAL] Patient Data Rights and Longevity Medicine 2026

**Sources:**
- [2026 — The Longevity Medicine Manifesto — David Luu](https://newsletter.longevitydocs.org/p/2026-the-longevity-medicine-manifesto)
- [Patient Data Ownership — Oxford JLB](https://academic.oup.com/jlb/article/8/2/lsab023/6380070)
- [Ownership of Health Data, Sharing, and Governance — BMC Medical Ethics](https://bmcmedethics.biomedcentral.com/articles/10.1186/s12910-022-00848-y)
- [Recent Digital Health Trends February 2026 — OpenLoop Health](https://openloophealth.com/blog/recent-digital-health-trends-and-news-february-2026)

**Key Theses:**
- In 2026, a network of 500+ physicians in 50+ countries is forming the "Longevity Medicine Manifesto" — 5 elements: clinical standards, real-world outcomes data, governance without commercial influence, infrastructure for medical careers, critical mass of trained specialists
- Debates on "ownership" of health data: neither private nor public model fully solves the problem — the focus shifts to **Data Access Committees** and procedural mechanisms
- Consensus 2026: transparent universal privacy policies are needed, covering collection, storage, transfer, and ownership of raw data

**Connection to LongevityCommon:**
> FCLC + 5-level privacy stack directly addresses this issue. Data remains with the user, aggregated cryptographically — not sold. LongevityCommon Pro = a model where the platform earns from people's health, not from their data.

---

### [FCLC] Federated Learning and Privacy in Healthcare 2026

**Sources:**
- [Securing FL with Blockchain in Medical Field — JMIR 2026](https://www.jmir.org/2026/1/e79052)
- [Federated Learning 2026: Privacy-First AI — Programming Helper](https://www.programming-helper.com/tech/federated-learning-2026-privacy-first-ai-training)
- [Federated Microservices + Blockchain — Nature Scientific Reports 2026](https://www.nature.com/articles/s41598-026-39837-1)
- [Health-FedNet Privacy Framework — ScienceDirect 2026](https://www.sciencedirect.com/science/article/pii/S2590123025025538)
- [FED-EHR Decentralized Analytics — MDPI 2026](https://www.mdpi.com/2079-9292/14/16/3261)
- [Federated Deep Learning for IoT Healthcare — Frontiers 2026](https://www.frontiersin.org/journals/computer-science/articles/10.3389/fcomp.2026.1725597/full)

**Key Theses:**
- Federated microservices: Kubernetes + TensorFlow Federated + Hyperledger Fabric → predictive accuracy 95.2%, API latency −42%, recovery time 10× faster vs monolithic
- Defense-in-depth: FL + Secure Enclaves + Differential Privacy — de facto standard for sensitive medical data
- Secure Aggregation: server sees only the aggregate, never individual updates
- DP + calibrated noise → mathematical guarantee of privacy against gradient leakage
- HIPAA + GDPR compliance is now implemented directly through federated architectures

**→ Duplicated in FCLC/KNOWLEDGE.md**

---

### [Ze] EEG-based Brain Age Clock — Breakthrough 2026

**Sources:**
- [BrainYears: EEG Brain Age Clock — bioRxiv 2026](https://www.biorxiv.org/content/10.64898/2026.03.26.714124v1.full)
- [Wearable Aging Clock — Nature Communications 2025](https://www.nature.com/articles/s41467-025-64275-4)
- [WHOOP 2026 Health Report: Rise of Healthspan — The Manual](https://www.themanual.com/fitness/whoop-2026-health-report/)
- [HRV-CV as Digital Biomarker 2026 — Science for ME](https://www.s4me.info/threads/heart-rate-variability-coefficient-of-variation-during-sleep-as-a-digital-biomarker-that-reflects-behavior-and-varies-by-age-and-sex-2026-grosicki.49521/)

**Key Theses:**
- **BrainYears (bioRxiv, March 2026):** EEG brain age clock using ML — Pearson r = **0.92**, MAE = **4.43 years**. Neuromodulation intervention reduced predicted brain age by **−5.18 years** in the group. No MRI — only EEG. Portable, cost-effective, repeatable measurements at home
- **Nature Communications (2025):** Wearable PPG aging clock strongly associated with diseases, behavior, longitudinal changes. Confirms: wearable bio-age viable
- **HRV-CV (2026):** coefficient of variation of HRV overnight = scalable digital biomarker for behavioral monitoring, risk stratification. Associated with alcohol, physical activity, sleep quality
- **WHOOP Age:** 9 parameters (sleep consistency, HRV, time in HR zones, etc.) → biological age. Mass-market product confirming market demand

**Connection to Ze:**
> BrainYears (r=0.92, MAE=4.43 years) — direct competitor/validator of χ_Ze. Chi_Ze gives R²=0.84 ~~[withdrawn: synthetic data; see CORRECTIONS §1.2]~~ on EEG+HRV. BrainYears — pure EEG r=0.92. Needed: 1) position χ_Ze as more interpretable (physical meaning vs black-box ML); 2) WHOOP Age = proof of market.

**→ Duplicated in Ze/KNOWLEDGE.md**

---

### [BioSense] Wearable EEG/HRV for Longevity 2026

**Sources:**
- [Wearable EEG for MCI Detection — npj Digital Medicine 2026](https://www.nature.com/articles/s41746-026-02342-w)
- [Bioelectric Signal Healthcare Monitoring — npj Biomedical Innovations 2025](https://www.nature.com/articles/s44385-025-00061-7)
- [Hume Band: Biological Age Wearable 2026 — Newswire](https://www.newswire.com/news/hume-band-review-2026-biological-age-metabolic-health-wearable)
- [10 Top HRV Biofeedback Monitors 2026 — Outliyr](https://outliyr.com/best-hrv-biofeedback-monitors)

**Key Theses:**
- **npj Digital Medicine (2026):** Wearable EEG shows high potential as a screening tool for MCI (mild cognitive impairment) — direct measurement of neural oscillations and functional connectivity
- **Hume Band 2026:** commercial wearable with HRV, SpO2, sleep, temperature → longevity feedback + biological age. Wearable sleep-tracking market reached **$7B** by 2026
- **Bioelectric signals:** multi-sensor EEG+PPG → real-time brain activity + blood flow + cognitive performance + emotional regulation — all in one device
- Main problem of consumer EEG: overestimation of sleep, underestimation of wakefulness vs polysomnography

**Connection to BioSense:**
> Hume Band — direct analogue of BioSense MVP. BioSense should be positioned: 1) open standard (not closed Hume), 2) χ_Ze as a theoretically grounded index (not proprietary black-box), 3) integration with CommonHealth/FCLC.

**→ Duplicated in BioSense/KNOWLEDGE.md**

---

### [CDATA] Centrosomes, Aging, and Senescence 2025–2026

**Sources:**
- [Drivers of Centrosome Abnormalities: Senescence Progression and Tumor Immune Escape — ScienceDirect 2025](https://www.sciencedirect.com/science/article/abs/pii/S1044579X25000173)
- [PLK4: Master Regulator of Centriole Duplication — Cytoskeleton 2025](https://onlinelibrary.wiley.com/doi/full/10.1002/cm.22031)
- [Senescence in Cancer — Cancer Cell 2025](https://www.cell.com/cancer-cell/fulltext/S1535-6108(25)00224-7)
- [Centrosome Dysfunction: Link Between Senescence and Tumor Immunity — Nature STTT](https://www.nature.com/articles/s41392-020-00214-7)

**Key Theses:**
- **ScienceDirect 2025:** Centrosome aberrations — hallmarks of cancer + senescence. ECASP (extra centrosome-associated secretory phenotype) via chronic NF-κB activation → IL-8, GDF-15, ANGPTL4. IL-8 = component of SASP → immunosuppressive microenvironment
- **PLK4 clinical trials (2025):** PLK4 — master regulator of centriole duplication. Inhibitor **RP-1664** (orally bioavailable) entered clinical trials
- **Cancer Cell 2025:** Senescence plays a dual role in precancer: first a tumor-suppressive barrier, then a pro-tumoral PreTME via paracrine SASP
- Centrosome amplification — the most frequent defect in tumors; associated with genomic instability and accelerated aging

**Connection to CDATA:**
> PLK4 inhibitor in the clinic — direct confirmation of therapeutic direction #2 (proteasomal clearance / regulation of duplication). ECASP + NF-κB pathway aligns well with the CDATA model: damaged centrioles → SASP → senescence → tissue aging. Update the MCAI model considering the ECASP component.

**→ Duplicated in CDATA/KNOWLEDGE.md**

---

## Links for Further Reading

| Topic | Priority | Link |
|-------|----------|------|
| BrainYears preprint (full text) | HIGH | https://www.biorxiv.org/content/10.64898/2026.03.26.714124v1.full |
| Federated Microservices + Blockchain (Nature) | HIGH | https://www.nature.com/articles/s41598-026-39837-1 |
| PLK4 inhibitor RP-1664 clinical trial | HIGH | https://onlinelibrary.wiley.com/doi/full/10.1002/cm.22031 |
| Wearable EEG for MCI (npj Digital Med) | MEDIUM | https://www.nature.com/articles/s41746-026-02342-w |
| DiScO — Digital Citizen Science Observatory | MEDIUM | https://www.frontiersin.org/journals/digital-health/articles/10.3389/fdgth.2024.1399992/full |
| SHIRE platform UNC (08.04.2026) | MEDIUM | https://www.unc.edu/posts/2026/04/08/university-unc-health-unveil-shire-health-care-innovation-platform/ |
| Senescence in Cancer (Cancer Cell 2025) | MEDIUM | https://www.cell.com/cancer-cell/fulltext/S1535-6108(25)00224-7 |
| HRV-CV biomarker 2026 | LOW | https://www.s4me.info/threads/heart-rate-variability-coefficient-of-variation-during-sleep-as-a-digital-biomarker-that-reflects-behavior-and-varies-by-age-and-sex-2026-grosicki.49521/ |

---

*NEWS.md — core file of LongevityCommon | update at each session*