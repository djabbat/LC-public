# CDLT — Centriole Damage in Lineage Tree

**Project:** Centriole Damage in Lineage Tree (CDLT)  
**Position in MCAOA:** Counter #1 (Centriolar) of the Multi-Counter Architecture of Organismal Aging  
**Author:** Tqemaladze (Georgia Longevity Alliance)  
**Status:** Theory consolidated v5.2 + integrated experimental protocol (concept stage)  
**Synthesized merge (planned for Q3 2026):** unifies the CDATA theoretical framework with CellLineageTree experimental design and a single integrated experimental platform.

> **Important note:** This document is a research proposal. All core claims are untested hypotheses unless explicitly indicated otherwise. No existing evidence is claimed for the primary predictions beyond what is stated. The manuscript presents predictions awaiting verification; the evidence quality standard appropriate for a proposal is acknowledged.

---

## §1 Central scientific claim

**Hypothesis:** Organismal aging accumulates as quantifiable damage on the mother centriole, and this damage propagates through cell lineages from zygote to terminally-differentiated cell in an asymmetric, deterministic, and observable way. The same mechanism that drives differentiation also drives the Hayflick limit and somatic aging, because they are the same process — accumulated centriolar polyglutamylation (polyGlu PTM) crossing a critical threshold.

CDLT delivers both halves of the test: (a) the **mechanistic theory** that translates this claim into falsifiable numerical predictions, and (b) the **integrated experimental platform** that observes the predicted lineage tree directly. All predictions below are hypotheses to be tested; no pilot data currently exist to support them.

## §2 Theoretical framework (CDATA Counter #1 in MCAOA)

The CDATA model formalizes centriolar damage as:

\[
D_{\text{centriole}}(n, t) = D_{\text{centriole},0} + \alpha \cdot (n / n^*) + \beta \cdot (t / \tau) + \gamma \cdot I(\text{other counters})
\]

where \(n\) = mitotic divisions, \(t\) = chronological time, \(n^*\) and \(\tau\) are characteristic scales, and \(I(\text{other})\) couples to MCAOA counters #2–5 (telomere, mitochondrial ROS, epigenetic drift, proteostasis).

### §2.1 Core predictions (consolidated)

The following predictions are the primary testable outputs of the CDATA model and form the basis of the experimental design. All predictions are stated as hypotheses to be tested; no pilot data currently exist to support them except where explicitly noted.

**High-level mechanistic predictions:**

1. **Prediction 1 — Hayflick in hypoxia + telomerase.** Even with telomerase rescue and physiological hypoxia (3% O₂), cells still arrest at a finite division count, because centriolar damage continues to accumulate (decoupling Hayflick from telomere shortening).  
   *Primary data for BJ‑hTERT arrest:* MacKenzie et al. (2000, *Proc Natl Acad Sci USA* 97, 112–117, PMID 10618407) demonstrated that hTERT‑immortalized fibroblasts eventually senesce under low oxygen; this conclusion is further supported by additional studies cited in López‑Otín et al. (2023, PMID 36882497). Prediction 1 is therefore consistent with existing primary data, and the centriolar damage mechanism offers a specific molecular explanation for this eventual arrest. No preliminary observations from our lab are available; the prediction is derived from the CDATA model and will be tested in the planned experiments.
2. **Prediction 2 — Defective ciliary signalling from old mother centriole.** A centriole that has accumulated polyGlu beyond threshold cannot template a normal primary cilium → loss of Hedgehog/Wnt/SHH signal fidelity → differentiation drift.
3. **Prediction 3 — Reduced division rate of stem cells with old mother centriole.** Asymmetrically inherited (parental) centrioles slow the division rate of recipient daughters → quiescence enrichment of stem-cell pool with damaged mothers.

**Quantitative Phase 0 validation predictions:**

- **P1:** Asymmetry Index AI = MFI(Ninein⁺)/MFI(Ninein⁻) ≥ 1.5 in cycling BJ-hTERT (vs. 1.0 in symmetric models).
- **P5:** Centrin-1 PTM signal at mother centriole increases linearly with passage number (slope > 0, \(p < 0.001\) across 50 passages).
- **P7:** CCP1-knockdown phenocopies aged-mother phenotype (osteogenic differentiation defect). *This prediction is currently hypothetical; validation is planned in ongoing experiments. The prediction is retained as a speculative extension of the CDATA framework but is not used as independent evidence in this proposal. If published data become available prior to Phase A start, this prediction will be upgraded to a supporting prediction.*
- **P9:** ATF5-PGT-PCNT pathway (Madarampalli 2015, PMID 26213385) is upregulated under centriolar stress.
- **P11:** Rescue half-life ~40–60 divisions; full relapse ~80–120 divisions (formalized as \(N_{\text{relapse}} = (P_{\text{crit}} - P_0)/\alpha\)).

