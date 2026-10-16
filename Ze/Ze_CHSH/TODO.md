# TODO — Ze_CHSH

> **📄 Articles and publications:** see `~/Desktop/Services/publications/PUBLICATIONS_TRACKER.md`

## P0 — Verification (current phase)

- [x] **Verify reference [11] EXISTS**: Abboud, Guan, Bradlyn, Noronha (2026) LGI–QFI bound — **FOUND 2026-10-15**: arXiv:2604.09772 (см. EVIDENCE.md)
  - [x] **Сверить содержимое**: выкачан arXiv:2604.09772 → форма в CONCEPT НЕ совпадает: реально $F_Q \geq 8(K-\langle Q^2\rangle)$ (коэф. 8, не 2)
  - [x] **ПЕРЕСЧЁТ β**: β НЕ выводится (5 дефектов, см. CONCEPT «BETA VERDICT») → β = постулат; статья §3.2 требует переформулировки
- [ ] **Verify reference [13] content**: Hall (2013) — ✅ EXISTS arXiv:1307.2697 (correlation distance ↔ mutual information, кубиты) — сверить точную форму $C \leq 2\sqrt{I(2\ln 2-I)}/\ln 2$; кросс-ссылки: Rastegin 2014 (DOI 10.1088/0253-6102/62/3/05), Cao 2016 (DOI 10.1038/srep23758)
- [x] **Verify reference [14] EXISTS**: Honma & Vu quantum thermokinetic TUR — **FOUND + VERIFIED 2026-10-15**: arXiv:2510.04866 (2025, год верный) = PRA 2026, DOI 10.1103/f77p-kw54; абстракт дословно содержит «quantum thermokinetic uncertainty relation»
  - [x] **Сверить содержимое** — подтверждено (arXiv:2510.04866: info-thermodynamic bounds, демон Максвелла, квантовые часы)
- [ ] **Verify reference [12]**: Seshadreesan 2013 «CHSH–QFI» — ❌ **неверная атрибуция**: arXiv:1310.1410 говорит ОБРАТНОЕ (CHSH-нарушение не квантует QFI); формула «S² ≤ 8+4F_Q» отсутствует — убрать/переписать
- [x] **Переписать §3.2 статьи v3**: ВЫПОЛНЕНО 2026-10-15 (β — постулат, 2√2/ln2, честная оговорка ref [12], §2.2-§2.3-§3.1 переработаны; v3_ru удалена по указанию автора — только v3_en)
- [x] **Численная валидация**: ВЫПОЛНЕНА 2026-10-15 (`scripts/chsh_entropy_validation.py`, §5.4 в v3_en) — MC-симуляция протокола + анализ чужих loophole-free данных (H_Ze=0.100-0.107 vs H_EOM=0.41-0.43, 4× разделение)
- [x] **Решить по журналу**: Quantum отклонён (обязателен arXiv-препринт — сейчас невозможно). ВЫБРАН **Foundations of Physics (Springer)** — без arXiv, Word/docx принимается, тематика foundations of QM идеальна, без обязательного APC. Пакет ГОТОВ: `Ze_CHSH/FoP_Submission/` (статья md+docx, симуляция, фигура, cover_letter_FoP.md, README, SUBMISSION_NOTES). Осталось: вычитать docx, переименовать Fig1.png, подать через портал Springer.
- [x] **Вычистка основной статьи**: AI-маркеры, эмодзи и служебные пометки убраны из `docs/Ze_CHSH_Entropy_Quantum_v3_en.md`; все 18 references согласованы с цитатами в тексте; docx перегенерирован.
- [ ] **Verify reference [7] content**: Burgholzer (2015) — ✅ EXISTS arXiv:1502.00214 (stochastic thermodynamics, entropy production) — сверить отождествление «импеданс ≈ ⟨ΔS⟩_gen»; кросс-ссылка: Rastegin 2015 (DOI 10.1016/j.aop.2015.02.015)
- [ ] **Double-check derivation of β** from combination of bounds
- [ ] **Run mbpr** — peer review of the article
- [ ] **RAKTS/SED-секция интерпретаций** (из анализа Anastasov 2026-10-15): добавить в «Relation to Other Interpretations» — детерминистические контр-модели: SED (PMID 9901125), Bohm 1952, RAKTS как современный кодируемый local-realist аналог (шум/энтропия → случайность, параллель с S(H)); см. `EVIDENCE.md`

## P1 — Foundation

- [ ] Write Python script: simulation of CHSH(entropy)
- [ ] Check: does the depolarizing channel give a different β?
- [ ] Compare predictions: Ze vs naive depolarizing channel
- [ ] Update the article with verified references

## P2 — Experimental

- [ ] Find a quantum optics collaborator
- [ ] Prepare a grant application for the experiment
- [ ] Write experimental protocol v2
