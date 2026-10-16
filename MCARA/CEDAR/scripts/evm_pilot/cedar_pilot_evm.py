#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
CEDAR EVM PILOT 2026-10-15 — протеостаз-пилот: пептидная связь в явно-электронном движке EVM
Цель: проверить, сохраняет ли EVM (O(N^2) дифференцируемый явно-электронный движок Anastasov)
стабильную геометрию аминокислоты (глицин) и дипептида Gly-Gly (минимальная модель пептидной
связи белков — ключевая единица протеостаза/фолдинга/агрегации).

Протокол = QM9-протокол авторов (500 шагов замороженные ядра + свободные шаги, RMSD <= 0.15 Å).
Дополнительно: длина пептидной связи C-N против эмпирической (~1.33-1.46 Å).
"""
import sys, os
sys.path.append(os.path.abspath("/tmp/Anastasov_Theory/04_Emergent_Valence_Mechanics"))
import torch
from rdkit import Chem
from rdkit.Chem import AllChem
from core.system_builder import EVMBuilder

torch.set_num_threads(os.cpu_count())

def conf_from_smiles(smiles, name):
    mol = Chem.MolFromSmiles(smiles)
    mol = Chem.AddHs(mol)
    AllChem.EmbedMolecule(mol, AllChem.ETKDGv3())
    if AllChem.MMFFOptimizeMolecule(mol, maxIters=1500) != 0:
        AllChem.UFFOptimizeMolecule(mol, maxIters=1500)
    conf = mol.GetConformer()
    nuclei_info = []
    for i, atom in enumerate(mol.GetAtoms()):
        p = conf.GetAtomPosition(i)
        nuclei_info.append({'Z': atom.GetAtomicNum(), 'pos': [p.x, p.y, p.z]})
    return mol, nuclei_info

def run_evm(name, mol, nuclei_info, free_steps=500):
    Zs = [n['Z'] for n in nuclei_info]
    print(f"\n=== {name} ===  атомы={len(Zs)}  электроны={sum(Zs)}  формула: ", end="")
    from collections import Counter
    c = Counter(Zs); print("C%d H%d N%d O%d F%d" % (c.get(6,0), c.get(1,0), c.get(7,0), c.get(8,0), c.get(9,0)))

    builder = EVMBuilder(forcefield_path="/tmp/Anastasov_Theory/04_Emergent_Valence_Mechanics/core/evm_forcefield.json", device='cpu')
    engine = builder.build_engine(nuclei_info, batch_size=1)
    initial_pos = engine.nuc_pos.clone()

    # Phase 1: 500 steps frozen nuclei (Born-Oppenheimer analogue)
    engine.damping_nuc = 0.0
    frozen = engine.nuc_pos.clone()
    for _ in range(500):
        engine.step(dt=0.005)
        engine.nuc_pos.copy_(frozen)
        engine.nuc_vel.zero_()

    # Phase 2: free nuclei
    engine.damping_nuc = 0.99
    for step in range(1, free_steps+1):
        engine.step(dt=0.005)
        if step in (100, 300, 500):
            rmsd = torch.sqrt(torch.mean(torch.sum((engine.nuc_pos[0]-initial_pos[0])**2, dim=-1))).item()
            print(f"  free step {step:>3}: RMSD vs start = {rmsd:.4f} Å  {'OK' if rmsd<0.15 else '>0.15 (деформация)'}")

    final = engine.nuc_pos[0].detach().cpu().numpy()
    start = initial_pos[0].detach().cpu().numpy()
    rmsd = torch.sqrt(torch.mean(torch.sum((engine.nuc_pos[0]-initial_pos[0])**2, dim=-1))).item()
    status = "STABLE" if rmsd <= 0.15 else "UNSTABLE"

    # Peptide bond C-N length (dipeptide): find C(=O)-N pair
    cn = None
    if 'Gly-Gly' in name or 'gly' in name.lower():
        from rdkit.Chem import rdchem
        # locate amide/peptide C-N: any N bonded to a carbonyl C
        amide_cn = []
        for bond in mol.GetBonds():
            a1, a2 = bond.GetBeginAtom(), bond.GetEndAtom()
            if (a1.GetAtomicNum()==6 and a2.GetAtomicNum()==7) or (a1.GetAtomicNum()==7 and a2.GetAtomicNum()==6):
                ia, ib = a1.GetIdx(), a2.GetIdx()
                amide_cn.append(((start[ia], start[ib]), (final[ia], final[ib])))
        if amide_cn:
            import math
            def d(p,q): return math.dist(p,q)
            cn = [(d(*s), d(*f)) for s,f in amide_cn]
    return name, mol, status, rmsd, cn, final, start

if __name__ == "__main__":
    results = []
    # Контроль воспроизводимости: водный димер (заявка авторов ~2.78 Å)
    O1,H1a,H1b = [0,0,0],[0,.9,0],[.8,-.4,0]
    O2,H2a,H2b = [0,2.8,0],[.8,3.3,0],[-.8,3.3,0]
    wd = [{'Z':8,'pos':O1},{'Z':1,'pos':H1a},{'Z':1,'pos':H1b},{'Z':8,'pos':O2},{'Z':1,'pos':H2a},{'Z':1,'pos':H2b}]
    b = EVMBuilder(forcefield_path="/tmp/Anastasov_Theory/04_Emergent_Valence_Mechanics/core/evm_forcefield.json", device='cpu')
    e = b.build_engine(wd, batch_size=1)
    e.e_phase = e.e_phase.clone(); e.e_phase[0,10:20,0] *= -1.0
    e.damping_nuc=0.0; e.damping_e=0.90
    frozen=e.nuc_pos.clone()
    for _ in range(500): e.step(dt=0.005); e.nuc_pos.copy_(frozen); e.nuc_vel.zero_()
    e.damping_nuc=0.99
    for _ in range(500): e.step(dt=0.005)
    oo = torch.norm(e.nuc_pos[0,3]-e.nuc_pos[0,0]).item()
    print(f"=== WATER DIMER (контроль) === O-O final = {oo:.3f} Å (заявка авторов ~2.78 Å)")
    results.append(("WaterDimer", None, "CONTROL", 0.0, oo, None, None))

    # Протеостаз-пилот: глицин + Gly-Gly (пептидная связь)
    for smi, nm in [("NCC(=O)O","Glycine"), ("NCC(=O)NCC(=O)O","Gly-Gly dipeptide")]:
        mol, ni = conf_from_smiles(smi, nm)
        name, mol, status, rmsd, cn, final, start = run_evm(nm, mol, ni)
        results.append((name, mol, status, rmsd, cn, final, start))

    print("\n" + "="*60)
    print("ИТОГИ ПИЛОТА (протокол QM9: RMSD<=0.15 Å = стабильно)")
    for name, mol, status, rmsd, cn, final, start in results:
        extra = ""
        if isinstance(cn, float):
            extra = f" | O-O = {cn:.3f} Å (эмп. ~2.78-2.82)"
        elif cn:
            cs = ", ".join(f"{s:.2f}->{f:.2f}Å" for s,f in cn)
            extra = f" | C-N: {cs} (эмп. ~1.33-1.47 Å)"
        print(f"  {name:22s}: {status:9s} RMSD={rmsd:.4f} Å{extra}")
    print("="*60)