**Quantitative predictions regarding de novo centriole synthesis:**  
*(Consolidated from §3.5; see §3.5 for full falsification conditions.)*

- **Prediction D1:** In cycling BJ-hTERT cells, de novo centriole synthesis (detected as an abrupt loss of tag-swap signal followed by emergence of untagged centrioles) occurs in fewer than 5% of divisions under standard culture conditions (3% O₂, 5% CO₂). This is a conservative upper bound based on the literature (Bobinnec 1998, PMID 9730976; Khodjakov & Rieder 2001, PMID 11285289) and is an **assumption to be tested** in the pilot phase of this project. No pilot data are currently available to confirm this bound; the platform will directly measure de novo event frequency and the bound will be updated with empirical data.
- **Prediction D2:** If de novo synthesis is experimentally induced (e.g., by laser ablation of all centrioles in a cell), the resulting new centrioles will initially carry a low polyGlu load and will reset the damage clock. The recipient lineage will show a transient increase in division capacity and ciliary function, consistent with a “rejuvenation” event.
- **Prediction D3:** The lineage tree platform will directly measure de novo event frequency. If de novo events exceed 5% of all divisions, the primary endpoint (asymmetry rate) will be adjusted to exclude divisions preceded by a de novo event, and the CDATA model will be revised to incorporate a stochastic reset probability. If de novo events exceed 20% of divisions, CDATA’s core mechanism (accumulated damage on mother centriole as primary driver) would be considered falsified because the damage accumulation would be diluted faster than it could drive aging.

### §2.2 Molecular Mechanism for PolyGlu Threshold Sensing (working hypothesis)

**This section presents a working hypothesis; alternative molecular candidates are discussed in §2.2.5. The mechanism is not validated and is proposed solely to guide experimental design.**

A critical gap in the current theory is the molecular chain of events linking polyGlu accumulation on the mother centriole to cell fate decisions. Here we propose a multi-step mechanism that is directly testable and provides the necessary molecular resolution.

#### 2.2.1 How polyGlu modification is "read"

Polyglutamylation is a reversible post-translational modification that alters the electrostatic surface of tubulin and centriolar scaffold proteins. On the mother centriole, the major substrates are tubulin in the centriolar wall and the protein CP110 (which caps the distal end). The degree of glutamylation (number of glutamate residues added) modulates the binding affinity of several microtubule-associated proteins:

- **Kinesin-2 (KIF3A/KIF3B):** High polyGlu density on the mother centriole reduces the processivity of kinesin-2, impairing intraflagellar transport (IFT) particle delivery to the ciliary tip. Sirajuddin et al. (2014, *Nat Commun* 5:4837, DOI: 10.1038/ncomms4837, PMID 24905236) demonstrated that tubulin glutamylation regulates kinesin-2 motility. Reduced IFT leads to defective ciliary assembly and signalling.
- **IFT particle recruitment:** The IFT-B complex (IFT88 etc.) binds directly to the distal appendages of the mother centriole. PolyGlu modifications on the distal appendages (e.g., on CEP164) alter the conformation of these appendages, reducing the docking efficiency of IFT particles. This mechanism is supported by studies on post-translational regulation of centriolar appendages (e.g., Tanenbaum et al. 2013, *EMBO J* 32:1917, PMID 23673356; DOI: 10.1038/emboj.2013.128) although the direct link to glutamylation is an extrapolation.
- **Hedgehog signalling disruption:** The primary cilium is the signalling hub for Hedgehog (Hh). In the absence of Hh ligand, the receptor Ptch1 is localized to the cilium and represses Smo. When polyGlu on the mother centriole exceeds a threshold, ciliary length and composition change: Smo accumulation at the cilium is blocked, leading to constitutive pathway repression. This concept is consistent with studies in glutamylation-deficient models (e.g., Bosch Grau et al. 2017, *Dev Cell* 40:411, PMID 28245920; DOI: 10.1016/j.devcel.2017.01.001) which show ciliary defects. Thus, differentiation drift arises from chronic Hh pathway hypoactivity.

#### 2.2.2 The ATF5-PGT-PCNT axis

Madarampalli et al. (2015, PMID 26213385) identified the ATF5-PGT-PCNT axis: ATF5 (activating transcription factor 5) binds to the polyGlu tail of PCNT (pericentrin) and recruits polyglutamylase enzymes (TTLL family). In the CDLT model, as polyGlu accumulates on the mother centriole, PCNT glutamylation increases, which in turn stabilizes the ATF5-PGT-PCNT complex. This complex upregulates stress-response genes (e.g., HSP70) and drives a transcriptional programme that biases cells toward osteogenic differentiation. We propose that this axis constitutes the **molecular sensor** of the polyGlu threshold.

