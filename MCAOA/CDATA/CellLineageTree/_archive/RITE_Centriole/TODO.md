# RITE_Centriole — TODO (Phase A)

## A1. Design (Weeks 1–2)
- [ ] Finalize scaffold protein choice (Centrin-1 vs SAS-6 vs CEP152) — pick top 2
- [ ] Design 3 plasmid maps in SnapGene (loxP/lox2272, tag swap, Cre-ER^T2)
- [ ] Codon-optimize for Homo sapiens
- [ ] Order gene synthesis (Twist Bioscience)
- [ ] Parallel: order pre-made Cre-ER^T2 lentivirus from Addgene as fallback

## A2. Cloning & packaging (Weeks 3–5)
- [ ] Subclone RITE cassette into pLenti backbone
- [ ] Sequence-verify full construct (Sanger + nanopore whole-plasmid)
- [ ] HEK293T lentivirus packaging (see `../LentiviralTools/`)
- [ ] Titer virus (qPCR + functional titer)

## A3. Validation (Weeks 6–8)
- [ ] Transduce HEK293T; confirm mCherry localization to centrioles (co-stain with γ-tubulin)
- [ ] 4-OHT pulse; measure swap kinetics by live imaging
- [ ] Quantify swap efficiency at 24/48/72 h post-pulse
- [ ] Determine optimal pulse duration

## A4. BJ-hTERT lineage-ready lines (Weeks 9–12)
- [ ] Transduce BJ-hTERT
- [ ] Single-cell clone by FACS
- [ ] Karyotype 3 best clones
- [ ] Deposit validated plasmids on Addgene
- [ ] Zenodo DOI for construct maps + validation dataset

## Gate to Phase B
- Swap efficiency ≥ 80 %
- Clear single-centriole resolution under live imaging
- ≥ 2 validated clonal BJ-hTERT lines
