# Commit Log — Wave 1 Corrections (CommonHealth Empirical Audit)

**Дата:** 2026-04-26
**Trigger:** Wave 1 peer review v2 (CommonHealth Empirical, agent ab87c710)
**Source:** `/home/oem/Desktop/LC/_audits/PEER_REVIEW_v2_Empirical_2026-04-26.md`
**Mode:** Overnight autonomous corrections (between Wave 1 and Wave 2)

---

## Применённые исправления

### BioSense/EVIDENCE.md — 4 PMID corrections

| Утверждение | Старый PMID | Новый PMID | Источник | Verification |
|---|---|---|---|---|
| Voytek 2015 J Neurosci (gamma age) | 22993427 | **26424877** | Voytek B et al *J Neurosci* 2015;35(38):13257-65 | PubMed 2026-04-26 |
| Iyengar 1996 Circulation (HRV age) | 11788280 | **8967405** | Iyengar N et al *Circulation* 1996 | PubMed 2026-04-26 |
| Kleiger 1987 Am J Cardiol (SDNN mortality) | 7545779 | **3812275** | Kleiger RE et al *Am J Cardiol* 1987;59(4):256-62 | PubMed 2026-04-26 |
| Task Force 1996 Circulation (HRV standards) | 15855343 | **8598068** | Task Force ESC/NASPE *Circulation* 1996;93(5):1043-65 | PubMed 2026-04-26 (15855343 was 2005 paper) |

### Ontogenesis/EVIDENCE.md — 2 fabrication removals

1. **Smith J. et al. 2025 Nat Commun 16:4501 — FLAGGED** (DOI 10.1038/s41467-025-65974-8 не разрешается, single-letter author = red flag fabrication). Помечено в EVIDENCE.md как FLAGGED, удалено из доказательной базы до верификации.
2. **DOI 10.1016/j.dcn.2021.100971 — REMOVED** (hypothetical citation, признанный авторами; недопустим в EVIDENCE.md).

### CDATA/EVIDENCE.md — 1 DOI correction

| Утверждение | Старый DOI | Новый DOI | Источник |
|---|---|---|---|
| Goetz & Anderson 2010 cilium Hedgehog | 10.1038/nature08117 | **10.1038/nrg2774** | Goetz SC, Anderson KV *Nat Rev Genet* 2010;11(5):331-44, PMID 20395968 |

---

## Не применено (требует ручного вмешательства)

### CDATA Sobol-парадокс (C1)

S1(epigenetic_rate)=0.403 > S1(alpha_centriolar)=0.224, ablation эпигенетического признака улучшает R². Это формальная internal contradiction центрального тезиса CDATA. **Не fixable через citation correction** — требует:
- Counter-factual analysis с only-centriolar features
- LOO-CV bias correction (-0.093)
- Reformulation core CDATA hypothesis
- Возможно reset gипотезы

Уже adressed в `~/Desktop/LC/CDATA/docs/CDATA_REFORMULATION_2026-04-26.md` (rigorous version, не universal cascade). Sobol-парадокс остаётся active concern для Wave 2 (fund-perspective review).

### HAP — stub state, требует rebuild

10/10 EVIDENCE.md PMID были fabricated, заменены stub. Текущая версия — single citation (Tqemaladze 2026 Longevity Horizon, non-PubMed-indexed). Не fixable through corrections — требует full literature search + rebuild EVIDENCE.md from verified PubMed sources. Halt status сохраняется (per CommonHealth/CLAUDE.md 2026-04-21 audit).

### Ontogenesis — 6/6 prior fabrications quarantined

Файл KNOWLEDGE.md.QUARANTINED_2026-04-21 сохраняется. EVIDENCE.md теперь содержит только verified ссылки + 2 fabrication-flag actions выше. Halt status сохраняется до полного rebuild.

### FCLC — структурные claims (PATE prototype, ε-budget, SecAgg+ semi-honest)

Не fixable through citation correction — требует:
- Full PATE implementation (currently stub)
- Byzantine-resilient aggregation integration (Krum/Bulyan)
- External audit (SOC2/HIPAA BAA/DP audit)
- Signed EU LoIs

Документировано в FCLC/DEEP_AUDIT_2026-04-21.md.

---

## Status

✅ **Citation integrity issues fixed:** 4 BioSense PMIDs, 1 CDATA DOI, 2 Ontogenesis fabrications flagged/removed.
⏸ **Structural issues queued для Wave 2:** CDATA Sobol-paradox, HAP stub, Ontogenesis pending rebuild, FCLC PATE/audit.

Wave 2 (fund-perspective review) запустится автоматически после получения 2 оставшихся Wave 1 agents (CommonHealth Top+MCAOA+Ze, PhD), применения их corrections, и затем launch Wave 2 agents на исправленной версии.