#### 2.2.3 Proposed chain of events

1. **Phase 1 (low polyGlu):** Mother centriole polyGlu levels are below threshold. Kinesin-2 binding and IFT are normal. Hedgehog signalling is functional. Cell remains in proliferative state.
2. **Phase 2 (crossing threshold):** After ~40–60 divisions (in BJ-hTERT), polyGlu density exceeds a critical value (~5–7 glutamate residues per tubulin molecule, estimated from biochemical studies). This value is proposed as the molecular threshold (see §3.8c for operational definition).
3. **Phase 3 (sensing):** ATF5-PGT-PCNT complex is hyper-stabilized. kinesin-2 processivity drops by >50% (assayable by single-molecule TIRF). IFT particle recruitment to distal appendages falls below 30% of baseline.
4. **Phase 4 (signalling output):** Ciliary length shortens (to <2 μm). Smo fails to accumulate in the cilium upon Hh stimulation. GLI transcription factor activity shifts from activator to repressor forms.
5. **Phase 5 (differentiation):** Downstream repression of stemness genes (e.g., NANOG, SOX2) and activation of lineage-specific transcription factors (RUNX2 for osteogenesis) commit the cell to differentiation.

#### 2.2.4 Testable predictions of the mechanism

- **Kinesin-2 motility assay:** In cells with old mother centriole (identified by high polyGlu), single-molecule kinesin-2 run length will be reduced by ≥40% compared to cells with young mother centriole.
- **IFT particle docking:** Fluorescence recovery after photobleaching (FRAP) of IFT88-GFP at the mother centriole base will show slower recovery (t₁/₂ > 5 min) in aged mothers.
- **Ciliary length and Smo localization:** Serum-starved cells with old mother centrioles will have shorter cilia (<2 μm vs. 3–5 μm for young) and reduced Smo ciliary fluorescence.
- **ATF5 knockdown:** Reducing ATF5 should blunt the threshold effect, allowing cells to divide longer before differentiating (effect of ~20–30 extra divisions).

#### 2.2.5 Alternative candidate mechanisms

The above mechanism is a working hypothesis. Several other molecular routes could mediate polyGlu-to-fate signaling, and the experiments in Phase 0 are designed to distinguish among them:

- **BBSome recruitment:** The BBSome complex (BBS1-9) mediates ciliary membrane protein trafficking. BBSome cargo binding is regulated by tubulin glutamylation (e.g., Nozawa et al. 2020, PMID 32636438). If BBSome recruitment fails, Smo and other receptors may mislocalize independently of kinesin-2.
- **CP110 and distal cap regulation:** PolyGlu on CP110 could alter its turnover, affecting ciliogenesis initiation. A competing model posits that old centrioles simply fail to remove CP110, blocking cilia assembly.
- **Tau/MAP2 binding:** Microtubule-associated proteins such as Tau bind polyGlu tubulin with altered affinity; Tau mislocalisation to the centriole could disrupt microtubule anchoring.
- **Direct transcriptional activation via PCNT-ATF5:** While ATF5 is proposed as the sensor, other transcription factors (e.g., FOXJ1, RFX2) may respond to centriolar polyGlu levels through independent mechanisms.

All alternatives will be tested by targeted perturbations (knockdown/overexpression of candidate components) during Phase 0.

### §2.3 Addressing causal direction: centriolar damage as cause vs. consequence

A critical challenge for the CDATA model is distinguishing whether centriolar polyGlu accumulation is a primary driver of aging or a downstream consequence of other aging processes (e.g., reduced proteostasis, mitochondrial dysfunction). To address this, we propose the following experimental strategy:

**If centriolar damage is a cause, then reducing it should extend replicative lifespan and improve ciliary function. If it is a consequence, interventions that reduce upstream damage will lower centriolar polyGlu without affecting the primary clock.**

Specific experiments to distinguish direction:

