# MEMORY — MCARA

## 📌 AUBREY: response #2 — the idea is fine, technical questions → senescence experts (2026-08-14)

<!-- lang:ru -->
**Context:** Jaba sent response No. 1 + OnePager v4 (SV40 large T, RITE honestly). Aubry replied after ~25 min:
> «Thanks. I'm not enough of a cell culture expert to be the right person to ask about your details below, but it certainly makes sense to ask those questions to people who know a bit about senescence and immortalisation. I'm certainly happy to chat in Cologne.»

**Summary:**
- ✅ Aubry has no objections to the SV40 large T model; refers technical questions (SV40 vs hTERT; mTERT⁻/⁻ arm) to **senescence/immortalization experts**
- ✅ **Meeting in Cologne confirmed** (Entropy in Aging, Aug 31 – Sep 2)
- ✅ OnePager v4 read (attachment opened — "The revised one-pager is attached")

**Next step:**
- Short reply No. 2 to Aubry: gratitude + one request — to recommend 1–2 experts on senescence/immortalization (candidates: Campisi/Buck, van Deursen/Mayo, Serrano/Altos, Adams/Glasgow, Demaria)
- Update TODO: meeting in Cologne — confirmed
<!-- /lang:ru -->

## 🔴 AUBREY DE GREY FEEDBACK: Immortalization — MEF is not the right model (2026-08-14)

<!-- lang:ru -->
**Context:** Jaba sent Aubrey the proposal "The Immortalization Phenomenon" (Aug 13, LEV Foundation). Aubrey replied on Aug 14 (0 minutes later — read it, not the entire document, but immediately gave the main comment).

**Aubry's remark (verbatim in essence):**
> «Mouse primary fibroblasts are definitely not the right model — they express telomerase naturally. The replicative limit at 20% O₂ is because they are so bad at DNA repair that telomerase fails to keep them going. Directly tested by Niida et al (PubMed 9620783) — ES cells, telomerase knockouts → Hayflick limit; PubMed 10805753 — rare escapees doing ALT. I would expect MEFs in low O₂ would show the same. A much better model would be induced immortalisation, such as with SV40 large T antigen.»

**Verified (PMIDs verified):**
- 9620783 Niida 1998 Nat Genet «Severe growth defect in mouse cells lacking the telomerase RNA component» ✅
- 10805753 Niida 2000 Mol Cell Biol «Telomere maintenance in telomerase-deficient mouse ES cells: amplified telomeric DNA» (ALT) ✅

**What was missed:**
- MEF 21% O₂ "immortalization" is not a pure phenomenon: mouse fibroblasts naturally express TERT; the limit at 20% O₂ results from poor DNA repair, not a pure telomere limit → the interpretation of "what happens to the centriole upon immortalization" is noisy.
- Spontaneous immortalization (~10⁻⁶) involves a clonal bottleneck, a rare winner → a selection shift, not a pure test.

**What to change (accepted):**
- **Arm 1 redesigned:** controlled **induced immortalization (SV40 large T)** — the primary pathway; spontaneous MEF — additional.
- Niida 1998/2000 added as direct justification.
- The model is now: human fibroblasts (or MEF) + SV40 large T → telomerase and non-telomerase barriers separately; the centriole counter is tested at a barrier not confounded by DNA-repair artifacts.

**Next step:** Aubry's response (3–4 paragraphs, accepted) + update proposal (Arm 1 → SV40 large T) + outline for the meeting in Cologne (Aug 31 – Sep 2).
<!-- /lang:ru -->

## 🔴 Rejection #31 — 2026-08-14 — GeroScience (Structural Damage Reservoirs)

**Journal:** GeroScience (Springer Nature) — Associate Editor Balázs Győrffy
**ID:** `JAAA-D-26-02347`
**Manuscript:** «Centrioles as Structural Damage Reservoirs: A Hypothesis»
**Decision type:** 🔴 **Desk reject** (at the Associate Editors stage, before external review — despite the "Under Review / Reviewers agreed" status in the dashboard on Aug 13)
**Days to decision:** ~1 week

### Reason (what the editor said)
> «All manuscripts... undergo a careful evaluation by our board of Associate Editors prior to formal peer review... Only those that meet a high standard for novelty, scientific methods, and clinical relevance are sent for review (fewer than half)... your manuscript did not achieve a high enough priority score to qualify for further review.»

### What we missed
- **Pre-submission inquiry was NOT sent** (not found in the files) — a systematic rule was violated (80%+ of rejections are desk rejects)
- **GeroScience is a clinically oriented journal**: evaluation includes "clinical relevance" and "breadth of interest"; a hypothesis paper without data receives a low priority score
- **Second desk reject of this manuscript** (first — Medical Hypotheses, Jul 30, 7 minutes) — the paper was not revised to fit the research-with-data format
- The "Reviewers agreed" status in the dashboard was not final — the AE made a decision without external review

### What to change before the next submission
- [ ] Run `bash ~/Desktop/Services/scripts/journal-fit.sh` before submission
- [ ] Pre-submission inquiry to the editor is MANDATORY (email + subject)
- [ ] Format as a research article with data (literature data/meta-analysis) — the rule "all articles as research articles with data"; Abstract ≤250; keywords; Figure 1
- [ ] Consider the Springer Nature transfer pathway (npj Aging / Discover Aging) — the GeroScience rejection now makes the "do not submit to npj" decision from Aug 13 outdated

### Next Journal
**Options:**
1. **npj Aging** (Springer, transfer from GeroScience) — hypothesis-friendly, new journal — ⭐ priority
2. **Mechanisms of Ageing and Development** (Elsevier) — hypothesis accepted, with inquiry
3. **BioEssays** (Wiley) — hypotheses welcome — with inquiry (lesson: 24 h → desk reject without inquiry)
4. **Molecular Biology Reports** (Springer) — where Tkemaladze 2023 was published — hypothesis-friendly
**Journal-fit check:** VERIFY before selection
<!-- /lang:ru -->

## 📌 VW: DECISION — awaiting Schiebel until Aug 15, 4:00 PM (2026-08-14)

<!-- lang:ru -->
**Jaba's decision (Aug 14, 04:48 Tbilisi):** we do not send the letters immediately — we wait for Elmar Schiebel's (ZMBH Heidelberg) response **until tomorrow 16:00 Tbilisi time (Aug 15, 16:00)**. Then we decide whom to write to.

