#!/usr/bin/env python3
# Replication of CEDAR pilot (GSE293225) on independent dataset GSE175533 (WI-38, RS PDL series)
# Young = PDL20+PDL25 (proliferating) vs Senescent = PDL50+PDL52+PDL53
import csv

TPM = '/tmp/RS_TC_raw_TPM.tsv'
GENES = ['PLK4','STIL','CEP128','CEP192','CENPJ','CEP152','SASS6','CEP97','TTBK2','POC1A',
         'CEP290','NIN','CEP250','POC5','PCM1','OFD1','POC1B','CEP63','CEP135','CEP164','CEP170']

young = ['PDL20_TP1_A','PDL20_TP1_B','PDL20_TP1_C','PDL25_TP2_A','PDL25_TP2_B','PDL25_TP2_C']
old   = ['PDL50_TP7_A','PDL50_TP7_B','PDL50_TP7_C','PDL52_A','PDL52_B','PDL52_C','PDL53_A','PDL53_B','PDL53_C']

rows = {}
with open(TPM) as f:
    r = csv.DictReader(f, delimiter='\t')
    for row in r:
        g = row['gene']
        rows.setdefault(g, {})
        for k, v in row.items():
            if k != 'gene':
                rows[g][k] = float(v)

print(f"{'gene':<8}{'young(PDL20-25)':>16}{'old(PDL50-53)':>16}{'old/young':>10}   pilot")
print('-'*72)
ups = downs = total = 0
for g in GENES:
    if g not in rows:
        print(f"{g:<8} NOT FOUND"); continue
    d = rows[g]
    yv = [d[c] for c in young]
    ov = [d[c] for c in old]
    ym = sum(yv)/len(yv); om = sum(ov)/len(ov)
    ratio = om/ym if ym > 0 else float('nan')
    total += 1
    if ratio >= 1.15: ups += 1
    elif ratio <= 0.87: downs += 1
    flag = ''
    if ratio >= 1.15: flag = ' UP>15%'
    elif ratio <= 0.87: flag = ' DOWN>15%'
    print(f"{g:<8}{ym:>16.2f}{om:>16.2f}{ratio:>10.2f}   {flag}")
print('-'*72)
print(f"ИТОГО генов: {total} | ↑>15%: {ups} | ↓>15%: {downs}")