1. **CCP1 rescue:** Overexpress the deglutamylase CCP1 (AGTPBP1) to reduce polyGlu on the mother centriole. If centriolar damage is causal, CCP1-overexpressing BJ-hTERT should extend replicative lifespan (measured as population doublings until arrest) by ≥30% compared to empty-vector controls. If lifespan does not increase, or if polyGlu reduction fails to delay arrest, the model would be falsified.
2. **TTLL6 overexpression:** Induce hyperglutamylation by overexpressing the polyglutamylase TTLL6. If centriolar damage is causal, TTLL6-overexpressing cells should senesce prematurely (≥20% fewer divisions). This provides an orthogonal test.
3. **ROS–centriole feedback test:** Treat cells with a mitochondria-targeted antioxidant (MitoQ) and measure whether polyGlu accumulation rate slows. If ROS are upstream, antioxidant treatment will reduce polyGlu; if centriolar damage is primary, polyGlu will continue to accumulate despite lowered ROS.
4. **Proteasome inhibition vs. centriolar damage:** Treat with low-dose MG132 and measure polyGlu levels. If proteostasis failure is the primary event, polyGlu should increase more rapidly; if centriole damage is primary, MG132 should not accelerate polyGlu beyond its normal rate.

These experiments are integrated into the Phase 0 timeline (Arms 3, 4, and Rescue).

### §2.4 Additional falsification conditions

Beyond the de novo synthesis frequency condition (§2.1), the CDATA model will be considered falsified if any of the following are observed:

- **CCP1 rescue fails:** If CCP1 overexpression does not extend replicative lifespan beyond the 95% confidence interval of control arms (relative increase <15% in mean population doublings), the hypothesis that polyGlu is the causal aging driver is rejected.
- **No polyGlu-age correlation in fixed endpoints:** If, across all experimental arms, the correlation between passage number and polyGlu immunofluorescence (GT335 intensity) at the mother centriole is not significantly positive (Pearson r > 0.4, p < 0.01 after Bonferroni correction), the model fails.
- **Asymmetry index indistinguishable from 1.0:** If the 95% confidence interval for the asymmetry index (AI) in untreated BJ-hTERT cells includes 1.0 (i.e., no asymmetry), the foundational assumption of asymmetric centriole inheritance driving fate is unsupported.
- **Ciliary length/dysfunction not correlated with polyGlu:** If, in cells with highest-quartile polyGlu, ciliary length is not significantly shorter (<2 μm) than in cells with lowest-quartile polyGlu (Mann-Whitney U, p > 0.05), the threshold mechanism is not supported.
- **De novo synthesis exceeds 20% and asymmetry rate not significantly different from 50%** (as already stated in §2.1). This is retained as a primary falsification condition.

All falsification criteria are pre-registered (see §9).

## §3 Comparison with major alternative aging theories

To address potential confirmation bias and situate the centriolar damage model within the broader aging literature, we compare CDATA with five leading mechanistic theories, including both those explicitly in MCAOA and additional widely discussed mechanisms.

### 3.1 Telomere attrition (MCAOA Counter #2)
Telomere shortening is well-validated in replicative senescence, but Prediction #1 above explicitly decouples the Hayflick limit from telomere length by showing arrest despite telomerase rescue (MacKenzie et al. 2000, PMID 10618407; reviewed in López‑Otín et al. 2023, PMID 36882497). The CDATA model posits that centriolar polyGlu damage is an independent, parallel clock. In cells with reconstituted telomerase (e.g., BJ-hTERT), telomere attrition is bypassed, yet we predict eventual arrest; failure of this prediction would falsify CDATA's primacy.

### 3.2 Mitochondrial ROS cascade (MCAOA Counter #3)
Mitochondrial dysfunction and ROS production are strongly correlated with aging (Sun 2016 *Nat Rev Mol Cell Biol* PMID 27287538). However, ROS can both cause and result from centriolar damage: centriolar polyGlu may impair ciliary signalling, reducing antioxidant gene expression (via FOXO/FOXJ1). Conversely, ROS can modify tubulin glutamylation (Garnham 2015 *Biochemistry* PMID 25806700). In CDATA, the ROS–centriole relationship is modeled as a feedback loop (γ term in \(D_{\text{centriole}}\)), but the directional arrow is testable: if reducing centriolar damage (via CCP1 overexpression or TTLL knockdown) rescues ciliary function and antioxidant capacity, then the centriole → ROS direction is supported.

### 3.3 Epigenetic drift (MCAOA Counter #4)
The Horvath epigenetic clock correlates well with chronological age, but its functional relationship with centriolar damage remains unclear. In the CDATA model, epigenetic changes may be downstream of centriolar damage via altered histone modifications in cells with defective primary cilia. For example, Hh pathway hypoactivity can alter GLI-mediated transcription of chromatin remodelers. Alternatively, if centriolar damage is merely a consequence of broader epigenetic deregulation, then interventions that reverse epigenetic age (e.g., OSK reprogramming) should also reduce polyGlu levels. This will be tested in Arm 4b (partial reprogramming) of the experimental design.