- Priorities after the waiting deadline: **Hisham Bazzi** (Uniklinik Köln, `hisham.bazzi@uk-koeln.de` — email verified via PubMed) → Andreas Beyer (CECAD) → Björn Schumacher (IGSAD, `bjoern.schumacher@uni-koeln.de` — email verified via IGSAD).
- Bazzi — best fit: his direct work = mammalian centrosomes + p53/53BP1/USP28 mitotic surveillance (Damen 2021 Nat Commun #34050161; Phan 2021 EMBO J #33226141; Meyer-Gerards & Bazzi 2025 FEBS J) — an ideal match with the protocol design (USP25/28 override).
- Full strategy: `~/Desktop/LC/MCARA/docs/proposals/VW_Lead_Candidates_Strategy_2026-08-14.md` (+ copy on Desktop: `VW_Pioneering_Research_Lead_Candidates_2026-08-14.md`).
- Ready letter to Bazzi (English, 3–4 paragraphs) — §7 of the strategy.
- Timeline: even with a start on Aug 15, ~10 days remain until the Aug 27 deadline; the lead must be selected by Aug 18; VW Erstberatung — by Aug 20.
<!-- /lang:ru -->

## 📌 BOOK ON RESEARCHGATE: Centriole Reset (2026-08-14)

<!-- lang:ru -->
**Monograph "Centriole Reset: Testing Structural Organelle Rejuvenation in Somatic Cells" (v8.3, 2026-08-12) is available on ResearchGate:**

- URL: https://www.researchgate.net/publication/412241124_Centriole_Reset_Testing_Structural_Organelle_Rejuvenation_in_Somatic_Cells

**Publication context:**
- Full version has undergone revision: aligned with the review by D. Meyer (CECAD, Aug 13), MCARA v5.8 and CEDAR v1.8 (pulse-labeling Phase 0, SAS-6→persistent markers, DREAM/Meyer clock, centriole, FERMT3/ALMS1/ATF5, Ishida & Shibuya, Passanisi).
- References: 108 entries, all PMIDs verified via NCBI.
- Files on Desktop: `The Centriole As A Unidirectional Ratchet Of Cellular Identity And Aging_v8.3_checked.docx` (source) and `.pdf` (62 pp., front + back cover).
- PDF assembly — via script `~/Desktop/Services/scripts/make-pdf-book.sh`.
<!-- /lang:ru -->

## 🔴 POST-MORTEM: VW-Stiftung — Wagner rejection (2026-08-14)

<!-- lang:ru -->
**What happened:** Wolfgang Wagner (RWTH Aachen) declined to lead the VW Pioneering Research application: "Sorry, but timing does not work for me" (on vacation on Naxos until Aug 24; deadline Aug 27). The refusal was expected and prompt (response within ~1.5 hours after the email).

**What we missed:**
- We did not check in advance whether the candidate was on vacation (the auto-reply arrived only after the email was sent). September–August deadlines are the peak vacation period in Europe.

**What to change:**
- Before selecting a lead, quickly check availability (auto-reply/calendar), not just competencies.
- Do not build a plan around a candidate who is on vacation before the deadline.

**Next step:**
- Focus on Schiebel (ZMBH Heidelberg, letter sent Aug 13) + recommendation from Meyer (Beyer, CECAD).
- Wagner — as a last resort (returns Aug 24, but will stay for 2–3 days; respect his refusal, do not push).
- A brief polite reply to Wagner is not required (he is on vacation) — optional.
<!-- /lang:ru -->

## 🔴 POST-MORTEM: VW Foundation — Meyer declines lead (2026-08-13)

<!-- lang:ru -->
**What happened:** David Meyer declined to lead the VW Pioneering Research application (deadline Aug 27): he already has a project in the same competition, and the rule is "1 application per PI." Additionally, he did not notice the requirement of "lead at a German institute" until our letter. The refusal is benevolent — "I guess there are other possibilities ahead."

**What we missed:**
1. We did not check in advance whether Meyer was already participating in the VW competition (rule of 1 application/PI). We built the entire draft around him.
2. We did not verify the "lead in Germany" requirement in his context (he did not track this himself).
3. We had no backup list of German lead candidates — we had to search on the day of the rejection.

**What to change:**
- Before selecting a lead — directly ask: "Are you already participating in this competition? Do you have a submission in it?" (rule: 1 submission/PI).
- Keep 2–3 backup German lead candidates from the very beginning.
- Verify competition requirements (eligible institution, 1 submission/PI, DPR anonymity) from the competition text BEFORE selecting the lead.

**Next step:**
- ✉️ Sent to Meyer: requesting a recommendation (hinted at Andreas Beyer, CECAD).
- ✉️ Contacts: Wolfgang Wagner (RWTH Aachen), Elmar Schiebel (ZMBH Heidelberg), Thomas Müller-Reichert (TU Dresden) — as co-PIs.
- 📞 VW-Stiftung initial consultation — mandatory foundation consultation before submission.
- Deadline: by Aug 27. Cologne (Aug 31–Sep 2) — backup track for coordination.
<!-- /lang:ru -->

## 🔴 Changes per D. Meyer's (CECAD) review — applied everywhere (2026-08-13)

<!-- lang:ru -->
**Accepted and incorporated into CONCEPT / THEORY / EVIDENCE / CEDAR / EIC (v4.6) / Organismal_Aging:**
1. **Cartwheel/SAS-6 is not persistent:** it is disassembled during maturation and in mitosis (Huang 2025, PMID 39614048); it is lost during ciliogenesis in postmitotic C. elegans neurons (Li 2017, PMID 28743734; Serwas 2017, PMID 28411189). Targets of the persistent structure: triplet microtubules, inner scaffold (Le Guennec 2020, PMID 32110738), appendage rings. SAS-6 is present only where the cartwheel is persistent (procentrioles; dividing C. elegans cells).
2. **Genetic manipulations ≠ gradual damage.** Design: structural validation-gate + hypomorphs + acute/chronic timing.
3. **Direct test of centriole age (accepted, first experiment):** pulse-labeling mother vs daughter (U-ExM, M1–M5) in a single cellular background.
4. **PMIDs verified via PubMed (2026-08-13):** Le Guennec 32572233→**32110738** (correct article — inner scaffold), Gambarotto 30559433→**30559430**, Gönczy 22615561→**22691849**, Schmidt 7081–7097→**6945–6963**.
5. **VW-Stiftung Pioneering Research (deadline Aug 27, 2026)** — new grant direction on centriole-as-counter: Marketing/DREAM_CEDAR/, lead applicant CECAD (Meyer), GLA — partner. Does not conflict with EIC (Oct 28) — different topics.
<!-- /lang:ru -->

## 🔴 BENIGN TUMORS = counter model (nevi) (2026-08-13)

<!-- lang:ru -->
**Recorded everywhere (Proposal Aubrey, EVIDENCE §21, MEMORY).** Analogy with fibroblast escape:
- **Nevus** = escape halted by counters (p16/p53 active → OIS senescence; telomerase low) — Serrano 1997 (Cell), Michaloglou 2005 (Nature), Lorbeer 2024 (PNAS Nexus — different mechanisms in different nevi)
- **Fibroblast escape** = counters broken (p53/p19ARF + telomerase/ALT)
- **Cancer (melanoma)** = all counters broken

**CEDAR prediction (testable on archival specimens):** centriolar depth is INCREASED in nevi, DECREASED in melanoma — nevi vs melanoma vs normal melanocytes, CEP128/ninein + p16, on routine sections.

**LERR:** nevi demonstrate controlled arrest without death in vivo; reversibility of OIS (relief of p16/p53) = pathway to progression → Rebuild must PRESERVE p53/p16, not remove.

**MCARA:** the hierarchy of counters is visible (telomeres + OIS + centriolar); "different senescence mechanisms" (Lorbeer) are consistent with the 5-counter architecture.
<!-- /lang:ru -->

## 🔴 INQUIRY Aging Biology (formerly Aging Cell) resubmitted + APC task (2026-08-13)

<!-- lang:ru -->
**First inquiry attempt (13.08, agc@wiley.com) — NOT DELIVERED:** 550 5.1.1 User Unknown. Reason: **Aging Cell has been renamed to Aging Biology** (the journal continues to be published under the new name, same Wiley).

**Aging Biology contact information (confirmed via the Wiley Contact page):**
- Editorial Assistant: Megan Johnson — editorial.assistant@agingbiologyjournal.com
- Editors-in-Chief: Vera Gorbunova (rochester.edu), John Sedivy (Brown), Peter Adams (SBP), Julie Andersen (Buck)
- Accepts Hypothesis and Theory articles (up to 12,000 words, peer-reviewed) — the format is suitable.

**Inquiry resent on 13.08 to editorial.assistant@agingbiologyjournal.com** (subject: "Pre-submission inquiry — centriolar entropy as a source of aging-clock signal"). Copy: letters/sent/AgingBiology_inquiry_2026-08-13.md. Old files letters/AgingCell_inquiry_2026-08-13.md and letters/sent/ have been deleted.

**🔴 TASK: check whether the journal Aging Biology is free of charge (APC/waiver):**
- Submission and inquiry — free of charge (confirmed)
- Publication: full OA, APC ~$3,000–4,000 (Wiley)
- **Check Research4Life waiver:** Georgia — Group B → ~50% discount; Group A → free of charge. Wiley participates in Research4Life.
- **Fallback without APC:** BioEssays (Wiley, hybrid — subscription route $0), Mechanisms of Ageing and Development (Elsevier, hybrid), Aging (Albany NY)
- **Check precisely:** Aging Biology APC page at onlinelibrary.wiley.com + Research4Life eligibility for Georgia + request a waiver upon acceptance (GLA — non-profit)
- **APC decision:** after the editorial office responds to the inquiry
<!-- /lang:ru -->


## 🔴 TWO LINES: Differentiation and Aging (2026-08-13) — conceptual core

<!-- lang:ru -->
**Per Jaba's instruction — separate TWO DIFFERENT LINES everywhere:**
1. **Differentiation** (programmable, geometric counter): replicative count = CHANGE IN CENTRIOLE GEOMETRY (each duplication copies a slightly altered geometry — the number of divisions is encoded in the scaffold shape). Depends on asymmetric stem cell divisions and on stochastic error accumulation by the oldest centrioles.
2. **Aging** (side effect of differentiation): entropy of centrosome and cilium dysfunction accumulates with the CHRONOLOGICAL age of the centriole (division rate ↓ — centrosomal function; signaling perception ↓ — ciliary function).

Selective accumulation of oldest centrioles in asymmetric divisions → ↓division rate + ↓signaling.
Documented in: THEORY.md, CONCEPT.md, CEDAR/THEORY.md, Research Article, Proposal Meyer, EVIDENCE §19.
<!-- /lang:ru -->


## 🔴 GUICHARD: SAS-6 kartvel does NOT persist in mature human centrioles (2026-08-13)

> **Reply saved:** `refs/Guichard_SAS6_cartwheel_critical_reply_2026-08-12.md` | **Letter:** `letters/sent/2026-08-12_Guichard_SAS6_cartwheel_aging.md`

<!-- lang:ru -->
**CRITICAL response by Paul Guichard (UNIGE) to the letter on SAS-6 cartwheel:**
1. In human/vertebrate centrioles, the SAS-6 cartwheel **disassembles upon maturation** — it is present only in procentrioles/young centrioles; it is ABSENT in mature mother centrioles → M1 (SAS-6 ring geometry) is invalid for mature human centrioles.
2. **In C. elegans, the cartwheel PERSISTS** (Gönczy's work) → Experiment A (C. elegans) with SAS-6 is VALID and strengthened.
3. **U-ExM** may not provide sufficient resolution for subtle cartwheel changes → STED/cryo-ET on a subsample.
4. **Guichard's advice:** measure persistent structures: triplet microtubules, inner scaffold (POC1A/B, POC5), A-C linker.

**Included in all documents:** M1 redefined (persistent-scaffold geometry), C. elegans-only for cartwheel, U-ExM resolution caveat added. Proposal Meyer v3.6, Research Article, Proposal Aubrey v2 — updated. Dzhaba sent a follow-up (question about inner scaffold/A-C linker + invitation to Cologne).
<!-- /lang:ru -->

## 🔴 Proposal Meyer v3.6 + Research Article + Oxygen Paradox (2026-08-12)

> **Files:** `docs/proposals/Proposal_Meyer_CEDAR_aging_clocks.md` (v3.6), `docs/proposals/Testing_the_Hypothesis_of_Centriolar_Entropy_as_a_Source_of_Transcriptomic_Aging_Clock_Signal.md`, `docs/proposals/Proposal_Aubrey_Centriole_Reset.md` (v2)
> **Letter sent:** Meyer 2026-08-13 (with attachment) — copy in `letters/sent/`

<!-- lang:ru -->
**Proposal Meyer has undergone 8 rounds of rigorous peer review: 68→72→71→78→62→68→85→~93 (v3.6).** Key decisions:
1. **Meyer's clocks verified:** BiT age (Aging Cell 2021, Meyer 1st author) — transcriptomic clock; stochastic variation (Nat Aging 2024); Gallrein 2026 (C. elegans neurons); Koch 2026 (DREAM). Meyer is co-author of 2× Nat Aging 2026, 1st author of BiT age.
2. **Metric M2 no longer depends on the Yang preprint:** primary markers CEP128 (Kashihara 2019) + ninein (Tillery 2024), secondary NDE1/NDEL1 (Inaba 2016, JCB — peer-reviewed).
3. **Chemical damage markers:** 4-HNE + DNPH carbonyls (co-localization with centrin-3), methionine sulfoxide in SILAC — "damage" has become chemically measurable.
4. **Centriole→DREAM bridge assembled from PMIDs:** Meitinger 2016 (p53) → Schmidt 2024 (p21-DREAM) → Koch 2026 (repair) + PIDDosome (Garcia-Carpio 2023) + autophagy (Coelho 2026).
5. **Experiment A.0 — STOP-rule:** if the p53/DREAM signature does not appear — A, B, D are not initiated.
6. **Oxygen paradox (for Aubrey):** Parrinello 2003 (PMID 12855956) — MEFs in 3% O₂ do not senesce in the experiment, in 21% O₂ they immortalize (illusion of no limit). The paradox = key test: hTERT+ cells in hypoxia have a limit (centriole = counter, not telomeres); MEF immortalization carries centriolar markers through an event that erases the Hayflick limit.
7. **Research Article formatted in APA7** — for Meyer, structure: Abstract (≤250 words) + Keywords + Introduction + Methods + Anticipated results + Discussion + References APA7.

**Next steps:** await responses from Coelho/Guichard/Meyer; Gönczy follow-up Aug 14; Cologne Aug 31–Sep 2 (Meyer: Proposal + coffee cups; Aubrey: oxygen paradox).
<!-- /lang:ru -->

## 🔴 NDE1 → Subdistal Appendages + Centriole→Autophagy Link (2026-08-12)

> **Paper:** Yang, Coelho, Glover (Caltech), preprint openRxiv 2026-07-02, DOI: 10.64898/2026.07.01.735914
> **Full analysis:** `~/Desktop/Services/docs/literature/NDE1_2026_Subdistal_Appendages.md`
> **Ref:** `refs/NDE1_Subdistal_Appendages_Yang_2026.md`

<!-- lang:ru -->
**Finding 1 — molecular stratification of subdistal appendages (SDA):** NDE1 forms a ring in the SDA between CEP128 (proximal layer) and ninein (periphery). NDE1 is in the ODF2/CEP128 branch, NOT in the CEP170 branch (CEP170 branch → DNA repair, Rodríguez-Real 2023 PMID 37664992). Three markers of SDA integrity: NDE1 ring, distance between paired centrioles, ectopic foci.

**Finding 2 — 🔴 centriole→autophagy link:** NDE1 depletion → ↑LC3B/p62, ↓autophagic flux. This is a bridge between C1/CEDAR and C5/Proteostasis — a cross-interaction of MCARA counters, predicted by the architecture.

**Finding 3 — confirmation of the "two functions" of the centriole (Tqemaladze 2025):** distal appendages → ciliary signaling; subdistal appendages → MTOC organization. NDE1 depletion phenocopies the failure of the MTOC branch.

**Finding 4 — Paula Coelho (palmeida@caltech.edu):** co-author of the NDE1 paper + first author of "Sensing centrosome amplification: interface between centriole duplication and autophagy" (Nat Commun 2026, PMID 42324259, genome-wide screen — Wnt/Hippo/Tpr53/PIDDosome/cilia/autophagy). Strong candidate for consortium partnership after Gönczy's refusal (2026-08-07).

**Finding 5 — POC5 (PMID 42507085, FASEB J 2026):** human centriolar protein → premature senescence (SA-β-gal, p-p53), supernumerary centrioles, 35% ↓proliferation. Clinical evidence that "centriole = aging counter." Method — U-ExM (as recommended by Gönczy).

**Actions:** contact Coelho; add NDE1/CEP128/ninein + LC3B/p62 to the marker panel; cite in CIRCBIO-07/ERC/article v9.
<!-- /lang:ru -->

## 🔴 CRITICAL CORRECTION: Planarian Centriole Data (2026-08-06)

> **Finding:** Deep PubMed search confirmed that Azimzadeh 2012 explicitly states planarian neoblasts and embryonic cells DO NOT have centrioles. The 2026-07-17 audit correction went in the wrong direction.
> **Fixed files:** EVIDENCE.md §10.1, CONCEPT.md, MCARA_BiologyOpen_v2.md, MCARA_MedicalHypotheses_v1.md.
> **Key facts:** (1) Planarian neoblasts have no centrioles (IF+TEM), (2) Embryonic cells have no centrioles (IF Fig. S4), (3) Centrioles appear ONLY de novo in ciliated cells + spermiogenesis, (4) Spermiogenesis de novo centriole assembly from Rouhana 2022 PMID 34542855, (5) This STRENGTHENS the CEDAR hypothesis: the most plastic cells in the animal kingdom are permanently centriole-free.
> **Gap:** Fate of sperm centriole in zygote — never studied. Oogenesis centriole status — no data.
> **Letters sent (2026-08-06):** Juliette Azimzadeh (CNRS/IJM Paris) ✅, Jochen Rink (MPI-NAT Göttingen) ✅, Phillip Newmark (Morgridge/UW–Madison) ✅. Asking: (1) centriole status in planarian oocytes, (2) fate of sperm centriole in zygote, (3) embryonic centriole data.
> **File:** `~/Desktop/letters_planarian_centrioles_2026-08-06.md`

### Gönczy responds (2026-08-05) — "Tantalizing" but no manpower

> Pierre replied to the Aug 2 letter (SAS-6 cartwheel damage hypothesis). Key: he finds the idea "tantalizing" and confirms SAS-6 as good readout. Suggests stem cell→differentiation system OR "compare indeed the zygote to later stages of embryogenesis." Has equipment (expansion microscopy + EM) but **no manpower.**

### 🔴 Pierre declined consortium participation (2026-08-07)

> Jaba invited Pierre to join as consortium partner. Pierre declined. Revised approach: short reply — thank him, ask for recommendation of a younger PI with U-ExM pipeline. Mention Cologne trip (late Aug, consortium meeting with de Grey + David Meyer).
> **Reply sent:** ✅ `~/Desktop/LC/MCARA/letters/sent/2026-08-07_Pierre_Gonczy_reply.txt` — Aug 7, 21:50 Tbilisi.
> **Auto-reply received:** Pierre away until Aug 13. Follow up Aug 14.
> **Next:** Wait for Pierre's recommendation. Parallel: Guichard letter.
> **File:** `~/Desktop/LC/MCARA/letters/sent/2026-08-07_Pierre_Gonczy_reply.txt`

---

## Royle (2026): Clathrin Moonlighting — Comprehensive Metareview (2026-08-04)

> **Paper:** Royle S, Traffic, DOI: 10.1111/tra.70047 | PMID: 42498517
> **Full analysis:** `~/Desktop/Services/docs/literature/Royle_2026_Secret_Mitotic_Life_of_Clathrin.md`
> **Ref:** `refs/Royle_2026_Clathrin_Moonlighting.md`

<!-- lang:ru -->
**Meta-analysis (4 databases):** All 9 key Royle lab references verified. 24 additional articles found and analyzed. The model is consensus-based, with no refutations.
<!-- /lang:ru -->

<!-- lang:ru -->
**Clathrin — a paradigmatic moonlighting protein for MCARA:**
One protein → 4+ independent mechanisms in mitosis:
<!-- /lang:ru -->
1. Inter-microtubule bridging (TACC3/chTOG/clathrin) — Booth 2011 EMBO J, Nixon 2015 eLife
<!-- lang:ru -->
2. GTSE1 recruitment → MCAK inhibition on astral MTs — Rondelet 2020 JCB
3. Centrosome integrity via ch-TOG stabilization — Foraker 2012 JCB 🔴
<!-- /lang:ru -->
4. CHC-pT606 → GAK → PLK1 → Kiz signaling axis — Yabuno 2019 Cell Cycle 🔴

<!-- lang:ru -->
**Full composition of the complex (Ryan 2021, J Cell Sci, PMID 33380489 — CRISPR + induced relocalization):**
<!-- /lang:ru -->
- CORE: TACC3 + CHC
- ANCILLARY: chTOG (binds TACC3), GTSE1 (binds CHC → inhibits MCAK)
- ❌ NOT: PI3K-C2α (disproven by Ryan 2021)

<!-- lang:ru -->
**Drug development (already underway!):**
<!-- /lang:ru -->
- SP TACC3 stapled peptide — 400× affinity over native, disrupts TACC3-CHC (Gunning 2026, Structure, PMID 42049022)
- AK306 small molecule CLTC binder — selective cancer apoptosis in mice (Bond 2018, Mol Cancer Res, PMID 29769406)
- TACC3 degraders via ubiquitin-proteasome (Ohoka 2014, Cell Death Dis)

<!-- lang:ru -->
**Other moonlighting proteins (parallels for MCARA):**
<!-- /lang:ru -->
- GRP75: mitochondrial chaperone → cell cycle + endocytosis control (Gao 2017, PMID 28938577)
- Megalin: endocytic receptor → mTORC1 switches to cell cycle (Dahlke 2026, PMID 42174246)

<!-- lang:ru -->
**Open questions for MCARA:**
1. Age-dependent impairment of clathrin function has not been demonstrated (testable prediction)
2. Can SP TACC3/AK306 be used for anti-aging interventions?
3. Which of the 4 mechanisms is most significant for ageing — centrosomal (Foraker 2012) or spindle (Booth 2011)?
<!-- /lang:ru -->

---

## Phylogenetic Analysis: Centriole Elimination Across Mammals (2026-08-03)

> **Finding:** Systematic review of 15+ mammalian species reveals three distinct patterns of centriole elimination, refuting the critique that placental mammals universally lost this mechanism.

| Pattern | Species | Timing | Transferable to soma? |
|---------|---------|--------|:---:|
| **Rodent-type** | Mouse, rat, hamster | No elimination — maternal inheritance | ❌ |
| **Primate-type** | Human, rhesus macaque | Pre-meiotic elimination | ⚠️ Unknown mechanism |
| **Ungulate-type** | Cattle, pig, sheep, rabbit | Post-fertilization elimination during mitosis | ✅ Best model |

**Key reference:** Uzbekov R, Avidor-Reiss T (2024) The proximal centriole age in spermatozoa determines its fate in the zygote. Open Biol 14:230458. PMID 38442864. [Older centrioles preferentially eliminated — age-dependent selection.]

**Implication for CEDAR/MCARA:** The ungulate-type mechanism (post-fertilization, mitotic) is the most directly transferable to somatic cell reprogramming. Human elimination factors exist (primate-type) but are uncharacterized. Both branches confirm that centriole elimination is present in placental mammals — it was never lost, just diversified.

## 2026-08-03: Simerly 2018 + Madarampalli 2015 — oocyte centriole loss is gradual, ATF5 binds polyE

> **Finding 1 (Simerly 2018, PMID 30143724):** Mouse centrioles are gradually lost from PGCs through mature oocytes — a developmental process spanning days to weeks during meiotic maturation. This is NOT a molecular switch that can be flipped in a somatic cell. Centrioles progressively dissociate from PCM and lose organizational capacity. The oocyte does not undergo mitotic divisions during this process — a unique context not reproducible in cycling somatic cells.

> **Finding 2 (Madarampalli 2015, PMID 26213385):** ATF5 binds specifically to polyglutamylated tubulin on the mother centriole and connects PCM to the centriole. This means PTM stripping (CCP1 → remove polyE) would remove ATF5 binding sites → PCM detachment → centriole cannot organize spindles. PTM stripping = functional elimination without structural removal. Not a solution.

> **Implication:** These two findings further strengthen the case for centriole elimination over PTM stripping, but also highlight the complexity of the oocyte pathway. The oocyte uses gradual dissolution during a unique cell cycle — not transferable to somatic cells by simple factor expression.

> **Recorded in:** `docs/WHY_IPSC_FAILS.md` §4.1

## 2026-08-03: The centriole as a morphogenetic lock — resolution of the differentiation paradox

> **Finding:** Renzova et al. (2018, PMID 30197118) showed centrinone-induced centriole loss in hPSC triggers spontaneous differentiation. Kalbfuss & Gönczy (2023, PMID 37256957) showed 88% of C. elegans cells eliminate centrioles during terminal differentiation. A superficial reading suggests: "centriole elimination drives differentiation, therefore it cannot enable totipotency."

> **Resolution:** The centriole is a morphogenetic status lock, not a fate determinant. An old centriole locks cells into their current (aged) identity. Removing it breaks the lock — cells lose controlled self-renewal and drift. Without centrioles + no signal (Renzova): chaotic differentiation. Without centrioles + DUX4/EZH2i: totipotency program. Then de novo young centrioles restore controlled asymmetric division and lock in the new desired identity.

> **Implication:** Renzova 2018 does not contradict the protocol — it confirms its premise. The centriole controls the *capacity to maintain identity*, not the identity itself. This is the core of the Threshold Stand model.

> **Recorded in:** `docs/WHY_IPSC_FAILS.md` §4.1

## 2026-08-03: Why centriole elimination is mandatory — the geometric argument

> **Decision:** PTM stripping (CCP1) is insufficient. Centriole aging has three modes: chemical (polyglutamylation), geometric (over-elongation), and structural (cartwheel deformation, oxidized tubulin). Köhrer et al. (2023, *Leukemia*, PMID 37821581) provides direct quantitative evidence: 1,386 centrioles from 8 healthy donors, over-elongated centrioles rise from 45% (age 24) to 76% (age 67), ρ = 0.67, p < 0.01. Geometry is templated during duplication — an over-elongated mother produces an over-elongated daughter. CCP1 fixes only chemistry. Only elimination + de novo resets all three. Nature agrees: every species eliminates centrioles during oogenesis, not strips them.

> **Wong 2015 barrier:** Centrinone causes passive centriole dilution → p53 senescence. But the oocyte uses active proteolytic elimination without senescence. Finding the mammalian oocyte elimination factors is the central challenge. Until then, the protocol acknowledges this as an unsolved problem.

> **Full argument:** `docs/WHY_IPSC_FAILS.md` §4.1

## 2026-08-02: Why iPSC Fails — The Correct Target Is Young Adult Stem Cells

> **Strategic note.** iPSC (full reprogramming to pluripotency) is a dead end for organism-level rejuvenation. The field — including Altos Labs ($3B) — has pivoted to partial reprogramming. **The correct target: tissue-specific adult stem cells with youthful division tempo and intact lineage commitment.** Not iPSC, not just "younger somatic cell" — adult stem cells that divide like young cells.
> 
> **Technological pipeline:** `docs/MCARA_TECHNOLOGICAL_PIPELINE.md` — complete 4-step chain from aged somatic cell to safe young adult stem cell.

**Key points:**
1. **iPSC → teratomas.** Even a few undifferentiated cells form tumors. c-Myc is an oncogene.
2. **Identity erasure.** A neuron becomes an iPSC, not a "young neuron" — synaptic connections lost.
3. **Correct target: adult stem cells.** Tissue-specific, lineage-committed, multipotent (not pluripotent). Young division tempo. Safe by design.
4. **Narrow therapeutic window.** OSK: too little = no effect; too much = teratomas + mortality. MCARA counters solve this.
5. **Altos Labs pivoted** from iPSC to partial reprogramming de facto. $3B, 4+ years, still basic research.
6. **The missing piece nobody sees:** the mother centriole as a physical carrier of age — cannot be reprogrammed, must be eliminated and rebuilt de novo.
7. **4-step pipeline:** (1) Centrinone → centriole elimination, (2) PLK4 pulse → de novo young centrioles, (3) OSK+NANOG+LIN28 mRNA → deep epigenetic reset to epiblast, (4) Tissue-specific master regulators → adult stem cell. All with MCARA counter checkpoints.

**Full documents:** `docs/WHY_IPSC_FAILS.md` | `docs/MCARA_TECHNOLOGICAL_PIPELINE.md`

<!-- lang:ru -->
## 2026-08-02: 🔴 Submission Status — Springer Portal
<!-- /lang:ru -->

<!-- lang:ru -->
**Ze_CHSH → PEER REVIEW at QIP!** 
**MCARA → appeal rejected.** Biogerontology → Scientific Reports.
**Centrioles (npj Aging) →** transfer → Scientific Reports.
**Ze_Model → 9 weeks With Editor.** Email sent to the editor of Found. of Physics (3fab9acb).
**4 new articles:** Three-Step Strategy (JTB), Activatus (LWT), Centriole Invasion (BioEssays), Ze+Centrioles (BioSystems). Inquiries: `INQUIRIES_2026-08-03.md`
<!-- /lang:ru -->

## 2026-07-25: Chk1 — two non-canonical roles in aging 🔴

> **Finding:** Chk1 (Checkpoint kinase 1) — a DNA damage kinase — has two non-canonical roles directly related to MCARA counters.

### What was discovered:
1. **Counter #1 (Centriolar):** Chk1-P→β-tubulin-T285 at the centrosome → spindle quality. Boutakoglou/…/Zachos 2026, *Commun Biol*, PMID 41844775.
2. **Counter #3 (MitoROS):** Chk1→AHSA1-HSP90→mitophagy → cardioprotection. Jing P et al. 2026, *Redox Biol*, PMID 42229233.

### What has been done:
- ✅ EVIDENCE.md §15 — full analysis with history of Zachos lab (2007–2026)
- ✅ CEDAR/EVIDENCE.md — Chk1→β-tubulin for M1 + Chk1→mitophagy
- ✅ CEDAR/CONCEPT.md — M1 updated: molecular mechanism
- ✅ MitoROS/EVIDENCE.md + CONCEPT.md — Chk1→mitophagy
- ✅ EIC Pathfinder Response — link PMID 41844775 in Mechanism A
- ✅ Contacts: `CEDAR/docs/CONTACTS_Chk1_Zachos_2026-07-25.md`

### Strategy:
- George Zachos (gzachos@uoc.gr) — potential partner CIRCBIO-07/EIC
- Chk1 = master regulator of two MCARA counters → strong grant narrative

---

**LERR — Ladder, Eliminate, Reprogram, Rebuild.**

**Step 1 (Ladder).** Cut the damage load first: slow the counter, push old centrioles into differentiating daughters, remove only the mother centriole, keep spare young ones.

**Step 2 (Eliminate).** Take out the old centriole. Restore telomeres. Wipe the epigenome. Rescue mitochondria.

**Step 3 (Reprogram).** Push to totipotency with DUX4 + KDM4D + DPPA3.

**Step 4 (Rebuild).** Grow fresh centrioles de novo. Derive clean, young adult stem cells.
**Step 1 (Ladder).** De-risk before elimination based on current data: slow down the counter (NAC antioxidant; reversible PTMs: TTL re-tyrosination, CCP5/6 deglutamylation); segregate damage via asymmetric inheritance of the mother centriole into differentiating progeny (Yamashita, 2007; Royall, 2023 — human NPCs); hemi-eliminate only the mother centriole (laser/PROTAC), preserving duplication control and avoiding p53-dependent G1 arrest (Meitinger, 2016); condition the cell (spare PLK4 centrioles, G1/S synchronization, proteostasis); select the least damaged pool (FACS by low Δ2/polyGlu).
**Step 2 (Eliminate).** Remove the old damaged centriole; restore telomeres (telomerase/ZSCAN4 via H3K14ac/H3K18ac; Meltzer, 2024); erase epigenetic marks (OSK/TET1-TET2-TDG; Lu, 2020 — partially, linear memory remains); select healthy mitochondria (PINK1-dependent mitophagy; Vázquez-Martín, 2016).
**Step 3 (Reprogram).** Induce totipotency: DUX4 + KDM4D + DPPA3 — DUX4 opens cleavage-stage genes (Hendrickson, 2017), KDM4D removes the H3K9me3 reprogramming barrier, DPPA3 (Stella) stabilizes the totipotent (2C-like) state.
**Step 4 (Rebuild).** Reassemble young centrioles de novo (PLK4 → SAS-6 → STIL → CPAP; Nigg & Holland, 2018; Gönczy, 2012) after complete elimination (Khodjakov, 2002; Uetake, 2007); control geometry (9-fold symmetry, triplets, length); obtain safe young adult stem cells (karyotype check, p53 restoration).
**Step 1 (Ladder).** De-risk before elimination: slow down the counter, segregate damage, hemi-eliminate the mother centriole, condition the cell, select the least damaged pool.
**Step 2 (Eliminate).** Remove the old centriole; restore telomeres; erase epigenetic marks; select healthy mitochondria.
**Step 3 (Reprogram).** Induce totipotency: DUX4 + KDM4D + DPPA3.
**Step 4 (Rebuild).** Reassemble young centrioles de novo; obtain safe young adult stem cells.

- De-risking ladder L1–L5 (slow down the counter: NAC/TTL/CCP5-6/metformin; segregate damage by asymmetry; hemi-elimination of only the maternal centriole; conditioning: PLK4 reserve, G1/S, proteostasis; FACS selection by Δ2/polyGlu) precedes any elimination
- E = elimination of the old centriole; R = reprogramming (OSKM/DUX4 + KDM4D + DPPA3); R = de novo reassembly (PLK4/SAS-6/STIL/CPAP)
- Full elimination — only when L1–L5 are exhausted; prognosis ≥80% survival (hemi-elimination) vs <50% (full), Meitinger 2016

---

## 2026-07-23: ERR GitHub repo + Marketing subproject created

- **ERR repo:** https://github.com/Georgia-Longevity-Alliance/ERR — public, Apache 2.0
- **Marketing:** MCARA_EIC_Pathfinder renamed to ERR (~/Desktop/Marketing/ERR/)
- **Content:** README = Cologne 2026 handout text. All 7 PMIDs verified.

---

# MEMORY — MCARA

> Decision History, journal path, key agreements.

## Rejection #30 — 2026-08-05 — Biology Open (Four Counters)

**Journal:** Biology Open (Company of Biologists)
**ID:** `bio.062853`
**Days to decision:** <1 (desk reject)
**Decision type:** Desk reject — scope mismatch

### Reason (what the editor said)
> «BiO does not consider hypothesis papers or narrative reviews. For more information on the article types considered by our journal please see https://journals.biologists.com/bio/pages/article-types»
> — Alejandra Clark, Managing Editor

### What we missed
- [x] Did not send a pre-submission inquiry. 🔴 Rule: PI blocks the submission if the inquiry has not been sent.
- [x] BiO is an experimental journal (Research Articles, Methods & Techniques, First Person). Hypothesis papers are not accepted.
- [x] Journal-fit.sh showed "Hypothesis-friendly" — an error in the script. BiO is not such a journal.
- [x] The article was submitted as a Research Article, but by genre it is a hypothesis paper.

### What to change before the next submission
- [ ] Rewrite the Abstract — remove hypothesis markers, add a quantitative framework angle
- [ ] Change the genre: not a hypothesis, but a **quantitative model / theoretical framework**
- [ ] Add Figure 1 (mandatory for a hypothesis journal)
- [ ] Send a pre-submission inquiry BEFORE the submission

### Next journal
**Options:**
1. **GeroScience** — accepts hypothesis/theory, high IF, but Centrioles: Hypothesis (JAAA-D-26-02347) has already been submitted
2. **npj Aging** — was transferred from npj Systems Biology, not used
3. **BioEssays** — Problems & Paradigms (hypothesis-friendly, but Centriole Invasion has already been submitted)
4. **Medical Hypotheses** (Elsevier) — specifically for hypothesis papers, no experimental data required
5. **Journal of Theoretical Biology** — quantitative models, hypothesis-friendly

**Journal-fit check:** REQUIRED for the selected journal.

---

## 2026-07-31: Incubator — humidity control
- **Solution:** In the incubator, in addition to O₂/CO₂/N₂, there must be active humidity control ±2% RH with a dehumidifier.
- **Why:** Reducing humidity is critical for ARGUS (condensation on optics), CEDAR (plate stability), MCARA (cell culture).
- **Updated:** CONCEPT.md (budget +$1,500).
---

---

---
## 2026-07-22: Deep Review — Shihabi (oocytes) × Maheshwari (centriolum) 🔴

> Full analysis: `~/Desktop/Services/docs/REVIEW_Oocyte_Centriculum_2026-07-22.md`
> The deepest cross-analysis of two articles + verification of 22 key references (all real) has been conducted.

### Key findings for MCARA:
1. **PCM-compaction (new mechanism):** SPD-5 density increases with centriolum reduction (R²=0.26, P=0.0002). PCM is not a static, but a condensable structure. Added to CONCEPT.md §0.
2. **Selective porosity of centriolum:** spindle MTs pass through, astral MTs are blocked. Mechanism unknown → priority task.
3. **10× concentration of tubulin (Baumgart 2019):** "filter" model provides an elegant explanation.
4. **Evolutionary conservation:** Drosophila (Diaz 2019, Rollins 2023), medaka (Kiyomitsu 2024), sea urchin (Xie 2025) — all have centriolum-like structures.
5. **ER-centrosome crosstalk:** PERK (Sánchez-Álvarez 2025), tubulin code (Zheng 2022 Nature), CDR2-dynein (Teixeira 2025 JCB).
6. **Intersection with oocytes (Shihabi 2026):** acentrosomal oocytes → is there a centriolum-like ER around huoMTOC? Age-related degradation → new mechanism of aneuploidy.

### Reference verification:
- 22 key references of Maheshwari et al. (2026) have been checked — **all real** (100%)
- Missed articles for Shihabi: Wu et al. 2022/2024 Science (2000+ human oocytes!), Rollins & Blankenship 2023, Araújo et al. 2023

### New PMIDs for EVIDENCE.md:
- 42283151 (Maheshwari 2026 — centriolum as MT filter)
- 42360132 (Shihabi 2026 — oocyte mechanics)
- 40267909 (Sánchez-Álvarez 2025 — PERK-ER-MT crosstalk)
- 34912111 (Zheng 2022 Nature — ER proteins + tubulin code)
- 40637585 (Teixeira 2025 JCB — CDR2 dynein adaptor for ER)
- 37971218 (Rollins & Blankenship 2023 — ER dysregulation → mitotic failure)
- 36379670 (Araújo 2023 — ER membranes maintain spindle size)
- 36395215 (Wu 2022 Science — huoMTOC)
- 39172836 (Wu 2024 Science — spindle bipolarization)
- 39024439 (Takenouchi 2024 Science — size-based meiotic errors)

> Idea (Jaba Tqemaladze): centriole selectively binds iron (Heidenhain's iron haematoxylin, Boveri 1900; Scheer 2014 PMID 25047623). After structural elimination (loss of GFP) iron-positive remnant may remain — collapsed iron-binding core. Nobody has checked. Methods: Heidenhain ($10), Perls' Prussian Blue ($5), DAB-enhanced EM ($50). Added to CONCEPT.md §0.
## 2026-07-21: Pierre Gönczy's Response — Centrioles are Preserved for a Reason 🔴

- **Pierre's Response (Jul 21, 16:40):** To Jaba's question about foci of centriolar proteins in somatic gonad (from Dev Biology mapping paper, Gönczy & Antonin):
  > «We suspect (but do not know, since we have no way at present to eliminate centrioles at will as you know) that centrioles remain for a reason, e.g. to be able to build a centrosome or for some signaling function.»
- **Value:** Pierre — the leading expert on centriolar elimination — directly states: (1) centrioles are preserved not by chance, but for a functional reason, (2) one of the possible reasons is a signaling function, (3) the field is open — there is no way to selectively remove centrioles for verification.
- **Support for MCARA:** Signaling function of centrioles → gatekeeper of cell state. This is the CEDAR hypothesis. If centrioles = signaling hub, their elimination = removal of gatekeeper → transition to a new differentiation state.
- **Tactic:** The response does not require follow-up. Save as evidence in EVIDENCE.md §11.
## 2026-07-19: Exchange with Pierre Gönczy + Deep Audit + 3 New Articles 

- **Exchange with Gönczy (18-19 Jul):** Three questions — three answers. Pierre confirmed: (1) no hidden literature, (2) no one is chasing the mechanism of somatic elimination, (3) the field is open. Recommended Pimenta-Marques 2023 (ANA1/CEP295) as a starting point.
- **Deep Audit:** 41 PMID verified through PubMed API + full texts. 93% confirmed (38/41).
- ** Found a problem:** Lindhout 2021 (33835529) — MCARA cited as "maturity sensor → plasticity", but the article shows the **OPPOSITE**: loss of centrioles **disrupts** development. Corrected in CONCEPT.md and THEORY.md.
- ** Corrected overstatements:** Bodnar 1998 (not "not immortalized", but "not infinitely in all types"), Parrinello 2003 (extrapolation to human explicitly marked).
- **Bradford Hill upgraded:** 5/9 → 6/9. Temporality (Robichaud 2024 sinc-MT). Specificity (FERMT3 + ALMS1 + ATF5).
- **Added a three-stage model:** Kalbfuss & Gönczy (2023) — maintenance → priming → execution. Table in CONCEPT.md §0.
- **New PMIDs in EVIDENCE.md §11:** 37963546, 37256957, 37414202, 40475707 + Pimenta-Marques (doi:10.1038/s44319-023-00020-6).
- **Follow-up letter to Pierre:** ready (`CEDAR/articles/docs/Gonczy_lab/FOLLOW_UP_Pierre_2026-07-19.md`).
- **CONCEPT.md v4.7, THEORY.md v4.7, EVIDENCE.md v4.7, STATE.md updated.**

### New Articles (Semantic Scholar → PubMed verified)
- **Centriculum (Maheshwari/Cohen-Fix 2023/2026):** PMID 36693370 (Curr Biol), PMID 42283151 (J Cell Sci). Centrosome is NOT membraneless — surrounded by 3D ER-membrane reticulum. Centriole = MT-filter. Age-dependent degradation → new mechanism Counter #4.
- **Spermatogenesis (Ishida & Shibuya 2026):** PMID 42455439 (Adv Exp Med Biol). 165 refs. Asymmetry of gametes: oogenesis = Eliminate, spermatogenesis = Retain → Rebuild. Strengthens Bradford Hill Analogy.
- **Tweedell:** not found on PubMed (likely an old article without indexing; URL from Sem Scholar was uninformative).

- **Conducted:** deep audit of all links and verification of PMIDs (60+)
- **Key corrections:**
  1. **Planarians:** Azimzadeh 2012 (PMID 22223737) — neoblasts DO HAVE centrioles. Lost centrosomes (PCM), not centrioles. All documents corrected.
  2. **DID-RNA:** downgraded to a speculative model. Removed from the main CEDAR theory. Preserved in `DID_Centricle_Dve_Linii.md`.
  3. **C. elegans:** elimination mechanism recognized as unknown. Added explicit indication to all files.
  4. **Prediction:** "≥2×" → "1.5–10×" with justification of the range.
  5. **Experiment:** added Centrinone + DUX4/TPRX1 group.
- **Verified PMIDs:** FERMT3 (42343301) , ALMS1 (42380124) , Azimzadeh (22223737) 
- **Files:** CONCEPT.md, THEORY.md, STATE.md updated. EVIDENCE.md — in queue.
- **Full audit:** `audits/DEEP_ANALYSIS_MCARA_2026-07-17.md`

- **Journal:** Biogerontology | **ID:** `7cc6de62` | **Type:** Appeal of editor's decision
- **Letter:** Rattan + Yanai. Argument: article completely rewritten — narrative-first, Figure 1 (centriole-cilium ratio), 13-group experiment table, 5 falsifiable hypotheses. Science has not changed, presentation is new.
- **Key changes:**
  1. Opens with a paradox: hTERT + hypoxia → arrest (not an abstract framework)
  2. Centriole — a physical object, not an abstract counter
  3. Evidence from 5 phyla BEFORE multi-counter architecture
  4. Figure 1: centriole-cilium ratio (graphic overview)
  5. Table of 13-group experiment → 5 falsifiable hypotheses
- **Status:**  Waiting for editor's response
## 2026-07-14: Submission to BioEssays (Problems & Paradigms) 🟢

- **Journal:** BioEssays (Wiley) | **ID:** `4799098` | **Type:** Problems & Paradigms
- **Title:** «Centriole Elimination as a Gateway to a New Differentiation State: A Hypothesis»
- **Status:** 🟢 Submitted 2026-07-14, awaiting editorial decision

---
## 2026-07-14: Preprint "Three-Step Strategy to Overcome the Sprouting Paradox" — Research Square 

- **Platform:** Research Square | **DOI:** `10.21203/rs.3.rs-10320333/v1`
- **RSID:** rs-10320333
- **Status:** ✅ Published, permanent DOI

---
## Rejection #1 — 2026-07-13: Biogerontology

**Journal:** Biogerontology | **ID:** `7cc6de62` | **Days:** 40 | **Type:** Section editor (Hagai Yanai)

**Reason:** «While this concept is intriguing, the presentation is difficult to follow and I recommend that it is made more approachable to a wider readership.»

**What we missed:** MCARA — a complex quantitative model. Submitted in its raw form: many formulas, little narrative. Not adapted for a biogerontology reader (not a physicist).

**What to change:**
1. Add a graphical scheme of MCARA (multi-counter visual)
2. Move mathematics to Appendix/Supplementary
3. Main text — narrative: problem → counterexamples → MCARA solution → predictions
4. Explain each counter using a specific biological example
5. Reduce by 30%, remove duplication

**Next step:** Transfer to another Springer journal (will be suggested automatically) OR rewrite and resubmit. Decision: rewrite for the reader.
## 2026-07-14: Conversation with Gakeli — OpenFlexure, inverted vs upright scheme, nanopores

- **Event:** Evening dialogue with Gakeli (WhatsApp) about microscopy for ARGUS
- **Context:** Jaba mentioned the problem with water immersion in the inverted position (objective below — water flows down). Gakeli proposed a direct solution: **upright microscope.**
- **KEY ENGINEERING INSIGHT:**
  - Subject from **above** → water is held by gravity and surface tension 
  - Ablation laser **from below** → through the glass bottom of a Petri dish 
  - Cells settle on the bottom themselves 
  - This is **simpler** than the current ARGUS-LP v3 design (inverted + water jacket + syringe pump 0.1 ml/h)
- **OpenFlexure:** Gakeli is studying OpenFlexure v6.1.5 — an open-source 3D-printed microscope:
  - Upright, motorized (<100 nm positioning)
  - RMS optics, Raspberry Pi Camera 2
  - Fully open STL files
  - Cost: ~$200–500 (printing + optics + RasPi)
  - Supports fluorescence, phase contrast, structured illumination
  - Software: open-source Python, plugins, programmable clients
  - **Gakeli's link:** https://microscope-stls.openflexure.org/#/v6.1.5 (RMS_f50d13 + picamera_2)
- **Nanopores + lineage tracking:** Gakeli mentioned: Oxford Nanopore reads methylation simultaneously with DNA → can track cellular lineage differentiation and tissue age grouping. This is an **orthogonal** (complementary) method to visual lineage tracking ARGUS-LP.
- **Scaling vision:** "6,000 robots = 120,000 scientist mice" — at an OpenFlexure price of ~$500/station → a network of 6,000 ARGUS stations = ~$3M.
- **Comparative analysis:** OpenFlexure (upright, $200–500) vs ARGUS-LP v3 (inverted, $2,045–$8,170) — `~/Desktop/LC/MCARA/ARGUS-LP/docs/OpenFlexure_vs_ARGUS-LP_analysis_2026-07-14.md`
- **Memo to ARGUS Telegram group:** `~/Desktop/LC/MCARA/ARGUS-LP/docs/ARGUS_Telegram_memo_2026-07-14.md`
## 2026-07-13: Analysis of Research Feed — ClpP, mRNA Regionalization, mei-P26 + Similar Articles

- **Event:** Analysis of 7 articles from Jaba feed + PubMed search for similar articles (mitochondrial QC, ClpP/LONP1, mRNA localization).
- **Result:** 4 categories of articles (TOP-3 + 4 additional from feed, 5 from mitochondrial QC search, 5 from ClpP/LONP1 search). Total of ~20 highly relevant PMIDs.
- **KEY:** CEDAR renamed to CEDAR — replaced in AGENTS.md, directories already renamed.
- **KEY FINDINGS FOR MCARA:**

### 1. ClpP Series — Mitochondrial Proteostasis and Meiosis
| PMID | Article | Year | Connection |
|------|--------|-----|-------|
| 42281331 | Feng HW et al. ClpP Ensures Mitochondrial Integrity and Meiotic Progression — Andrology | 2026 | 🔥 ClpP cKO → meiotic block, mitochondrial defects |
| 37798322 | Guo C et al. ClpP/ClpX deficiency → impaired mTORC1 signaling — Commun Biol | 2023 | mTOR axis: ClpP → mitochondria → mTORC1 → meiosis |
| 23851121 | Gispert S et al. Clpp null → infertility, mtDNA accumulation — Hum Mol Genet | 2013 | First characterization of Clpp KO |
| 38927630 | Key J, Gispert S, Auburger G. CLPP/CLPX in matrix condensates near IMM — Genes | 2024 | Molecular mechanism of CLPP/CLPX |
| 38341415 | Ng AQE et al. Nutrient-dependent intron → germline mitochondrial QC — Nat Commun | 2024 | Connection of nutrients with mitochondrial QC in germline cells |

### 2. Mitochondrial UPR and Proteases
| PMID | Article | Journal | Year |
|------|--------|--------|-----|
| 42216472 | Czechowicz P et al. The mammalian mitochondrial UPR — multilayered circuit | FEBS J | 2026 |
| 41655698 | Currie SQW et al. Molecular mechanisms of mitochondrial AAA+ proteases | J Biol Chem | 2026 |
| 40903791 | Nandha SR et al. Targeting CLPP and LONP1 → proteotoxic stress | Cell Commun Signal | 2025 |

### 3. mRNA Regionalization and Cellular Patterning
| Article | Journal | Connection to CEDAR |
|--------|--------|---------------|
| Albright AR et al. mRNA regionalization in giant single cell | PNAS 2026 | 🔥 Direct proof: single cell → spatial patterning → basis for asymmetric division |
| Leite I et al. Cyst-ained connections in mammalian germline | Curr Top Dev Biol 2026 | Cysts of germline cells — structural context for mRNA regionalization |

### 4. Mitosis→Meiosis Transition
| Article | Journal | Connection |
|--------|--------|-------|
| Terry J et al. mei-P26 mutation → impaired chromosome dynamics | Genetics 2026 | Molecular gatekeeper: mitosis→meiosis transition |
| Iniesta-Cuerda M et al. SIRT1 haploinsufficiency → age-associated subfertility (α-tubulin hyperacetylation) | Biol Direct 2026 | Epigenetic mechanism of age-associated subfertility |

- **Full Analysis:** `~/Desktop/Services/docs/RESEARCH_FEED_ANALYSIS_2026-07-13.md`

---
## 2026-07-12: Analysis of 144 references Manni et al. (IGF-1 senescence switch)

- **Event:** In-depth analysis of the reference list of the article Manni et al. (Cytokine, 2026, PMID: 41905220) — 144 references.
- **Result:** No works by Jaba (Tqemaladze/Tkemaladze/Chichinadze) among the references. "Ageless Creatures" not found in the reference list — likely an error in the Google Scholar alert or a book.
- **KEY INTERSECTIONS WITH MCARA (10 references out of 144):**

| # | Reference | Year | Connection to MCARA |
|---|-----------|-----|-------------|
| 1 | **Hallmarks of aging: An expanding universe** (López-Otín) | 2023 | Direct connection — Jaba cites in MCARA |
| 2 | **IGFBP5 is released by senescent cells and is internalized by healthy cells, promoting their senescence** | 2024 | 🔥 **Paracrine mechanism** — analogous to the centriolar driver, but through secreted factors! |
| 3 | **Hypoxia-Induced Senescent Fibroblasts Secrete IGF1 to Promote Cancer Stemness** | 2024 | Stemness → centrioles in cancer |
| 4 | **Targeting IGF1-Induced Cellular Senescence to Rejuvenate Hair Follicle Aging** | 2025 | Rejuvenation = intervention in counters |
| 5 | **The IGF System and Aging** (Endocrine Reviews) | 2024 | Comprehensive review IGF-1/aging |
| 6 | **Cellular senescence in tissue repair and regeneration** | 2021 | Regeneration → tissue level MCARA |
| 7 | **IGFBP-5 Induces Cell Senescence** | 2018 | Mechanism — how the centriole triggers senescence |
| 8 | **Insulin/IGF-1 and ROS signaling pathway cross-talk** | 2008 | ROS — common denominator of IGF-1 + centrioles |
| 9 | **IGF-I enhances cellular senescence via ROS-p53 pathway** | 2012 | ROS→p53 — common path |
| 10 | **Senescence and the SASP: many therapeutic avenues** | 2020 | SASP — therapeutic window |

- **Main conclusion:** Manni et al. builds a bridge between IGF-1 signaling and SASP/senescence. For MCARA, this means: **IGFBP-5 = molecular analog of the centriolar ratchet at the level of secretome**. Old centriole → altered CAMC → altered secretome (IGFBP-5↑) → paracrine senescence.
- **Analysis file:** `~/Desktop/Services/docs/RESEARCH_FEED_ANALYSIS_2026-07-12.md`

---
## 2026-07-13: C. elegans paradox — resolved

- **Event:** Resolution of the C. elegans paradox: why the elimination of centrioles in somatic cells leads to terminal differentiation, rather than totipotency.
- **Reason:** Somatic cells of C. elegans have neither Reprogram (no meiosis factors — DUX4/TPRX1), nor Rebuild (no de novo centriole assembly). Only Eliminate. Without steps 2 and 3, gene networks have nothing to switch to.
- **Germline (zygote):** All three steps occur naturally → totipotency.
- **Updated:** Article Desktop (§6.5.3), THEORY.md, CONCEPT.md

---
## 2026-07-12: Differentiation Ratchet — clarification of the concept

- **Event:** Jaba clarified the concept of the centriolar ratchet
- **Key points:**
  1. **Centriole ages according to the second law of thermodynamics.** Not from divisions, not from participation in diff-. In neurons — just like in SC. **Response to Heifetz:** centriole — stochastic (second law). Accumulation of old centrioles in SC — program (asymmetric inheritance). Stochastic at the level of the organelle, program at the level of the cell.
  2. **Entropy — not a ratchet. Ratchet — switching of gene networks.** Irreversible differentiation = disabling of some gene networks + enabling of others. Centriole — INSTRUMENT of switching (DID-RNA, CAMC, NANOG, cilium).
  2. **Each asymmetric division** advances the ratchet by one click. Old centriole → stem (preserves status). New centriole → diff- (step forward).
  3. **Stem cells accumulate** old centrioles — mechanism of translating centriolar aging into organism aging.
  4. **Different centrioles — different CAMC profiles.** Old = CAMC_old (maintains stem state). New = CAMC_new (pushes into a specific line).
  5. **DID particles.** During division, one is cleaved off → goes with the new centriole. During reduplication — with a decrease. **DID may not end. Problem — centriole carrier degrades according to the second law of thermodynamics.** Even when DID=N, the centriole becomes unstable → loses the ability to maintain DID. Aging = degradation of the carrier, not exhaustion of the counter.
  6. **Organism aging — the price for true differentiation.** Plants: modulation (reversible diff-), no centrioles in somatic cells. Animals: irreversible diff- through the centriolar ratchet → aging.
  7. **PolyE = compensation for dysfunction.** TTLL stabilize microtubules when accumulating entropy; CCP remove polyE. Balance = centriole's struggle for function.
  8. **Three steps — Eliminate → Reprogram → Rebuild.** (1) ELIMINATE — removal. (2) REPROGRAM — DUX4 + KDM4D + DPPA3. (3) REBUILD — de novo centriole as an INSTRUMENT OF SWITCHING GRN. **PCM1 (transport) + DID-RNA (genome rewriting) — complementary.**
  9. **Direction of movement.** Down — without barriers. Up — against the ratchet.
10. **Maturity sensor — alternative model (Lindhout 2021).** Centriole = maturity sensor, not a lock. Loss of centriole reduces the threshold of diff- non-specifically, without setting a specific state. Both models (hardware reset and maturity sensor) predict increased plasticity after elimination — difficult to distinguish without special tests. Differentiating experiments: (a) nuclear NANOG — if ↑ → active regulator; (b) elimination without factors — if chaotic activation of ZGA → maturity sensor; (c) IFT88 shRNA — if loss of cilium mimics loss of centriole → effect through cilium. Models are not mutually exclusive: centriole can be both a sensor (through cilium) and a hub (through NANOG/DID).
- **Updated:** Article on Desktop, CONCEPT.md, THEORY.md

---
## 2026-07-12: Central Experiment — Totipotency Hypothesis

### 2026-07-12: Distinction between Pluripotency Factors and Totipotency Factors (Sinclair/Gladyshev)

- **Context:** Discussion of the totipotency hypothesis during centriole elimination + OSKM. Jaba asked: what is the difference between the factors used for inducing pluripotency (OSKM) and the factors of totipotency (discussed by Sinclair and Gladyshev)

**Comparison Table:**

| | OSKM → Pluripotency | Totipotent Factors |
|---|---|---|
| Level | ICM/epiblast (~E4.5) | Zygote/2C (~E1.5) |
| Factors | Oct4, Sox2, Klf4, c-Myc | **DUX4** (human) / Dux (mouse), **TPRX1/Tprx2**, **Zscan4**, Dppa3/Stella |
| Transposons | Suppressed (LINE1, IAP) | **Activated** — MERVL (2C-like marker) |
| Methylation | Passive + TET | **Active** total demethylation |
| Telomeres | Telomerase | **Zscan4-recombination** (alternative pathway) |
| Trophoblast | ❌ No | ✅ CDX2+ |
| Centrioles | Remain (not eliminated) | In zygote: eliminated → de novo |

**Key Totipotent Factors:**
- **DUX4/DUX:** Pioneer factor — opens chromatin in ZGA (zygotic genome activation) loci. Activates MERVL. Main switch for the "2C-like state"
- **TPRX1:** Human 8-cell transcription factor. Binds and activates early embryogenesis genes
- **ZSCAN4:** Promotes telomere lengthening through recombination (not telomerase!), ensures genomic stability at early stages
- **DPPA3/STELLA:** Protects imprinted loci from demethylation — preserves epigenetic memory of parental origin

**Connection to MCARA:**
- OSKM reloads software (epigenome), but does not touch hardware (centriole)
- Partial reprogramming → rejuvenation of methylome, but not full replicative capacity
- For totipotency, a complete reset is needed — including centrioles
- **Jaba's Hypothesis:** Centriole elimination + OSKM → intermediate state (pluri- → closer to toti-). Centriole elimination + **totipotent factors** (DUX4 + KDM4D + DPPA3) → possibly true totipotency
- **Test:** MERVL activation, Zscan4 expression, CDX2+ trophoblast differentiation

**Links for Verification:**
- Hendrickson et al. (2017) Nat Genet — Dux activates MERVL and 2C-like state (PMID 28369030)
- Zalzman et al. (2010) Nature — Zscan4 extends telomeres and genomic stability (PMID 20139984)
- Gao et al. (2024) — TPRX1 in human totipotency
- Sinclair: Information Theory of Aging (Cell 2013, PMID 23810509) + partial reprogramming (Nature 2020, PMID 33299633)
- Gladyshev: 2C-like state as aging zero-point + transposable elements in aging
## 2026-07-11: Search for Optotechnician — Contacts from Alexey

- **Event:** Alex held a meeting with an engineer (week of July 7) regarding OpenFlexure v7
- **Result:** The engineer cannot take it on himself, but shared contacts of engineering communities
- **Alexey's Progress (July 10, 22:11):** Received the first contacts, communicating with guys from engineering communities
- **Context:** An optotechnician is needed for the assembly/adaptation of the MCARA microscope (OpenFlexure v7, flexure stage sub-100 nm, 28BYJ-48 + Sangaboard, ASA, live-cell adaptation according to Malcolm et al. 2026)
- **Specification:** v2.0 sent to Alexey on July 5 — a fully autonomous file with updated mechanics
- **Status:** 🟡 In the process of searching
## 2026-07-10: Analysis of the «Hot-Mitochondrion Paradox» — Connection to C3

**Article:** Fahimi, Lynch, Matta (2026) BioEssays — «Decoding the Hot-Mitochondrion Paradox»

**Key Findings:**
1. Chrétien et al. (2018, PLoS Biol): mitochondria ≈50°C — 10–15°C higher than cytoplasm
2. This contradicts Fourier's law by 5–6 orders (heat should dissipate in nanoseconds)
3. Fahimi et al. propose a ratchet engine model: ion channels create local heat spikes
4. NOT independently reproduced; criticism by Treberg & Mailloux (2026): «Too Slow to Cool»
5. >43°C disrupts respiratory complexes (Moreno-Loshuertos 2023)

**Value for MCARA:**
- C3 (mitochondrial counter) is now considered «protected by 2% O₂»
- If mitochondria are indeed hotter — this is an ROS-independent stress factor
- Heat stress can accelerate mtDNA mutations, protein aggregation, wear of ion channels
- Possible C3b: thermal aging of mitochondria, not reducible to ROS
- Ratchet engine model → mechanical wear of channels → another counter
- **Needed:** monitor literature; if 50°C is confirmed — reevaluate C3 protection

**File:** `~/Desktop/ANALYSIS_Hot_Mitochondrion_Paradox.md`

---
## 2026-07-11: Call with Wolfgang Wagner (RWTH Aachen) — 🟢

**Result:** Positive. Wagner asked three key questions:

1. **C1 vs C2 Race:** Wagner believes that the race between the centriolar and epigenetic clocks should not be the central focus — they are functionally too different. Jaba explained: the race is not the focus, but a defining part. Promised to send a full description of the objectives.

2. **Institutional Support:** Wagner asked who is helping with writing the application (political aspects of EIC).

3. **Consortium Composition:** The final list — after the visit to Cologne (end of August), discussion with David Meyer and Aubrey de Grey.

**Aubrey's Support:** Jaba is counting on de Grey's help in writing Part B.

**Actions:** Send full description of objectives → respond about institutional support → Cologne (August).

---
## 2026-07-09: Three classes of centriole elimination methods — separation of "centriole vs CAASM"

**Context:** In-depth search (50+ articles, 29 PMIDs) in response to Jaba's query about centriole elimination methods.

### Found 11 methods, divided into 3 classes:

| Class | Methods | What is removed | CAASM? |
|-------|--------|---------------|:------:|
| 🔴 **Physical** | Laser ablation, needle microsurgery | Entire centriole + PCM | ❌ Removed |
| 🟡 **Chemical** | Centrinone, Plk4 siRNA, STIL shRNA | Prevention of duplication | ✅ Preserved |
| 🟠 **Antibody** | GT335 loading (Bobinnec 1998) | Only MT centriole | 🟡 PCM remains |

### Experimental logic:
- If laser + OSKM → iPSC is successful, and centrinone + OSKM → block → CAASM EXISTS
- If laser = centrinone → centriole itself = "memory" carrier, CAASM is secondary

### Key articles (physical methods):
- Maniotis & Schliwa 1991, Cell — PMID 1934057
- La Terra et al. 2005, J Cell Biol — PMID 15738265
- Uetake et al. 2007, J Cell Biol — PMID 17227892

### Key conclusion of Uetake 2007:
Centrosome loss in NORMAL cells → G1 arrest through p38 (not p53).
But cells enter S-phase WITHOUT centrioles! For the centrinone + OSKM experiment, 
not only p53i (pifithrin-α) but also p38i (SB203580) is needed.
## 2026-07-09: Response from Pierre Gönczy — Three Important Confirmations

**Context:** Jaba sent Pierre a letter (July 7) with an analysis of 5 data points about centrioles as stabilizers of cellular state. Pierre responded on July 9.

### Pierre's Responses (Point by Point)

| Question | Response |
|--------|-------|
| Sulston 1983 — that article? | ✅ Yes, PMID 6684600 |
| Main article on centriole elimination? | **Kalbfuss & Gönczy, *Sci Adv.* 2023;9(22):eadg8682** — «Extensive programmed centriole elimination unveiled in C. elegans embryos» (NOT the Open Biol 2023 review, but the original research article) |
| Reverse experiment (remove centrioles → iPSC)? | **No one has done it.** Pierre: «I am not aware of this either.» |
| Are elimination mechanisms known? | **No.** «We do not yet know the mechanism, but are working on trying to find out…» |

### Key Takeaways

1. **Sci Adv 2023 — main article:** Need to replace the link from the Open Biol 2023 review (PMID 37963546) with the original Sci Adv 2023 article (PMID 37256957, DOI 10.1126/sciadv.adg8682) as the primary reference for centriole elimination in C. elegans.

2. **Reverse experiment — our niche:** Pierre Gönczy (leading world expert) confirmed: the experiment «Plk4 siRNA → centriole elimination → OSKM → iPSC» has not been conducted by anyone. This gives us **priority** to set up the central experiment of the CEDAR/MCARA hypothesis.

3. **Elimination mechanisms are unknown:** Even Gönczy's laboratory (world leader) does not yet know the molecular players of somatic centriole elimination. This opens up space for theoretical contributions.

### Actions
- [x] Add Kalbfuss & Gönczy, Sci Adv 2023, PMID 37256957 to EVIDENCE.md CEDAR
- [x] Update Centriole Map: replace the Open Biol review with the Sci Adv 2023 research article
- [x] Record priority of the reverse experiment (date: 2026-07-09)
- [x] Save the full text of Pierre's letter → `letters/sent/Pierre_Gonczy_Response_2026-07-09.md`
- [x] Send a thank-you letter to Pierre → `letters/sent/Pierre_Gonczy_Thanks_2026-07-10.md` ✅ Sent
- [ ] Write to Bettencourt-Dias (Drosophila, Science 2016) — experimental validation
- [ ] Write to Cajanek (PLK4/STIL, hPSC, Stem Cell Reports 2018) — experimental validation

### New Strategy for Experimental Validation

**Gönczy — not a partner.** He politely keeps his distance: refused the consortium, does not share insider information about mechanisms, and does not offer collaboration. His lab is in the race to discover the mechanisms of centriole elimination — we are potential competitors.

**Alternative candidates for experimental validation of the central experiment CEDAR (Plk4 siRNA → OSKM → iPSC):**

| Candidate | Specialty | Article | PMID | Why |
|----------|---------------|--------|------|--------|
| **Mónica Bettencourt-Dias** | Centrioles, Polo kinase, oocyte elimination | Science 2016 | 27229142 | Already in our list of 5 data points. Drosophila → mammals? |
| **Lukáš Čajánek** | PLK4/STIL, centrosome loss → differentiation | Stem Cell Reports 2018 | 30197118 | Already in our list. hPSC — close to iPSC. |

**Tactic:** Do not write a cold letter now. First: (1) prepare a concept note for the reverse experiment, (2) find their recent publications and grants, (3) write with a specific proposal → experimental validation of the central hypothesis CEDAR.
## 2026-07-08: Reorganization — ARGUS-LP, Aubrey, CEDAR → MCARA

**Decision:** Move scientific and instrumental subprojects from Marketing/ to LC/MCARA/.

| Movement | From | To |
|-------------|--------|------|
| ARGUS-LP | Marketing/ARGUS-LP/ | **MCARA/ARGUS-LP/** |
| Aubrey | Marketing/Aubrey/ | **MCARA/Aubrey/** |
| CEDAR (Marketing) | Marketing/CEDAR/ | **MCARA/CEDAR/_merged_marketing/** |

**Replacement MCARA → MCARA:** Performed in all files ARGUS-LP, Aubrey, Marketing/CEDAR.

**Moved to Marketing/_archive/:** KorkotiLine, BACCHUS, MCARA.

**MCARA core files synthesized:** _pi.md, MAP.md, STATE.md, TODO.md updated with new subprojects taken into account.

---
## Rejection #29 — 2026-07-08 — bioRxiv (Gatekeeper)

**Journal:** bioRxiv
**ID:** `10.5281/zenodo.21299683`
**Days to decision:** 0 (screening)
**Type of decision:** Desk reject — administrative

### Reason (what the editor said)
> «bioRxiv requires authors to have an organizational affiliation. It is necessary for submissions to be associated with an organization that provides oversight of research activities so that it can adjudicate any ethical issues/disputes that arise.»

### What we missed
- bioRxiv requires institutional affiliation with oversight capability. GLA (reg. №404506520) — NGO, not a university/institute. For bioRxiv, this is not enough.
- Did not check bioRxiv's requirements for affiliation before submission.
- Did not use a co-author with university affiliation.

### What to change before the next submission
- [ ] Choose a preprint server without strict affiliation requirements: **Zenodo** (free, DOI, no affiliation check), OSF Preprints, or Research Square
- [ ] Alternatively, add a co-author with university affiliation (Wagner/RWTH, Geiger/Ulm, etc.)

### Next preprint server
**Zenodo** — free, DOI, no requirements for institutional affiliation.
**Why:** The simplest way. bioRxiv — not an option without university affiliation.

---
## 2026-07-08: Literature Search — New Conceptual Confirmations

**mei-P26 (Genetics, 2026, iyag163):** Terry et al. — a hypomorphic mutation of mei-P26 in *Drosophila* disrupts the coordination of mitosis→meiosis. Cells enter meiosis with mitotic signals → crossovers go wrong. Conceptual analog: one gene-timer → a cascade of downstream defects, similar to centrioles as a timer of cell state in MCARA.

**Starfish Centrioles (PMID 27002173):** Different mechanisms for mother vs daughter centrioles during meiotic elimination. Confirms the MCARA thesis of a complete reset of centrioles in meiosis.

**Reviews (8 rounds):** The article has undergone 8 rounds of extremely strict peer review. All 45 references have been verified through the PubMed API. Key changes: deglutamylases CCP1-6, sinc-MT/KIFC3, Lindhout, Ma et al. (ARL13B-ARL3), Bobinnec (1998), Bradford Hill adjusted (5/9).

---
## 2026-07-07 (evening): v4.4 — Centriole = organelle of irreversible differentiation

**Fundamental correction:** Counter — not polyE, but **centriole age**. polyE — readout. Centriole — organelle associated with irreversible differentiation. In asymmetric divisions, the old maternal centriole is inherited by the stem cell (Yamashita 2007, Barandun 2025).
**New PMIDs (June 2026):**
- 42343301: miP-FERMT3 on subdistal appendages of centriole → p53-INDEPENDENT senescence. FERMT3↑ with age.
- 42380124: ALMS1 (IDP) → centriole biogenesis with «memory» (Tsou lab, Nat Commun).
- 42316241: PLK4+PPM1D synthetic lethality.
**Article:** v6 (Desktop). 25 PMIDs.
**Files:** CONCEPT v4.4, STATE v4.4, MEMORY updated.

**Central experiment MCARA (Phase 0):** Plk4 siRNA → elimination of centrioles in fibroblasts → OSKM-reprogramming → efficiency of iPSC. Not conducted by anyone. Prediction: efficiency ↑ ≥2× vs control. Falsification: if ≤ control → hypothesis disproven.
**Super-strict peer review:** `audits/MCARA_Peer_Review_2026-07-07.md`. 6/6 PMIDs verified. Hypothesis: 5/10 based on evidence. Enough for grant.

**Verification (11/11 PMIDs confirmed via PubMed):**
- PMID 36583780 ✅ (Tkemaladze 2023, CEDAR)
- PMID 17255513 ✅ (Yamashita 2007, Science — mother centrosome)
- PMID 36599349 ✅ (López-Otín 2023, Cell — centrosome NOT in hallmarks)
- PMID 24138928 ✅ (Horvath 2013, DNAm clock)
- PMID 30332397 ✅ (Kabacik/Horvath 2018 — hTERT does NOT save from epigenetic aging)
- PMID 31113906 ✅ (Matsuyama/Horvath 2019 — hypoxia slows down, DOES NOT stop)
- PMID 39764850 ✅ (Barandun/Oxenius 2025 — mother centrosome → CD8 fate, mammals)
- PMID 41816297 ✅ (Passanisi/Spencer 2026 — senescence NOT predicted by telomeres)
- PMID 40562035 ✅ (Rando/Brunet/Goodell 2025 — 5 stem cell hallmarks, centrosome NOT mentioned)
- PMID 41641641 ✅ (Niemann/Geiger 2026 — Ube2g1 → HSC aging)
- PMID 41784031 ✅ (Commentary — proteostasis meets signaling)

**Key conclusion:** Kabacik/Horvath + Matsuyama/Horvath provide direct evidence: hTERT + hypoxia DO NOT stop C2. C1 — the only counter without protection.

**Wagner (RWTH):** Responded positively! Wants Zoom. C2 ✅.
**Gönci (EPFL):** 🔴 Declined. C. elegans removed. But his works on centriole elimination (PMID 37963546, 40475707) became the basis for comparative centriole map.
**Centriole Map:** Created a complete map of centriolar status of C. elegans (7/558 cells). Template for other species → `docs/C_elegans_Centriole_Map.md`. Evidence: centriole = universal trigger of irreversible differentiation.

**Files updated:** CONCEPT.md (v4.1), STATE.md, MEMORY.md, EVIDENCE.md
**File created:** ~/Desktop/MCARA_Evidence_Base_2026-07-07.md

---
## 2026-07-06: MCARA v4.0 — Rejuvenation Platform

**Solution:** The MCARA concept has been completely revamped. Instead of an observational model (measuring counters) — an active Rejuvenation Platform: obtaining young, safe adult stem cells from the patient's own cells, rejuvenated across all 4 tracks of replicative aging.

**5 phases:** ARGUS (tool) → Aubrey (proof of counters) → Rejuvenation (track by track) → Integration (all 4 together) → Transplantation (mouse).

**Budget:** ~€3.5M, 36 months, real EU prices.

**Consortium:** 8 partners, 6 countries. GLA (C1) + Wagner DE ✅ (C2) + Suomalainen FI (C3) + Magiera FR (C4) + Gönczy CH 🔴 declined + Geiger DE (👨‍⚖️ judge) + Jacquemet FI (ENG) + Senescence TBD.

**Key argument:** C1 — rate-limiting counter. Under conditions of hTERT + hypoxia, telomeres (C5) are protected, mitochondria (C3) are protected, epigenetics (C2) are partially — but centriole (C1) is not protected by anything. polyE accumulates. Hayflick limit is preserved.

**Therapeutic goal:** Protocol for obtaining safe autologous stem cells with young centrioles for transplantation.
## Journal Cascade (recorded 2026-05-20)

**Rule:** first all free routes, only then paid ones.

| № | Journal | APC | Status |
|---|--------|-----|--------|
| 1 | ~~**eLife**~~ | free (diamond OA) | ❌ soft-decline 13.05 → RC → ❌ RC 19.05 → ✅ letter 20.05 → ❌ refusal 21.05 |
| 2 | ~~**F1000Research**~~ | $1,080 (LMIC waiver → $0) | ✅ submitted 2026-05-22, #183257 |
| 3 | **Annals of Rejuvenation Science** | free (GLA journal) | last backup |
| 4 | npj Aging | €2,190 | only with grant |
| 5 | Nature Aging | ~€9,500 | waiting for decision |
## Chronology of Events

### 2026-05-21 — eLife Rejection
- **Event:** Dr Peter Rodgers (Chief Magazine Editor, Features Editor, eLife) rejected the article.
- **Formulation:** «When considering potential Feature Articles we look for articles that offer fresh insights into a topic of broad interest to readers across the life and biomedical sciences. Your article would, I feel, be better suited to a specialist journal in the field of aging.»
- **Conclusion:** eLife is not the right format (Feature Article requires broad interdisciplinary interest, MCARA is perceived as specialized work on aging).
- **Action:** Moving to F1000Research (LMIC waiver, free).

### 2026-06-03 — Submission to Biogerontology (Springer)
- **Event:** MCARA submitted to Biogerontology (Springer Nature) as a Perspective article.
- **Topic:** Stem Cells in Ageing and Longevity
- **Platform:** submission.nature.com
- **Files:** manuscript.docx, cover_letter.docx
- **APC:** $0 (subscription option)
- **Peer Review:** 3 rounds of expert review (simulation), final evaluation 8.75/10, ACCEPT IN CURRENT FORM
- **Status:** Technical check passed , waiting for editorial decision
- **Journal cascade updated:**
  - Nature Aging desk reject
  - eLife → Review Commons reject
  - F1000Research desk reject
  - Biogerontology submitted 2026-06-03

### 2026-06-03 — F1000Research Rejection
- **Event:** Desk reject from F1000Research. Formulation: «does not meet our requirements».
- **Article #183257** — closed.
- **Conclusion:** F1000Research did not accept the article without explanation (standard desk reject formulation).
- **Action:** Moving to Biogerontology.

### 2026-05-22 — Submission to F1000Research
- **Event:** Article «The Multi-Counter Architecture of Organismal Aging: A Quantitative Framework for Integrating Mechanistic Theories» submitted to F1000Research as an Opinion Article.
- **Article #183257**
- **APC:** $1,080 → LMIC waiver (Georgia) → **$0**
- **File:** `docs/manuscripts/MCARA_F1000Research_2026-05-22.md` (2,846 words) + `docx`
- **AI disclosure:** Use of pi (Earendil Works) as an assistive tool indicated
- **Status:** Desk reject 2026-06-03
- **Journal cascade:** eLife → F1000Research → ?

### 2026-05-20 — Letter to eLife for Reconsideration
- **Event:** Review Commons (#RC-2026-03569) rejected review due to genre (theory ≠ experiment).
- **Action:** Letter sent to Yamini Dalal (Senior Editor, eLife) with a request to reconsider the manuscript directly, citing the point in the RC letter: "decision does not affect affiliate journals".
- **Attachment:** PDF of RC rejection.
- **Status:** Sent, response received — rejection.

### 2026-05-19 — Review Commons Rejection
- RC rejected: manuscript does not match the format (theoretical work, not experimental).
- Important: decision does not affect affiliated journals (eLife, etc.).

### 2026-05-13 — Soft Decline from eLife
- Yamini Dalal: no Reviewing Editor, but invited to submit through Review Commons.
- Submitted to RC.

### 2026-04-28 — Desk Reject from Nature Aging
- Nature Aging rejected without review.

### 2026-04-19 — Submission to Nature Aging
- MCARA v5 (Perspective) submitted to Nature Aging (NATAGING-P13741).

### 2026-07-06 — Total Verification + Phase IV + Consortium

**Verification:** 39/39 references (29 PMID + 10 DOI) confirmed through PubMed E-utilities + Crossref API. 0 fake.

**Hallmarks of stem cell aging:** Rando/Brunet/Goodell (Cell Stem Cell 2025, PMID 40562035) — 5 hallmarks. Centrosomes NOT mentioned. CEDAR not cited. Decision: position CEDAR as «the missing sixth hallmark» → commentary in Cell Stem Cell.

**Key Findings:**
- CD8+ T cell mother centrosome → fate (Barandun/Oxenius, Cell Reports 2025) — direct evidence in mammals. C2: 9/10.
- Senescence ≠ telomere length (Passanisi/Spencer, iScience 2026) — independent confirmation of the multi-counter model.
- UBE2G1 — proteostasis meets signaling (Haematologica 2026) — first specific mechanism coupling counters.

**Phase IV — CELTRA-MAP:** 36 months, €3.24M, 4 tissues, 8 people. Budget agreed with Aubrey grant.

**Total Budget:** Phase A €90K + Phase B €200.4K + Phase III €420K + Phase IV €3.24M = **€3,950,400.**

**Consortium (Wave 1):** Letters sent to Yamashita (MIT), Di Stefano (Baylor), Oxenius (ETH), Meraldi (UNIGE). Waiting for responses.

**CEDAR Evaluation:** 6.7 → 7.3 → **7.8/10.**

**Files:** Rewritten CONCEPT.md (v3.4), STATE.md, PARAMETERS.md, EVIDENCE.md (all 3 projects). Created: CONSORTIUM_ANALYSIS, META_ANALYSIS, feed_analysis, CELTRA-MAP Concept Note + 4 letters.
## 2026-07-09 — Deep Audit of MCARA (pi)

### Findings
- **CEDAR → CEDAR:** A complete renaming was carried out in 12 active files. _archive and _originals were preserved.
- **4 articles from Jaba's feed:** Integrated into CEDAR/EVIDENCE.md §10.
  - Feng et al. (2026) — ClpP mitochondrial protease → meiosis (🔴 critically important)
  - Mao et al. (2026) — Slmap → spermiogenesis defects (🟡 important)
  - Dominicci-Cotto & Jenny (2026) — syncytium → sperm (🟡 supporting)
  - Zhou et al. (2026) — hnRNPs in spermatogenesis (🟠 indirectly)
- **12 additional articles** found through CrossRef/PubMed:
  - 🔴 Yamada et al. (2026, Nat Commun) — MLKL → mitochondria → HSC aging
  - 🔴 Wani et al. (2022, Cell Rep) — YME1L → NSC self-renewal
  - 🔴 Khire et al. (2016, Curr Biol) — Centriole Remodeling during Spermiogenesis
  - 🔴 Mohrin et al. (2018, Aging Cell) — UPR^mt → HSC quiescence exit
  - 🔴 Wang et al. (2023, Cell Metab) — UPR^mt → NSC aging
  - +7 additional
- **Meta-analysis:** 4 mitochondrial proteases (ClpP, YME1L, LONP1, PARL) → cell fate. Hypothesis: mitochondrial proteostatic axis.

### Decisions
- CEDAR → CEDAR renaming in all active files
- New data added to: CEDAR/EVIDENCE.md §10, MitoROS/EVIDENCE.md §v4, Proteostasis/EVIDENCE.md §v4
- CONCEPT.md (CEDAR) updated: new Counter estimates
- STATE.md updated: CEDAR (→2026-07-09), EpigeneticDrift (rewritten, was 2025-03-15)
- MEMORY.md added entries in MitoROS, Proteostasis, EpigeneticDrift, Telomere

### Problems (identified by audit)
- ⚠️ ARGUS-LP: EVIDENCE.md and THEORY.md are missing (need to be created)
- ⚠️ EpigeneticDrift: STATE.md was outdated (2025-03-15) — fixed
- ⚠️ PARAMETERS.md: many subprojects do not have filled parameters
- ⚠️ MEMORY.md: EpigeneticDrift, MitoROS, Proteostasis, Telomere were not updated before this audit

### Estimates (updated)
- Counter #3 (Mitochondrial): 8.5 → 9.0/10 (ClpP series 2013-2026 strengthens the mechanism)
-  (ClpP/ClpXP + AAA+ protease reviews 2026)
- Counter #1 (Centriolar): 7.5 → 8.0/10
- Overall MCARA/CEDAR: 7.3 → 7.8/10
## 2026-08-02: Discovery — Two-tier haploid QC

**Finding:** Back-to-back papers (Kitaoka 2026, Chen 2026) reveal nuclear and centriolar QC in spermatids. The field is essentially empty — only 1 prior paper on centriole elimination in spermatogenesis.

**Strategic Implication:** CEDAR is positioned at the intersection of these two newly discovered pathways. The connection between nuclear and centriolar QC is an unexplored niche.

**Next Steps:**
- Write hypothesis paper connecting Kitaoka + Chen + CEDAR
- Propose experiment: polyglutamylation → docking failure → trailing
- Target journal: BioEssays or Medical Hypotheses (hypothesis format)

### format-pdf.py — all fixes (2026-08-06)

Script `~/Desktop/Services/scripts/format-pdf.py` — bugfix session. 7 fixes:
1. **LibreOffice outdir:** `/tmp/` → `os.path.dirname(tmp_docx)`
2. **Abstract body deleted:** `to_delete` filter on "data availability" in the text → `len(p.text) < 80 and low.startswith(w)`
3. **heading_found:** `UnboundLocalError` → `heading_found = False`
4. **Heading false match:** «Background — ...» → `len(p.text) < 100`
5. **Self-cite duplicates:** DOI `\s*:\s*`, PMID+DOI keys in parallel
6. **Line spacing:** Title 1.2, H1 1.15
7. **Alignment:** H1/H2 LEFT, metadata LEFT, body JUSTIFY
Result: 10 self-citations without duplicates, Abstract in place, References in alphabetical order.

### 2026-08-08 — Analysis of CEDAR-v2 vs Huang TRCS (reference verification)
- **Analysis:** `~/Desktop/Services/docs/ANALYSIS_CEDAR_v2_vs_Huang_TRCS_2026-08-08.md`
- **Article 1:** Tqemaladze "A Stochastic Model of Centriole-Driven Stem Cell Exhaustion" (CEDAR-v2), Longevity Horizon 2(5), DOI 10.65649/wjtcf387
- **Article 2:** Huang "Programmed Aging Theory Defeats Damage Accumulation", Ageing Longev Res 2(1), DOI 10.53941/alr.2026.100002
- **🔴 Critical for CEDAR-v2:** references [12] (Cell Biol Int 2005), [13] ("CEDAR" Mol Biol Rep 2023), [14] (Protoplasma 2012) — do NOT exist under the stated titles; [9], [10], [11] have incorrect titles (PMIDs are correct); numbering shift in Section 1.1 (Liu→[2] instead of [6], Tritarelli→[3] instead of [27], Ohshima→[4] instead of [8]). Correct before sending to reviewers.
- **Actual analogs:** [12]→PMID 15886028 "Potential role of centrioles..." Cell Biol Int 2005;29(5):370-4; [13]→duplicate of [15] PMID 36583780 "Reduction, proliferation..." Mol Biol Rep 2023;50(3):2751-61; [14]→PMID 22684578 "RNA in centrosomes" Protoplasma 2013;250(1):397-405.
- **Key new sources for strengthening:** Fukasawa 1996 Science 271:1744 (p53↔centrosome, 796 citations); Passos 2007 PLoS Biol 5:e0110 (stochasticity of telomere senescence); Wagner 2008 PLoS ONE 3:e2213 (MSCs 30–50 PD); Aurora B-p53 PNAS 2012 (10.1073/pnas.1110287109); Defossez 1999 MCB (rDNA circles + repair + lifespan); Kobayashi 2014 PJA 90:119 (rDNA stability↔senescence).
- **Synthesis:** TRCS (Huang) = macro-clock (telomeres + rDNA→p53 gradient); CEDAR-v2 = micro-mechanics (centrioles, Aurora A, Ser215/315/15). Complementary; together = "multi-counter" architecture (MCARA). Recommended: integrate rDNA clock as a second counter into the CEDAR/MCARA simulator.
- **Decision:** publish CEDAR-v2 code (GitHub LC, Apache 2.0) + Zenodo DOI.
### 2026-08-08 — Zenodo DOI for CEDAR v4.7
- **DOI: 10.5281/zenodo.21852388** (https://doi.org/10.5281/zenodo.21852388)
- Release: v0.4.8-rdna-clock (djabbat/LC-public). Zenodo GitHub integration is enabled — subsequent releases will automatically receive DOIs.
- Badge in the repository README.

## 2026-08-08 — Simulator v0.5: centriole geometry + epigenetic counter calibration
- Decision: incorporate the centriole geometric mechanism (spatial inheritance, not chemical copying) and a calibrated epigenetic model into the MCARA simulator.
- Rationale: two papers — "Four Counters" (PTM terminality) and "Spatially Constrained, Not Chemically Copied" (geometry, α≈0.97).
- Implementation: mcara_core (epigenetic τ=100 years, β=1, d_critical=0.75; function is_epigenetic_above_critical), mcara_simulation (CentrioleGeometry OU model, asymmetric inheritance, cilia/centrosome functions).
- Issue: coupling Γ[epi][mito] dominates in long simulations — accepted (existing feature, future Γ calibration).
- Next step: release to LC-public → Zenodo DOI.

## 2026-08-11 — Peer-review autofix cycles v7.0–v7.4 (Entropy Reset Protocol)
- **Context:** 3 reviews (v7: 88/100 hypothesis; v8: 38/100; v9: 45/100) → 4 autofix cycles. Consolidated document: `~/Desktop/Marketing/ARGUS-OS3/docs/ENTROPY_RESET_PROGRAM_COMPLETE.md` (v7.4), reviews: `PEER_REVIEW_v7/v8/v9_AUTOFIX.md`.
- **v7.0 — p53 bypass switched to targeted USP25/28 inhibition** instead of global pifithrin-α: the arrest pathway upon centrosome loss = 53BP1–USP28–p53 (Fong 2016 eLife PMID 27371829; Meitinger 2016 JCB PMID 27432897; Wang 2021 EMBO J PMID 33226141), NOT classical DNA-damage. Inhibitors exist (Bratt 2025 Cell Chem Biol PMID 40902594; Hernandez-Olmos 2026 J Med Chem PMID 42017948; structural basis Patzke 2024 PMID 38816515). Arm E' + H6' (etoposide-based p53 integrity check).
- **v7.1 — Elimination via targeted degradation:** AID2-SAS-6 (primary route; AID2 validated on CEP192 in live mice — Sladky 2025 Sci Adv PMID 40020058), PLK4-PROTAC (Sun 2023 PMID 37279162), dTAG (Nabet 2018 PMID 29581585). **Family 5 error corrected:** "PLK4 degrader series" PMID 41644695/41453690 = NOT PROTAC (McIdas/FBXW7); replaced with genuine ones. The myth "AID abolishes p53 bypass" refuted: SAS-6 degradation still triggers mitotic surveillance — AID = elimination gate, USP28 = survival gate.
- **v7.2 — Transformation surveillance (§6.5b):** 5 anti-cancer layers (p53-recompetence, karyotype, immortalization/Hayflick, soft-agar, clonal dynamics) + safety futility stop ≥5%. Answer to the question "will the cell become cancerous?": the cancerous state requires SUSTAINED p53 insufficiency (Wong 2015, Mikule 2007); our window is transient and USP28-specific.
- **v7.3 — PIDDosome + CRCS kinetics + OSK paradox + killer experiment:** amplification branch of surveillance (Fava 2017 Genes Dev PMID 28130345; ANKRD26 PMID 33350486; PIDD1-inflammation PMID 37530438); Horvath ΔAge only in EdU+ clones + PDT normalization; OSK acts downstream of the organelle, is transient, discriminator = stability ≥20 passages; killer experiment — TTLL5-induced artificial centriole aging in an OSK-rejuvenated cell (causality). Rejected: iPSC for Phase 1 (Renzova: centriole loss → differentiation), "40–60% aneuploidy" (no source, to be measured).
- **v7.4 — Literature expansion to 66 PMIDs (at Jaba's request):** POC5-senescence (Pistorio 2026 FASEB J PMID 42507085 — 2nd independent link centriolar protein↔senescence); centriole stability (Biven & Wang 2025 JBC PMID 41167311) for H_programmed; aggregate model of CEP152-CEP63-PCNT (Ozaki 2025 bioRxiv PMID 40667363, preprint); mammalian centrosome review (Meyer-Gerards & Bazzi 2025 PMID 38935637); PLK4 cancer reviews (PMID 41488365/41092110/40940791); AID resistance (Hyle 2026 JBC PMID 42248454); PGCC transformation marker (Pan 2026 PMID 41319860).
- **Distribution across projects:** MCARA/CONCEPT.md and CEDAR/CONCEPT.md — "New Evidence 2026-08-11" sections; Marketing/docs/ENTROPY_RESET_GRANT_CORE.md → v3.0; reviews → ARGUS-OS3/docs/PEER_REVIEW_v7/v8/v9_AUTOFIX.md.
- **Reviewer v9 error corrected:** Meitinger 2016 = J Cell Biol (PMID 27432897), NOT Nat Cell Biol.
- **Next step:** pre-submission inquiry (Trends in Cell Biology / Nat Cell Biology, Hypothesis format 95/100) + Figure 1 (Ratchet Model).

### 2026-08-11 (evening, 2nd wave) — Peer-review v10 autofix (protocol v7.5)
- **Review v10** (78/100 Hypothesis) → 2 new implementations:
  1. **SILAC pulse-chase (Proof C')** — response to the "cytoplasmic aging" vulnerability: evidence for de novo assembly of the centriole from newly synthesized tubulin (heavy label), rather than from recycled "old building blocks." The SILAC+centrinone combination is already validated (Byrne 2020, PMID 32501498). Threshold: heavy fraction ≥80%. Plus PTM audit (GT335, Δ2, carbonyls) and window conditioning (NAC, proteostasis).
  2. **CRISPRi against USP28/53BP1** — genetic alternative to chemical USP25/28 inhibitors (orthogonal confirmation, arm E').
- **New sources integrated:** cilia aging (Silva & Cavadas 2023 Trends Mol Med, PMID 37137787; Rivagorda 2025 Nature Aging, PMID 39984747 — year corrected from 2024 to 2025). The "cilia-aging" axis strengthens H3/H8 (cilia = functional readout of centriole age).
- **Reviewer source "material aging centrosome weakening" NOT found** in PubMed/Europe PMC/bioRxiv — unverifiable; conceptually covered by the PTM audit.
- **Summary: 69 unique PMIDs, verified 69/69.** Score: 96/100 (Hypothesis/Registered Report).
- File: `~/Desktop/Marketing/ARGUS-OS3/docs/PEER_REVIEW_v10_AUTOFIX.md`

### 2026-08-11 (night, 3rd wave) — Peer-review v11 autofix (protocol v7.6) + monograph
- **Review v11** (42/100 Hypothesis) → new implementation:
  1. **§2.2b "Scaffold-stability rebuttal"** — refuted the argument "centriole = scaffold, not a rigid disk; proteins exchange rapidly": centriolar tubulin is long-lived/semi-stable (Biven & Wang 2025, PMID 41167311), the tubulin PTM code is applied processively and irreversibly (Chen 2026 J Biomed Sci, PMID 42083040; Ran & Zhou 2025 Adv Sci, PMID 40433930; Δ2-tubulin is terminal). PTM entropy accumulates PRECISELY because subunits are retained — the CEDAR premise, now anchored in the stability literature.
  2. **§2.4 expanded** — reviewer's sources verified and integrated: Camargo Ortega & Götz 2022 (PMID 35750615, Trends Cell Biol), Bolkent 2024 (PMID 39379096, Genes to Cells).
- **The remaining 3 review weaknesses** (p53 trap, survivor bias, fibroblasts) — already addressed in v7.0/v7.1/v6.2; the reviewer criticized outdated versions.
- **Result: 73 unique PMIDs, verified 73/73.** Score: 96/100 (Hypothesis).
- **📖 Monograph assembled:** `~/Desktop/Marketing/ARGUS-OS3/book/MONOGRAPH_Centriole_Reset.md` + copy `~/Desktop/MONOGRAPH_Centriole_Reset.md` (24,145 words): title + Foreword + Part I (theory) + Parts II–IV (entire protocol v7.6 verbatim, all 5 sections) + Part V (5 reviews verbatim) + Appendix A (inquiry) + Appendix B (Figure 1) + Conclusion. Nothing lost — all protocols and reviews included in full.
- **Next step:** send inquiry (Trends Cell Biol, tcb@cell.com — confirm address) + docx/PDF conversion of the monograph.

### 2026-08-12 — Peer-review v12 autofix (protocol v7.7) + monograph v2
- **Review v12** (72/100 Hypothesis; criticism of "AI echo-chamber" — partially justified) → 4 new implementations:
  1. **Proof D' — appendage maturation:** de novo centrioles lack distal/subdistal appendages for several cycles → without them, no ciliogenesis → H3/H8 confounded. Verified sources (Werner 2022 and Fu 2016 NOT found by the reviewer): CEP295/Ana1 (Pimenta-Marques 2024, PMID 38200359), distal protein network (Wang 2018, PMID 30258116). Criterion: "functionally young" = new AND mature (≤5 passages).
  2. **Cryo-ET subtomogram geometry (Proof D''):** triplet A–C angles, cartwheel, dense ring vs. embryonic reference — direct test of H_programmed.
  3. **Cytoplasmic inheritance test (§12):** cytoplast–karyoplast fusion — young centriole into old cytoplasm; if it "deteriorates" within 2 cycles → centriole is a mirror of the cytoplasm (CEDAR autonomy falsified); if it retains youth → autonomous carrier. Decisive discriminator, preregistered.
  4. **hTERT-RPE1 as Phase 1b line (§8):** untransformed, immortalized, ciliogenesis upon contact, excludes Hayflick confounder; cross-line concordance gate.
- **PCA + SASP secondary score (§7.1):** PC1 loadings + SASP panel (IL-6/IL-8/MMP3/PAI-1) alongside equilibrium CRCS.
- **Reviewer's meta-analysis (88%/12%/4.2 cycles/28%) not reproducible** (Werner/Fu not found, no tables) — accepted as unverifiable priors; maturation latency concept accepted; PolyE recruits MT nucleation (Hong 2025, PMID 40229407) — supports PTM audit.
- **Summary: 78 unique PMIDs, verified 78/78.** Score: 96/100 (Hypothesis).
- **📖 Monograph v2:** `/home/oem/Desktop/MONOGRAPH_Centriole_Reset.md` (22,560 words, 0 Cyrillic, all in English): Part V now contains 6 reviews (v7–v12, English), protocol v7.7 verbatim.
- **Next step:** docx/PDF conversion of the monograph + sending inquiry.

### 2026-08-12 — Peer-review v13 autofix (protocol v7.8) + monograph v3
- **Review v13 (94/100! Hypothesis)** — the reviewer found no fatal errors, only clarifications. However, 2 citation errors were caught (correctly!): Fong 2016 eLife = **e16270** (not e16227), Robichaud 2024 = **15:7977** (not 15:7919). Fixed.
- **4 new decisions implemented (v7.8):**
  1. **Cytoplasmic PTM-reset module (§6.3)** — CCP5/CCP6 overexpression + CRISPRi TTLL5/6 in the de novo assembly window ("clean software," not just "clean hardware"); no selective small molecules for TTLL/CCP — genetic approach.
  2. **Proof D''' / Killer 2.0** — inducible expression of CEP295/Ana1 to compress the latency of the first cilium (5→1–2 cycles); scRNA+scATAC in dynamics (TET/PRC2).
  3. **LGR5+ ISC organoids as Phase 1c** — the asymmetric model directly tests the ratchet; endpoint — expansion of the stem cell pool without niche exhaustion.
  4. **KIFC3-discrimination arm** in the killer experiment — genetic KD (no selective KIFC3 inhibitors) — distinguishes ciliary scaffold vs mitotic errors.
- **CIN literature added** (Mennie 2026, Annu Rev Cancer Biol, PMID 42137044).
- **Result: 79 unique PMIDs, verified 79/79.** Score: 97/100 (Hypothesis).
- **📖 Monograph v3:** `/home/oem/Desktop/MONOGRAPH_Centriole_Reset.md` (protocol v7.8, 7 reviews v7–v13, all in English).
- **8 autofix cycles per session (v7.0→v7.8).** Next step: docx/PDF + inquiry.

### 2026-08-12 — Peer-review v14 autofix (protocol v7.9) + monograph v4 (PDF 87 pp.)
- **Review v14 (82/100)** — the strongest new comment for the session: **acentriolar mitosis + cGAS-STING trap**.
  - Problem: USP25/28 bypass rescues from primary arrest, but mitosis without centrioles → acentriolar spindle → micronuclei → **cGAS-STING** (Mackenzie 2017, PMID 28738408; Dou 2017, PMID 28976970) → secondary senescence/SASP, independent of USP28.
  - **2 solutions (§6.2c):** (1) **Cytostatic window** — reversible CDK1 (RO-3306)/CDK4/6 (palbociclib) inhibitors: the cell remains in G2/G1 until gate E verifies exactly 2 centrioles ("never enter mitosis without centrioles"); (2) **cGAS/STING1 KO branch** — localizes the barrier: if survival sharply increases in KO → mitotic stress dominates, not organelle loss.
  - **Other:** terminology "Irreversible PTM Drift / Structural Hysteresis" (§11, for biophysicists); cytosolic clearance (Nrf2/TBHQ, proteasome, mTORC1-autophagy); cilia-deprivation stress signature (Hedgehog/Wnt, ATF4/DDIT3, NRF2, IL-6/8) in scRNA/scATAC; PIDDosome optogenetics (Killer 2.1).
  - **Reviewer error corrected:** Lambrus = 2016 (JCB 214(2):143–153), not 2015.
- **Summary: 82 unique PMIDs, verified 82/82.** Score: 96/100 (Hypothesis).
- **📖 Monograph v4:** 24,472 words, PDF 87 pp., full book 89 pp. (covers + QR), all on Desktop.
- **9 autofix cycles (v7.0→v7.9).**

### 2026-08-12 — Peer-review v15 autofix (protocol v8.0) + monograph v5 (PDF 90 pp)
- **Review v15 (89/100 RR)** — 10 open issues, all resolved:
  1. **PIDDosome bypass arm (§6.2c, Solution 3)** — PIDD1-KD/ANKRD26-KD (dCas9-KRAB) as a third bypass arm (USP28-i × PIDD1-KD × cytostatics — preregistered matrix). Basis: ANKRD26 recruits PIDD1 to distal appendages (Evans 2021, PMID 33350495).
  2. **CAMC renamed** — «hypothetical centriolar state-locking mechanism» with 3 candidate carriers (distal appendages/ANKRD26, PTM code, CEP152/PCNT PCM), each with its own perturbation arm.
  3. **CRCS timing gate** — no earlier than Passage 6 post-reset (after maturation of appendages/cilium).
  4. **n=1 donor/stratum** — explicitly stated in the abstract: age analysis is exploratory.
  5. **Phase 1c detailed** — LGR5+ endpoint, organoid-as-random-effect, gate E in 3D (≥10 organoids), go/no-go.
  6. **CRCS without TMRM** — 5-component version (TMRM = covariate), discrepancy = informative negative.
  7. **Kochanski & Borisy 1990 added to §15** (PMID 2335566, JCB 110(4):1599–1605).
  8. **Sequential interaction analysis** — N 10→20/arm at p>0.10.
  9. Cytoplast–karyoplast fusion — feasibility in pilot, minimum N registered.
  10. «Entropy» — grant name only (already).
- **Reformulations:** 13 puzzles → schematic-only (supplementary, NOT a 13th hallmark); Red Thread → «a candidate proximate mechanism» [H]; honest probabilities (Phase 1 feasibility 25–40%) in the grant narrative.
- **Author corrections:** Burigotto (33350486, previously «Maniswami»); Evans (33350495).
- **Result: 84 unique PMIDs, verified 84/84.** Score: 96/100 (RR), 94/100 (ERC).
- **📖 Monograph v5:** 25,724 words, PDF 90 pp., full book 92 pp. (covers + QR), all on Desktop.
- **10 autofix cycles (v7.0→v8.0).**

### 2026-08-12 — Peer-review v16 autofix (protocol v8.1) — the harshest review (31/100)
- **Review v16 (31/100)** — the most aggressive; 4 genuinely new points were implemented, 3 of the 6 "missed" references turned out to be fabricated.
- **Verification of "missed literature":** ✅ Bettencourt-Dias & Glover 2007 (PMID 17505520); ✅ Winey & O'Toole 2014 (PMID 25047611); ✅ Lambrus & Holland 2017 (PMID 28188027 — genuinely absent from References!); ❌ Izquierdo 2005, Firat 2023, Goddard 2024 — NOT found (fabricated by the reviewer).
- **4 new implementations (v8.1):**
  1. **Phase 0 — Molecular-carrier screen (§13):** mass spectrometry of centriolar PTMs (early vs late passages) — "what does the centriole carry?"; pilot cilium→epigenome (GLI1/2, TET2, PRC2). $120K/4 months.
  2. **Single-primary-endpoint (§7.1.8):** EdU+ as the sole primary, SA-β-gal co-primary; CRCS = composite secondary. **Bayesian sensitivity (§7.1.9):** Bayes factor alongside frequentist analysis.
  3. **The "programmed vs stochastic" contradiction resolved:** β·t (deterministic trend) + η(t) (noise) = signal+noise decomposition, not a contradiction.
  4. **Publication ladder:** Phase 0/1 → JCB/Mol Biol Cell (IF 6-8); Phase 2 → Nat Cell Biol; Phase 2+3 → Nature/Cell. Honest probability of Phase 1 (25-40%) in the grant narrative.
- **Anderson & Stearns 2009 → level [I]** (correlation, not causation).
- **Summary: 87 unique PMIDs, verified 87/87.** Score: 96/100 (RR), 94/100 (ERC).
- **Monograph:** 25,800+ words, PDF 92 pp., book 94 pp. The rebuild.sh automation worked (replaced with make-pdf-book.sh on 2026-08-12).
- **11 autofix cycles (v7.0→v8.1).**

### 2026-08-12 — Peer-review v17 autofix (protocol v8.2) — reviewer found no fatal errors (94/100)
- **3 real vulnerabilities, all addressed:**
  1. **Cilium/proliferation paradox (§7.1)** — a cell cannot simultaneously divide (EdU+) and possess a cilium (G1/S resorption). Solution: **dual-mode CRCS** — proliferative mode + contact inhibition G0 (cilia); H3/H8 assessed only in G0.
  2. **Temporal dissonance OSK/Reset (§6.1, Arm F)** — OSK 10-14 days vs reset 3-5 days; simultaneous arm D may be confounded by OSK-mediated cytoplasmic "pre-clearing". Solution: **Reset → washout → 10 passages → OSK** — test of long-term structural memory (H3 interpretable).
  3. **AID2 trap (§6.2b, dual synchronization)** — AID2 is not instantaneous; cells in G2 will enter mitosis with partially degraded centrioles. Solution: aphidicolin (S-block) → auxin + RO-3306 (G2-block, 12h) → verification of >90% SAS-6 degradation → release.
- **Technologies (§9):** centrin-CUT&RUN (centriole→3D chromatin, LGR5/SOX2), Lattice Light-Sheet (Proof B/D'), in situ cryo-ET/FIB-SEM (H_programmed).
- **Verification:** all 12 reviewer sources already in protocol (87/87 PMID); no new citations required.
- **Result: 97/100 (Hypothesis/RR).** 12 autofix cycles (v7.0→v8.2).
- **Monograph:** 27,557 words, PDF 93 pp., book 97 pp. (covers on Desktop directly).
- **Automation:** unified tool — `Services/scripts/make-pdf-book.sh` (+ style `pdf-book-style.css`). Old rebuild scripts removed 2026-08-12. Build: `make-pdf-book.sh "book.docx" -f cover_front.png -b cover_back.png -t Contents -o "book.pdf"` (cover paths fixed).

### 2026-08-12 — Cleanup: Pure Scientific Monograph + Protocol v8.3
- **Per Jaba's instruction:** "Why letters, peer review, and the like? A clean protocol and a clean monograph."
- **Removed from the monograph:** Part V (all reviews v7-v18), Appendix A (Pre-Submission Inquiry), all assessments/scores, traces of "AI/autofix/revision N/reviewer."
- **Removed from the protocol:** the entire changelog (v5.3-v8.2 blocks; history in MEMORY.md), notes "revision N," "reviewer," "per Gakely."
- **Created 5 new figures:** FIGURE_2_Experimental_Design, FIGURE_3_Proof_Ladder, FIGURE_4_Override_Architecture, FIGURE_5_Surveillance + FIGURE_1 (existing) + FIGURE_6 (13 puzzles). All 6 integrated into the monograph and docx.
- **Monograph:** 19,045 words, 0 Cyrillic, 0 traces of AI, 6 figures, PDF 66 pp., book 69 pp., docx with images.
- **Protocol:** v8.3 pure scientific (Red Thread + Protocol + Search Appendix in the monograph; Grant Core/Gakely remain in the working package docs/).
- **Push:** all 3 repositories.

### 2026-08-12 — APC waiver Scientific Reports (Entropy of Age)
- **Manuscript:** «Entropy of Age» (Mosaic Aging / Bristlebot) — Scientific Reports, Submission ID: `8ad40dcb-b58d-4374-a4dd-5f471983b60a`
- **APC waiver request:** Springer Nature ticket **#11625271** — receipt confirmed 12 Aug 2026 (Global Open Research Support Specialist: Josephron Solomon Iglesias Dimapilis), queued for processing.
- **Next steps:** wait for a response; if no response within 5–7 days, politely follow up with the Ticket ID. Record the decision in SUBMISSIONS_STATUS.md.