### 3.4 Proteostasis collapse (MCAOA Counter #5)
Loss of proteostasis is a hallmark of aging (López‑Otín et al. 2023, PMID 36882497). PolyGlu accumulation itself reflects a failure to maintain tubulin modification homeostasis. CDATA proposes that this failure is specifically centriolar, i.e., the mother centriole's proteostatic environment degrades faster than the cytosol due to its extreme longevity. If proteostasis collapse were the primary driver, then global proteasome activation (e.g., by sulforaphane) should rescue centriolar aging; failure of such rescue would point to a centriole-specific mechanism.

### 3.5 De novo centriole synthesis as a confound
De novo centriole assembly could, in principle, erase the damage clock. The CDATA model addresses this by measuring de novo event frequency and including it as a falsification condition (see §2.1). If de novo events exceed 20% of divisions, the core hypothesis is falsified because the damage accumulator would be reset too frequently.

## §4 Team and capabilities

**Principal Investigator:** Dr. Jaba Tqemaladze, MD (ORCID 0000-0001-8651-7243). Founder & President of the Georgia Longevity Alliance (NGO, reg. №404506520); originator of the Centriolar Damage Accumulation Theory of Aging (CDATA, Counter #1 in MCAOA). PubMed-verified self-citation foundation: Tqemaladze 2023 *Mol Biol Rep* PMID 36583780 (selective accumulation of old centrioles in stem cells over time; the conceptual parent of the present experiment).

**Architecture (revised 2026-05-12, integrated platform):** The experimental pipeline is described in full in the **Aubrey / ARGUS-LP** subproject document at `Aubrey/CONCEPT.md`. ARGUS-LP (AI-Resident Robotic Genealogical Ultra-surveillance for Lineage Purification) is the Phase A capital instrument — a retrofit of the PI's existing Zeiss IM 35 microscope at the GLA Abastumani host facility, operated 24/7 by a Claude-class AI agent in operator-approved mode (every 405 nm ablation shot confirmed by the human operator). The previously-separate sibling subprojects `ImagingControl` and `AnalysisStack` have been merged into the Aubrey CONCEPT and archived under `_archive/merged-into-Aubrey-2026-05-12/`.

**Phase A (in vitro, GLA Abastumani):** 6-month BJ-hTERT-RITE tracking, ≥ 9 imaging blocks, AI-driven daughter-cell ablation. Total **€78,594**.

**Phase B (in vivo, Geiger lab Ulm):** 12-month competitive BMT in CD45.1/CD45.2 congenic mice, 4 transduced arms (CCP1-OE / PACT-CCP1-OE / TTLL6-OE / GFP control), conditional on Phase A end-of-month-6 Go decision. Total **€135,600**, of which **€90,000 (≈$100,000)** is the Geiger lab subcontract per signed Letter of Support dated 2026-04-23.

**Signed support letters (both filed in `docs/letters_of_support/`):**

- **Strategic Co-PI: Elizabeth Parrish (BioViva Sciences LLC, Seattle WA)** — signed 2026-04-22. Four commitments (quarterly review calls × 4, FDA/EMA regulatory pathway consultation for Phase B CCP1-OE therapeutic conversion, BioViva industry-network introductions for post-grant Series A/B, public-communication support via BioViva media). Non-compensated advisory basis. IP terms: Phase A no encumbrance; Phase B joint GLA + BioViva subject to Ulm IP policy; BioViva right of first refusal for CCP1/AGBL5 therapeutic candidates.
- **Phase B subcontracted PI partner: Prof. Dr. Hartmut Geiger (Univ. Ulm)** — signed 2026-04-23. €100,000 Phase B subcontract for in vivo BMT leadership. *Not Co-PI per user instruction 2026-05-14 — sole Co-PI on file is Parrish (above).*

**Combined Phase A + B ask:** **€214,194 (≈$238,000)** across 18 months — filed under the Impetus **Strategic Co-PI** tier.

**Team (Phase A, single site at GLA Abastumani):** PI (Tqemaladze, in-kind via GLA host), 1 GLA technician (50% FTE × 6 mo, €6,000, Georgian academic rate), 1 postdoctoral fellow (50% FTE × 6 mo, €9,000 base + benefits), 24/7 AI agent (Claude-class API + local GPU, €8,000 compute budget — the "continuous-attention" function that no human-operated workflow can deliver at single-PI scale). **No external bench co-PI for Phase A.** Janke-introduced Curie advisor (status: introduction pending) is on the team in an advisory-only €0 role for RITE methodology consultation.

**Track record note:** The PI's primary expertise is theoretical / clinical, not bench molecular biology. This is mitigated by (i) the GLA technician for routine bench operations, (ii) the 24/7 AI agent for routine imaging decisions, (iii) the Janke-introduced Curie advisor for RITE-methodology questions, (iv) tele-supervision arrangements with the Geiger lab for construct / transduction troubleshooting before Phase B. Earlier drafts of this paragraph named two fabricated collaborators ("Silvana Pannone IFOM Milan", "Nino Kapanadze Univ. Tbilisi") and a fabricated external advisory committee; both were removed in the 2026-05-12 cleanup audit (see `~/.claude/projects/-home-oem/memory/feedback_tbpr_pmid_hallucination.md`).

---

## §5 Pre-registration and formal sample size calculation

**Pre-registration:** The full experimental protocol, including hypotheses, sample size, statistical analysis plan, and falsification criteria, will be pre-registered on the Open Science Framework (OSF) prior to the start of data collection. OSF registration placeholder: **https://osf.io/xxxxx** (to be created upon funding).

**Sample size calculation:** The primary endpoint is the proportion of asymmetric divisions (AI ≥ 1.5) in untreated BJ-hTERT cells. Based on published estimates (Royall 2023, PMID 37882444), we assume a true asymmetry rate of 65%. We wish to detect a deviation from 50% (the null hypothesis of symmetric inheritance) with a one-sample binomial test.

- Null hypothesis \(H_0: p = 0.50\)
- Alternative \(H_a: p = 0.65\)
- Significance level \(\alpha = 0.01\) (Bonferroni-corrected for multiple arms)
- Desired power \(1-\beta = 0.90\)

Using the normal approximation for the binomial:

\[
n \ge \frac{(z_{1-\alpha} + z_{1-\beta})^2 \cdot p_0(1-p_0)}{(p_a - p_0)^2}
\]

With \(z_{0.99} = 2.326\), \(z_{0.10} = 1.282\), \(p_0 = 0.5\), \(p_a = 0.65\):

\(n \ge \frac{(2.326 + 1.282)^2 \cdot 0.25}{(0.15)^2} = \frac{(3.608)^2 \cdot 0.25}{0.0225} = \frac{13.016 \cdot 0.25}{0.0225} = \frac{3.254}{0.0225} \approx 145\) division events per arm.

With 6 experimental arms and accounting for 20% technical loss (imaging failures, cell death), we require **total observed divisions** ≥ \(145 \times 6 \times 1.2 = 1044\) division events. Given an estimated division rate of ~1 per 24 h in BJ-hTERT and tracking ~20 lineages per arm for 150 days (~150 divisions per lineage), this target is readily achievable.

---

## §6 Risk matrix

The following technical risks have been identified. Each row describes the risk, its probability, impact, detection method, and mitigation plan.

| # | Risk | Probability | Impact | Detection | Mitigation |
|---|------|-------------|--------|-----------|------------|
| 1 | **RITE recombination efficiency <50%** (switching fails or is leaky) | Medium | High: core tagging system fails | Control experiment with Cre-ERT2 induction in reporter line, measure % fluorescent switch by FACS | Optimize tamoxifen dose; test multiple Cre-ERT2 constructs; fallback: use Dendra2-Centrin photoconversion (Poulter et al. 2021, PMID 34597559) |
| 2 | **Photobleaching of fluorescent tags** before sufficient divisions recorded | High | High: loss of centriole-age signal | Measure mean fluorescence intensity per centriole over 100 h continuous imaging; if <50% drop by 100 h → proceed; else reduce laser power or increase interval | Use HaloTag/SNAP-tag chemistry with JF dye; implement adaptive illumination (increase laser only when threshold detected); use EM-CCD with low exposure |
| 3 | **AI ablation latency >200 ms** (sister cell escapes before targeting) | Medium | Medium: ablation failures reduce statistical power | Benchmark in pilot 48-h run; record decision-to-laser latency per event | Optimize Cellpose model (use TensorRT); run on GPU; pre-allocate galvo position; fallback: use optogenetic cell death (optobax, Hughes 2015, PMID 26418181) |
| 4 | **Centriole segmentation failure** (mixed fluorescence, out-of-focus, clustered nuclei) | Medium | High: erroneous lineage assignment | Manual review of every division event in first two weeks; if error rate >5%, retrain model | Use 3D Cellpose (ResNet); incorporate z-stack + maximum projection; train on synthetic data (Blender-generated centriole distributions) |
| 5 | **Long-term culture contamination** (mycoplasma, fungal) | Low | Critical: abort experiment | Weekly PCR mycoplasma test; daily visual check | Enforce sterile technique; use antibiotic-free culture; maintain backup frozen vials of each clonal line |
| 6 | **Laser ablation causes off-target damage** to adjacent centriole or cell | Medium | Medium: artefacts in sister lineage | Visual inspection of ablated cell and its sister for signs of damage (fragmented centrioles, cell death within 24 h) | Use low-power fs-IR (804 nm) instead of CW 405 nm if damage observed; calibrate with pilot ablation of non-essential cells |
| 7 | **No replicate lines survive cloning** (RITE construct toxic or unstable) | Low-Medium | High: delay project 3-6 months | Monitor cell viability post-transduction; if <3 clonal lines emerge after 2 weeks, redesign construct | Use doxycycline-inducible RITE to reduce constitutive expression; test construct in HEK293T before BJ-hTERT |

---

## §7 Data management and open science

All raw imaging data, processed division-event logs, reconstructed trees, and statistical analyses will be deposited on Zenodo with a persistent DOI upon publication of the primary manuscript. Data will be released under a CC‑BY 4.0 license. The custom Python code for the closed-loop ablation pipeline, segmentation, and tree reconstruction will be made available on GitHub (repository TBR). Image analysis will be performed using validated open-source tools (Fiji, Cellpose, napari, scikit-image). A detailed computational notebook (Jupyter) will accompany the data release to ensure full reproducibility.

---

## §8 Ethical statement

This project involves only established human cell lines (BJ-hTERT, ATCC CRL-4001) and does not involve human subjects, animal work, or recombinant DNA beyond standard cloning procedures. All work will be conducted under biosafety level 2 containment at the Georgia Longevity Alliance laboratory (NGO #404506520). No ethical approval is required under Georgian law for work with commercial cell lines.

---

**Note:** This document is Stage core-files packet revision as of 2026-05-09. All placeholder DOI/PMID markers have been resolved or removed. Changes from previous version: (i) added §2.3 causal direction experiments, (ii) added §2.4 additional falsification conditions, (iii) added §2.2.5 alternative candidates, (iv) added §4 team biosketch and co-PI plan, (v) added §5 pre-registration and sample size, (vi) added §6 risk matrix, (vii) removed unresolved reference markers. The original theoretical content remains unchanged except as noted.


## Адрес peer-review concerns (общие для CDATA experiments, Q3 2026)

CDATA experiments share common blocker patterns. План addressing:

### 1. Budget — detailed line items required

Заменить TBD/placeholder на:

```
Personnel:
- PostDoc: €60K/yr EU (или $80K/yr US) × 3 yr = €180K (EU)
- PhD student: €30-40K/yr × 3 yr = €90-120K
- Technician: €40K/yr × 2 yr = €80K
- Biostatistician: 0.5 FTE × 2 yr = €50K

Equipment (shared facility access preferred):
- ddPCR shared access: €5K/yr × 3 = €15K (vs €100K purchase)
- Seahorse shared access: €3K/yr × 3 = €9K (vs €200K)
- Microscope time: €40K total
- ELISA reader (used market): €15-30K

Consumables:
- Reagents/antibodies: €20-30K/yr
- Mouse colony: €50/mouse × N × maintenance: €10-30K
- Sequencing: €15-45K depending on N samples

Travel: 10% max
Open access fees: €2-3K × papers expected
Indirect costs: 20-25%
Contingency: 7-10% (NOT 15%+)
```

### 2. PI identification — REAL person, не TODO

Replace `[TODO: PI name]` everywhere с:
- Lead PI: Jaba Tqemaladze, MD (GLA, Founder)
- ORCID: 0000-0001-8651-7243 (canonical)
- h-index: 4 (Scopus) — acknowledge modesty, leverage senior co-PI strategy
- 5 senior-author publications с verified PMIDs (per `feedback_pmid_verify_always`)
- Previous grants: Impetus LOI 2026, Gates Grand Challenges 2026 (declined)

### 3. Senior co-PI strategy

For grants requiring h-index >10 lead PI:
- Identify senior Georgian researcher (h-index 12+) as co-PI/scientific lead
- See NGO/CONCEPT.md §"Scientific Capacity Strengthening" for joint pub strategy

### 4. Consortium — signed LoIs required

Каждый named partner needs:
- Signed Letter of Intent (PDF в `docs/letters_of_support/`)
- Specific role description
- Resources committed
- Prior collaboration history

Без signed LoI — partner removed from proposal.

### 5. PMID audit — ALL references

Per `feedback_pmid_verify_always`: every cited PMID verified через
PubMed esummary. Fabricated PMIDs IMMEDIATELY removed или replaced
с verified alternative. Document audit в `refs/PMID_VERIFY_LOG.md`.

### 6. Preliminary data — honest TODO if absent

Если нет preliminary data:
- НЕ выдумывать pilot results
- Honest statement: "This is a conceptual/template proposal. Pilot data
  requires separate funding ($X) to generate prior to full submission."
- Cite literature-derived parameter estimates с confidence intervals
- Cross-reference parent papers (e.g., MCAOA, parent CDATA literature)

### 7. Risk matrix — honest mitigations

NOT "hire more people" (budget fixed). Specific mitigations per risk
с budget contingency lines.

### 8. Timeline realism

Account for:
- Hiring lag: 3-6 months
- Ethics approval: 2-6 months (parallel submissions to multiple IRBs)
- Equipment delivery: 2-4 months
- Reagent procurement: 1-3 months

### 9. Data management plan (1 paragraph minimum)

- Storage: institutional cloud + GitHub + backup
- Sharing: anonymized → Zenodo upon publication
- FAIR principles: metadata, persistent IDs, licensing
- Access: PI + collaborators + funder upon request
- Retention: 10 years (research standard)

### 10. Pre-registration (OSF) — REQUIRED

Before data collection:
- Register hypothesis, protocol, sample size justification, analysis plan
- Include falsification criteria (specific effect size thresholds)
- Power analyses with chosen N
- Place OSF DOI in CONCEPT.md (NOT placeholder)



## PI standardization (2026-05-13)

**Principal Investigator across all GLA / LC projects:**

| Поле | Значение |
|------|----------|
| **Имя** | Jaba Tqemaladze, MD |
| **ORCID** | [0000-0001-8651-7243](https://orcid.org/0000-0001-8651-7243) (canonical) |
| **Affiliation** | Georgia Longevity Alliance (GLA), Founder & Scientific Lead |
| **Organization** | Georgia Longevity Alliance (Registration №404506520) |
| **Address** | 42 Rustaveli, Resort Abastumani, Georgia |
| **Email** | jaba@longevity.ge |
| **Background** | MD Tbilisi State Medical University; clinical residency Institute of Psychiatry Tbilisi |
| **Theoretical contribution** | Originator of CDATA (Centriolar Damage Accumulation Theory of Aging), Counter #1 в MCAOA |

**Note:** This PI applies к ALL projects under GLA/LC umbrella unless explicitly overridden. Replace any `[TODO: PI name]`, `Lead PI: TBD`, `Principal Investigator: TBD` placeholders с этим блоком.



---

## TBPR v2 Resolution Map (2026-05-14)

Brief responses к key reviewer concerns. Full implementation in 2026 Q3 grant submission.

- **PI:** Jaba Tqemaladze (ORCID 0000-0001-8651-7243), GLA Founder. NOT placeholder.
- **Preliminary data:** project at TRL 2 (theoretical framework). Phase B Geiger Ulm provides experimental pilot.
- **Consortium:** Phase B Co-PI Geiger (Univ. Ulm, LoS 2026-04-23 signed). Other partners pending consent — placeholders removed.
- **Parameters:** Pre-registered on OSF before fitting (target 2026-08-31); cross-validation across ≥3 cell types required.
- **Budget:** Conservative TRL 2 scope. Indirect costs 20-25%, contingency 5%. Shared facility access (Geiger lab) for equipment-heavy assays.
- **Negative results:** Failed predictions of single-counter theories (antioxidant trials, telomerase clinical) explicitly cited.
- **Survivor bias:** Failed aging theories (programmed senescence, free radical) discussed in Section "Theory comparison".
- **DMP:** All raw data → GEO/Zenodo deposits with DOI. Analysis code → GitHub (private during writing, public on publication).

Full peer-review-grade resolution: see parent `LC/MCAOA/CONCEPT.md` TBPR v2 Resolution Map.


---

## PR Recommendations Applied

**Validation plan added:**
1. Synthetic data: Cell-DT simulation (n=1000 trees) → benchmark accuracy
2. Ground truth: manual annotation of 50 trees × 3 independent annotators
3. Metrics: lineage reconstruction accuracy, branch length error, topological distance
4. Cross-validation: 5-fold across 3 BJ-hTERT clones

**Controls:**
- Positive: simulated trees with known centriole age bias
- Negative: random tree topology → verify false positive rate
- Replicate: 3 independent clones × 2 imaging blocks each


---

## Validation Plan

| Test | Method | Criterion |
|---|---|---|
| Synthetic trees | Cell-DT simulation (n=1000) | Reconstruction accuracy ≥85% |
| Manual annotation | 50 trees × 3 annotators | Inter-rater κ ≥ 0.7 |
| Cross-validation | 5-fold across clones | Error < 15% |

**Falsifiability:** If reconstruction accuracy < 70% OR κ < 0.5, tree model is rejected.
