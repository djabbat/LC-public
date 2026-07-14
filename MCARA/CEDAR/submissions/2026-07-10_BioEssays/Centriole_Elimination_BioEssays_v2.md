**Jaba Tqemaladze, MD**\
Georgia Longevity Alliance, Tbilisi, Georgia; International Teaching University of Georgia, Tbilisi, Georgia\
jaba@longevity.ge \| ORCID: 0000-0001-8651-7243

# Abstract

No one has tested whether a somatic cell stripped of its centrioles can
be pushed to a more plastic state. A systematic search of the literature
--- sixty verified PMIDs --- turns up nothing. The closest experiment,
by Renzova and colleagues (2018), showed that removing centrioles from
already-pluripotent stem cells triggers their differentiation. What
happens in the opposite direction --- removing centrioles from a
fibroblast, then adding Yamanaka factors --- has simply never been
tried.

The CEDAR hypothesis makes a specific, falsifiable claim about this gap.
The centriole, it holds, is a physical barrier to reprogramming. Over
time the organelle accumulates polyglutamylation on its tubulin --- a
form of structural entropy --- and the proteins at its appendages
remodel into a Centrosome-Associated Memory Complex, or CAMC. CEDAR
argues that CAMC actively maintains the differentiated state. If the
hypothesis is right, eliminating centrioles before OSKM exposure should
raise reprogramming efficiency, not lower it. The claim seems
counterintuitive only if one thinks of the centriole as a
spindle-organizer. If it is instead a hub for fate-determining signals
--- ODF2, AKNA, NANOG --- removing it removes a lock.

The evidence that centrioles matter for cell fate is broad. In C.
elegans, 88% of somatic cells eliminate their centrioles during
embryogenesis on a rigid, cell-type-specific schedule. In planarians,
stem cells lack centrioles entirely; the organelles appear de novo only
when a cell commits to differentiation. In Drosophila, Polo kinase
drives centriole elimination during oogenesis --- block it, and the
flies are sterile. In mammals, the older mother centrosome segregates
asymmetrically at division and helps determine whether a daughter
becomes an effector T-cell or a memory T-cell, a neuron or a progenitor.

This paper describes a thirteen-group experiment that pits five
competing hypotheses against one another. Chemical elimination with
centrinone preserves CAMC on existing centrioles; physical ablation with
a laser removes everything. If the two methods give different results,
CAMC is real. If they give the same result, the centriole's microtubules
--- not its signaling surface --- carry the relevant information. Every
possible outcome is informative.

The experiment is technically feasible with off-the-shelf reagents, a
three-year timeline, and a budget under three million euros. Whatever it
finds, it will settle a question at the intersection of centrosome
biology, stem cell reprogramming, and the biology of aging.

**Keywords**: centriole, centrosome, reprogramming, iPSC,
differentiation, CEDAR hypothesis, CAMC, polyglutamylation, stem cell,
centrinone, cell fate, PLK4

## 1. Introduction

A fibroblast does not easily forget what it is. The Yamanaka factors ---
OCT4, SOX2, KLF4, and c-MYC --- can push it back to pluripotency, but
roughly one cell in a thousand completes the journey \[24\]. The rest
encounter barriers: p53-dependent checkpoints, histone modifications
that refuse to budge, the awkward mesenchymal-to-epithelial transition
that reprogramming demands \[12\]. Two decades of work have catalogued
these barriers in molecular detail.

The CEDAR hypothesis \[25\] points to a barrier of a different sort ---
one that sits not in the genome or the epigenome but in a tiny cylinder
of microtubules that every dividing cell inherits from its mother.

### 1.1. What CEDAR Claims

The centriole is a slow-renewing structure. Like any long-lived protein
assembly it accumulates post-translational modifications. CEDAR can be
decomposed into three nested claims.

**CEDAR-α (the odometer).** Centriolar tubulin acquires
polyglutamylation through TTLL5 and TTLL6. Because the centriole lacks
efficient dediglutamylase activity, polyE accumulates with successive
divisions --- a cumulative marker of replicative history. This is a
correlational claim: polyE reflects age.

**CEDAR-β (the signaling complex).** The mother centriole's appendages
carry a Centrosome-Associated Memory Complex --- CAMC --- whose
candidate components include ODF2, AKNA, and NANOG. CEDAR proposes that
as polyE builds up, CAMC remodels, altering the signaling properties of
the centriole surface. This is a mechanistic claim: CAMC transduces
centriole age into cell-fate decisions.

**CEDAR-γ (the reprogramming barrier).** CAMC actively maintains the
differentiated state. Removing the centriole --- and CAMC with it ---
removes this maintenance signal, making the cell more amenable to
Yamanaka factor-mediated reprogramming. This is the central, falsifiable
prediction that the proposed experiment tests.

The three claims are separable. The experiment tests CEDAR-γ directly;
CEDAR-β is tested by comparing physical and chemical elimination
methods; CEDAR-α is tested by measuring GT335 fluorescence as a function
of donor age and reprogramming outcome.

The idea borrows from what happens in the germline. Oocytes eliminate
their centrioles during meiosis. The sperm brings a centriole that is
restructured into a seed --- not a template --- for the de novo
centrioles of the embryo. The organism resets its hardware at the start
of every generation. CEDAR asks whether the same trick can be pulled in
a dish, with Yamanaka factors standing in for fertilization.

The prediction is straightforward: remove centrioles from a fibroblast,
add OSKM, and reprogramming efficiency should go up. The centriole, on
this account, carries a signal that maintains differentiation; removing
the signal should make the cell more malleable.

The prediction has never been tested.

### 1.2. The Gap

A PubMed search for "(centriole OR centrosome) AND (iPSC OR
reprogramming) AND (OSKM OR Yamanaka)" returns nothing. "Centrinone AND
reprogramming" also returns nothing. The experiment has not been
performed, nor reported in preprint, nor mentioned in any review I have
been able to locate. Pierre Gönczy, whose laboratory at EPFL leads the
field of programmed centriole elimination, confirmed as much in a
personal communication on July 9, 2026: to his knowledge, no such
experiment exists anywhere in the literature.

An obvious objection: if the centriole is a genuine barrier, why does
standard OSKM reprogramming work at all, producing iPSCs at 0.1--1%
efficiency? The answer is that standard iPSCs are pluripotent, not
totipotent. They form the three germ layers but cannot generate an
entire organism. This restriction may reflect the persistence of
centriolar entropy. Full reprogramming to totipotency --- the kind that
occurs naturally only through the germline --- may require both the
epigenetic reset that Yamanaka factors provide and the organelle-level
reset that centriole elimination would achieve. The low efficiency of
standard reprogramming and the failure to produce totipotent cells are
both consistent with a centriolar barrier that OSKM alone cannot fully
overcome.

The remainder of the paper reviews the evidence from five phyla linking
centriole status to cell fate; describes a thirteen-group design that
distinguishes five competing hypotheses; and walks through the possible
outcomes and their implications for CEDAR.

## 2. Evidence from Five Phyla

### 2.1. C. elegans: Programmed Elimination in the Soma

Kalbfuss and Gönczy (2023, Science Advances) combined lattice
light-sheet microscopy with correlative electron microscopy and lineage
tracing to track centrioles through C. elegans embryogenesis. The result
was unambiguous: 88% of cells lose theirs. The elimination follows a
rigid schedule --- same cell type, same developmental time, every time.
When the authors experimentally altered a cell's fate, its centriole's
fate followed.

A common objection to CEDAR is that centriole elimination in C. elegans
occurs in cells already committed to their fate --- much like the
expulsion of the nucleus from human erythrocytes. The objection mistakes
the observation for a counter-argument. Centrioles are eliminated at two
distinct points in the life cycle: when a cell reaches its terminal
differentiation state and no longer needs the organelle (somatic
elimination, as in erythrocytes), and when the differentiation program
must be reset entirely (elimination during meiotic division of oogonia).
The former is a consequence of terminal differentiation; the latter is a
precondition for totipotency. The proposed experiment recreates the
second scenario --- eliminating centrioles before resetting cell
identity --- in a somatic context. That nature uses both strategies, at
different points, strengthens rather than weakens the experiment's
rationale.

The molecular choreography has three acts \[9\]. First, SAS-1 and SPD-2
actively maintain the centriole's structure. In the priming stage, these
maintenance factors are inactivated and the central tube is lost.
Finally, the microtubule cylinder itself disassembles. In the adult
worm, Croisier and Gönczy (2025) confirmed by electron microscopy that
only seven cells in the entire L1 larva still possess centrioles.

### 2.2. Planarians: Centrioles Appear Upon Differentiation

The planarian *Schmidtea mediterranea* inverts the C. elegans pattern.
Li and coworkers (2020, Biology of the Cell) found that proliferating
neoblasts --- the animal's pluripotent stem cells --- have no centrioles
at all. The organelles appear de novo only when a neoblast commits to a
differentiation pathway. Here, centriole gain rather than loss marks the
transition out of stemness, but the fundamental point is the same:
centriole status and cell fate change together.

### 2.3. Drosophila: Polo-Dependent Elimination

Pimenta-Marques and colleagues (2016, Science) showed that in the
Drosophila female germline, Polo kinase keeps the pericentriolar
material intact. When Polo activity drops, PCM disperses and the
centrioles are eliminated. Flies that cannot execute this elimination
are sterile. Bonente and colleagues (2025) have since catalogued the
phenomenon across Drosophila development.

### 2.4. Mammals: Asymmetric Inheritance

A mammalian cell about to divide has two centrioles of unequal age: a
mother centriole, built at least two cycles ago, with fully mature
distal and subdistal appendages, and a daughter centriole, built in the
previous cycle, still structurally immature. The two do not segregate
randomly at mitosis.

Barandun and coworkers (2025, Cell Reports) showed that when an
activated CD8+ T-cell divides, the daughter that inherits the mother
centrosome becomes an effector cell; the daughter that gets the daughter
centrosome becomes a memory cell. The effect depends on ninein, a
protein of the subdistal appendages.

Zhao and colleagues (2025, Nature Communications) found that in radial
glial progenitors of the zebrafish, PCM1 on the mother centrosome
coordinates polarized endosome trafficking --- through Par-3 and dynein
--- to determine whether a division produces two progenitors or a
progenitor and a neuron.

Yamashita and coworkers (2007, Science) first described the phenomenon
in Drosophila germline stem cells: the mother centrosome stays with the
stem cell; asymmetric inheritance is necessary for stem cell
maintenance. Royall and colleagues \[12\] later confirmed the same
pattern in human neural progenitor cells.

Taken together, these observations establish that asymmetric centrosome
segregation is a conserved mechanism for specifying daughter cell fate
--- but the direction is tissue-specific. In CD8+ T-cells, the mother
centrosome goes to the effector daughter \[1\]; in neural progenitors,
the mother centrosome is retained by the stem cell \[22\]. This
context-dependence means the mother centriole does not carry a universal
differentiation signal. Rather, the cell interprets centrosome age
according to its own developmental program. In CEDAR's framework this is
expected: the same centriole can signal differently depending on whether
the cell resides in a stem cell niche or an immune context. The unifying
principle is not the direction of the signal but the fact that
centrosome asymmetry carries fate-relevant information.

A further implication, often overlooked, is that early asymmetric
divisions leaving only one daughter with stem cell potential correlate
with centriole age: the daughter inheriting the younger centriole tends
to retain stemness, while the daughter inheriting the older one tends to
commit. Whether the older centriole actively drives differentiation or
is passively segregated alongside other fate determinants remains open.
CEDAR interprets the correlation as evidence that the older centriole
carries CAMC inducers of differentiation, but this is a hypothesis ---
the proposed experiment tests whether the tendency is causal.

### 2.5. The Renzova Experiment

Renzova and colleagues (2018, Stem Cell Reports) came closest to the
experiment proposed here --- but in reverse. They treated human
embryonic stem cells and iPSCs with centrinone, a PLK4 inhibitor that
prevents centriole duplication. Within three days, over 95% of cells had
lost their centrioles. The cells did not die. Instead, they shed
pluripotency markers and spontaneously differentiated into mesoderm,
endoderm, and neuroectoderm.

Two mechanisms were at work. One ran through p53: prolonged mitosis in
the absence of centrioles stabilized p53, which turned on p21 and drove
differentiation. The other was p53-independent: OCT4 and NANOG proteins
were degraded faster by the proteasome; MG132 --- a proteasome inhibitor
--- partially rescued their levels.

That experiment established that centrioles are required to maintain
pluripotency in cells that already possess it. It says nothing about
whether centrioles are required to acquire pluripotency in cells that do
not. That is the question the proposed experiment will answer.

### 2.6. Centrosome-to-Senescence

Robichaud and colleagues (2024, Nature Communications) traced a direct
molecular chain from centrosomal proteins to cellular senescence. DNA
damage triggers KIFC3, a minus-end-directed kinesin, to build transient
microtubule bundles that physically connect the nucleus to the primary
cilium. These bundles require TTLL5 and TTLL6 --- the same enzymes that
CEDAR implicates in centriolar polyE accumulation. Knock down KIFC3,
TTLL5, or TTLL6, and senescence initiation is suppressed. The scaffold
for the apparatus is ODF2, also called cenexin, which sits at the distal
appendages of the mother centriole.

This is the first causal pathway traced from a centrosomal
post-translational modification through a specific motor protein to a
cell-fate decision --- senescence. It gives CEDAR a concrete molecular
mechanism.

### 2.7. Three Proteins on the Mother Centriole

Three proteins anchored at the mother centriole's appendages have been
independently shown to influence cell fate. They are not hypothetical;
they have been localized, knocked out, and functionally characterized.

ODF2/cenexin sits at the distal appendages and organizes the
nucleus-to-cilium signaling axis that triggers senescence \[21\]. ODF2
is essential for the formation of both distal and subdistal appendages;
Odf2-knockout cells lack appendages entirely and cannot form primary
cilia \[7\]. AKNA occupies the subdistal appendages and is both
necessary and sufficient for the delamination of neural progenitors ---
an EMT-like process that physically removes a cell from the
neuroepithelium (Camargo Ortega et al., 2019, Nature). NANOG, best known
as a pluripotency transcription factor, also localizes to the mother
centriole, where it associates with centriole maturation; this has been
observed in eleven cell lines, including normal KF1 fibroblasts \[16\].

The coordinated loss of these three proteins upon centriole elimination
--- whether or not they are organized into a unitary CAMC --- is likely
to be the key molecular event in whatever effect the experiment detects.

### 2.8. The Germline Precedent

Primordial germ cells maintain PLK4 mRNA at eight to eleven times the
level found in somatic cells. Phan and colleagues (2022, Genes and
Development) showed that this excess is held in check by an upstream
open reading frame in the PLK4 transcript that blocks translation of the
main coding sequence. Disrupt the uORF, PLK4 protein floods the cell,
centrioles amplify catastrophically, and the germ cells die. The
germline has evolved specialized machinery to prevent centriole
abnormalities --- it knows, in an evolutionary sense, that centriole
number must be tightly controlled for the totipotent lineage to survive.

## 3. Three Ways to Remove Centrioles

The methods available for eliminating centrioles differ in one crucial
respect: what else they remove. This difference provides the
experimental handle for testing whether CAMC exists.

### 3.1. The Methods

Physical ablation with a laser removes the centriole, the surrounding
pericentriolar material, and whatever proteins are assembled at the
appendages --- including CAMC, if CAMC exists. The method is clean but
slow: roughly one cell per minute.

Centrinone, the PLK4 inhibitor developed by Wong and colleagues (2015,
Science), prevents centriole duplication. Existing centrioles persist.
Over two to three cell cycles they are diluted by division. Their
appendage proteins --- and CAMC, if it is there --- remain in place on
the surviving centrioles throughout. The same holds for genetic methods:
PLK4 siRNA, PLK4 shRNA, SAS6 CRISPR-KO.

GT335, a monoclonal antibody against polyglutamylated tubulin,
disassembles centriolar microtubules when loaded into cells. Bobinnec
and colleagues (1998, Journal of Cell Biology) first demonstrated the
effect. The PCM disperses with the microtubules. What happens to
appendage proteins is unknown.

### 3.2. What the Comparison Reveals

The logic is straightforward. If physical ablation --- which takes CAMC
with it --- raises reprogramming efficiency, while centrinone --- which
blocks duplication but leaves CAMC in place --- does not, then CAMC is a
real, functionally significant structure. It is the barrier, not the
centriole's microtubules.

If centrinone and laser ablation produce the same result, the effect
runs through the centriole cylinder itself --- its polyE-decorated
microtubules --- rather than through any signaling complex assembled on
its surface.

If GT335 loading yields an intermediate result, CAMC may be partially
anchored on the microtubules and partially in the PCM. Alternatively,
GT335 --- which depolymerizes centriolar microtubules without the
physical trauma of laser ablation --- may reveal a contribution of
cytoskeletal mechanics to the barrier: the rigid centriole cylinder
could itself impede the cytoplasmic reorganization that MET requires,
independent of any signaling complex.

One complication is that centrinone removes centrioles, but
pericentriolar material --- γ-tubulin, pericentrin, CDK5RAP2 --- can
persist in the cytoplasm as acentriolar clusters capable of organizing
microtubules and influencing signaling independently of the centriole
cylinder. Laser ablation removes both centriole and associated PCM. The
comparison therefore tests "centriole + PCM removed" versus "centriole
removed, PCM possibly retained," not simply "CAMC absent versus CAMC
present." Immunostaining for pericentrin and CDK5RAP2 at day 0 and day 3
after each elimination method will determine whether PCM dispersal
correlates with the observed effect. If it does, PCM --- not CAMC ---
may be the relevant signaling hub. CEP192 depletion disrupts
γ-TuRC-dependent microtubule nucleation but does not strip PCM entirely.
A more stringent test would use PCNT (pericentrin) knockdown, which
destabilizes PCM broadly, or anti-γ-tubulin antibody nanoinjection.
These are noted as refinements for future experiments rather than
requirements for the initial screen. If CEP192-KD reproduces the
centrinone effect, the barrier likely involves PCM; if it does not, CAMC
anchored at the appendages becomes the leading candidate.

This three-way comparison is the strongest mechanistic test the
experiment can perform, but its interpretation must account for the PCM
variable.

## 4. The Experiment

### 4.1. Basic Design

The experiment uses p53-knockout BJ human foreskin fibroblasts, with
SB203580 added to block the p38 stress kinase. Wild-type fibroblasts
arrest in G1 after centriole loss; the double blockade is necessary to
let the cells keep dividing through the reprogramming window \[23,26\].

Centriole elimination is carried out by one of three methods.
Elimination is verified by immunofluorescence for CP110 and Cep135; more
than 90% of cells must be centriole-negative before OSKM is delivered.
Sendai virus (Cytotune 2.0) delivers the factors. Reprogramming
efficiency is scored as the percentage of TRA-1-60-positive colonies at
days 21 to 28.

### 4.2. The Thirteen Groups

  ------------------------------------------------------------------------
  Group   Treatment                        What it tests
  ------- -------------------------------- -------------------------------
  1       DMSO → OSKM                      Baseline efficiency

  2       Centrinone 500 nM × 3d → OSKM    Does centriole loss affect
                                           reprogramming?

  3       Centrinone + p53-KO + SB203580 → Is the effect independent of
          OSKM                             p53/p38 stress?

  4       MLN8237 10 nM → OSKM             Aurora A off-target control
                                           \[12\]

  5       KIFC3-KD → OSKM                  Nucleus-to-cilium senescence
                                           pathway

  6       Odf2-KO → OSKM                   Centriolar fate-switch protein

  7       IFT88 siRNA → OSKM               Cilium loss without centriole
                                           loss

  8       Centrinone + MG132 → OSKM        OCT4/NANOG proteostasis \[20\]

  9       SAS6 CRISPR-KO → OSKM            Genetic (non-pharmacological)
                                           elimination

  10      Laser ablation, both centrioles  Physical removal of centriole +
          → OSKM                           CAMC

  11      Laser ablation, mother only →    Asymmetric inheritance test
          OSKM                             

  12      Centrinone washout → OSKM        Reversibility

  13      Centrinone + PCM dispersal       PCM vs centriole discrimination
          (CEP192-KD) → OSKM               
  ------------------------------------------------------------------------

### 4.3. What Gets Measured

The primary endpoint is the fraction of input cells that form
TRA-1-60-positive colonies. Secondary measurements include pluripotency
markers (OCT4, NANOG, SOX2 by qPCR and Western blot at days 7, 14, and
21); apoptosis (Annexin V/PI at days 3 and 7); cell-cycle kinetics
(continuous EdU labeling); MET progression (E-cadherin, N-cadherin,
Snail, Zeb1 at days 3, 5, and 7); centriole status (CP110, Cep135,
Centrin immunofluorescence at days 0, 7, 14, and 21); p53 and p21
levels; phosphorylated p38; karyotype at day 28; teratoma formation in
SCID mice; and trilineage differentiation potential.

### 4.4. Five Hypotheses the Experiment Distinguishes

  -------------------------------------------------------------------------
  Hypothesis        Mechanism                      Decisive comparison
  ----------------- ------------------------------ ------------------------
  **H1: CEDAR**     The centriole is an entropy    Laser outperforms
                    carrier. CAMC and polyE        centrinone; mother
                    maintain differentiation.      ablation outperforms
                    Removal resets the cell.       daughter ablation

  **H2: Stress**    Centriole loss triggers        p53-WT + p38i control;
                    p53/p38. Reprogramming         effect disappears
                    proceeds only when these       without knockout
                    pathways are blocked.          

  **H3: Cilium**    Centriole loss removes the     IFT88 siRNA (group 7)
                    primary cilium, disrupting     reproduces the
                    Wnt, Hedgehog, and TGF-β       centrinone effect
                    signaling.                     

  **H4: Signaling   Centriole loss removes Aurora  MLN8237 or KIFC3-KD
  platform**        A, KIFC3, Odf2, and NANOG ---  reproduces the
                    specific proteins that         centrinone effect
                    regulate MET and cell cycle.   

  **H5:             The barrier resides in         CEP192-KD (group 13)
  PCM-dependent**   pericentriolar material        reproduces the
                    (γ-tubulin, pericentrin,       centrinone effect
                    CDK5RAP2), not in the          
                    centriole per se. PCM          
                    regulates Wnt/β-catenin        
                    independently of the centriole 
                    cylinder.                      
  -------------------------------------------------------------------------

### 4.5. Controls That Cannot Be Skipped

**TRIM37.** Meitinger and colleagues (2020, Nature) showed that TRIM37
status determines how cells respond to PLK4 inhibition: amplification of
the 17q23 locus sensitizes cells; deletion of TRIM37 confers resistance
through ectopic PCM foci that compensate for the absent centrioles. The
TRIM37 status of BJ-hTERT fibroblasts has not been documented. It must
be measured by Western blot and RT-qPCR before the experiment begins. If
TRIM37 is low, the alternative PLK4 inhibitor CFI-400945 should be used.

**53BP1 and USP28.** Fong and colleagues \[12\] and Meitinger and
colleagues (2016, Journal of Cell Biology) identified a centriole-loss
sensor that operates independently of p53 and p38. Even in p53-knockout
cells, 53BP1 and USP28 can detect the absence of centrioles and trigger
stress responses. Western blot for 53BP1 foci at day 0 and day 3 is
required.

**De novo centriole assembly.** La Terra and colleagues (2005) showed
that HeLa cells can assemble centrioles from scratch after laser
ablation. Lindhout and colleagues (2021) showed that centrioles recover
after centrinone-B washout in neural stem cells. If this occurs during
the reprogramming window, the experiment becomes uninterpretable. The
rule: if more than 10% of cells in any group regain CP110-positive
centrioles by day 5 --- or more than 20% by day 7 --- that group is
excluded. A more rigorous alternative is FACS-sorting of
centriole-negative cells (using Centrin-GFP or CP110 immunostaining)
immediately before OSKM delivery, ensuring a uniformly acentriolar
starting population. Continuous centrinone through day 21 should be
tested as a separate condition.

**Statistical power.** The Bonferroni correction for thirteen groups
pushes the significance threshold to roughly α ≈ 0.004. A power
calculation illustrates what this means in practice. Assuming a baseline
reprogramming efficiency of 0.2% in the DMSO control and an expected
4-fold increase to 0.8% in the centrinone group, with α = 0.004 and 80%
power, a two-proportion z-test requires approximately 8 × 10⁴ cells per
group. At 1 × 10⁵ cells per group the design is adequately powered.
Benjamini-Hochberg FDR control is less conservative and maintains
adequate power at 5 × 10⁴ cells per condition. For the laser ablation
groups (10, 11), where throughput limits N to roughly 10³ cells, only
large effects --- more than a 10-fold change --- will be detectable;
these groups should be interpreted as qualitative mechanistic tests
rather than quantitative comparisons.

## 5. What the Experiment Could Find

### 5.1. Reprogramming Increases

The centriole is a barrier. CAMC is real. The organelle level is a
previously unrecognized axis of reprogramming control, complementary to
epigenetics.

The strongest version of this outcome is method-dependent: laser
ablation outperforms centrinone, and mother-only ablation outperforms
daughter-only ablation. That pattern would provide the first functional
evidence for a centriole-associated memory complex.

Ohmine and colleagues (2018) showed that iPSCs with centrosome
amplification produce aggressive, metastatic teratomas. iPSCs derived
from centriole-free fibroblasts must be karyotyped, scored for
centrosome number, and tested in teratoma assays before any claim of
pluripotency can be made.

### 5.2. Reprogramming Decreases

Centrioles are required. The correlation between centriole loss and
differentiation that runs through the comparative literature is not
causal in the direction CEDAR predicted: differentiation may cause
centriole loss, not the reverse. CEDAR would need reformulation.

The follow-up question is practical: which centriole function is
essential? The MTOC? The cilium? A specific signaling protein? The
control groups in this design --- IFT88, KIFC3-KD, Odf2-KO --- are
positioned to answer it.

### 5.3. No Change

Centrioles are irrelevant. Other barriers dominate. Before accepting
this conclusion, one must verify that centrioles were genuinely absent
throughout the reprogramming window. De novo assembly is fast;
continuous monitoring is essential.

### 5.4. Method-Dependent Results

This is the most mechanistically informative outcome and the most
complex to interpret. If physical and chemical elimination produce
different effects, the nature of the difference points to the mechanism.

Laser succeeds, centrinone fails → CAMC is real and is the barrier.
Laser fails, centrinone succeeds → the centriole's microtubules are the
barrier; CAMC is either irrelevant or protective. GT335 intermediate →
CAMC is partially microtubule-anchored.

## 6. Discussion

### 6.1. Why Has This Not Been Done?

The experiment requires something that normal human cells cannot
tolerate --- centriole loss --- combined with something that centriole
loss normally prevents --- continued division. Getting past this
contradiction requires a p53 knockout and a p38 inhibitor. The knockout
is straightforward, but the conceptual jump --- that removing a stress
pathway reveals a reprogramming barrier rather than simply masking
toxicity --- is not obvious. A second factor is the breadth of
literature the hypothesis draws on: C. elegans embryology, planarian
stem cells, Drosophila oogenesis, mammalian immunology, and human
pluripotent stem cell biology. Few laboratories sit at the intersection
of these fields.

### 6.2. Slow-Renewing Structural Memory and Epigenetic Memory

Epigenetic reprogramming resets the cell's chemical memory. DNA
methylation, histone modifications, and chromatin organization are all
mutable states --- written and erased by enzymes. The centriole is
different in kind, not merely in degree. It is a structured protein
assembly whose tubulin accumulates polyglutamylation over time through
TTLL5 and TTLL6. This modification is not rapidly reversed; the
centriole has no known dediglutamylase activity comparable to the
demethylases and deacetylases that maintain epigenetic plasticity. In
this sense the centriole functions as a slow-renewing structural domain
--- a form of molecular memory encoded in post-translational
modifications rather than in nucleic acid chemistry. If CEDAR is right,
full reprogramming requires both the erasure of epigenetic marks, which
Yamanaka factors provide, and the elimination of the centriole's
accumulated PTM record, which only organelle removal can achieve. The
germline performs both: methylation is erased and centrioles are
eliminated during meiosis. Somatic reprogramming currently accomplishes
only the first. This may explain why standard iPSCs reach pluripotency
but remain short of the plasticity that the germline achieves.

### 6.3. Safety

iPSCs derived from centriole-free cells will eventually regenerate their
centrioles de novo. Whether these de novo centrioles are normal ---
correct number, proper appendage structure, functional ciliogenesis ---
is an open question. The experiment must include karyotyping, centrosome
counting by Centrin and CP110 immunofluorescence, and teratoma assays
that look beyond trilineage differentiation to cellular atypia and
aggressive growth patterns.

### 6.4. Limitations

The experiment is confined to p53-knockout cells. There is no way around
this --- wild-type cells arrest --- but it means the result is always
conditional on p53 status. Centrinone at high concentrations begins to
inhibit PLK1 (Wong et al., 2015); dose-response curves and a PLK1
inhibitor control are necessary. Laser ablation throughput is low;
statistical power for groups 10 and 11 will be limited. Results in BJ
fibroblasts may not generalize; parallel experiments in IMR-90, RPE1,
and HMEC cells would strengthen the conclusions but raise costs
considerably.

### 6.5. Beyond Reprogramming

If centrioles carry a differentiation-maintenance signal that
accumulates over time, the age of a tissue's stem cell pool should
correlate with the polyE load on its centrioles. Measuring GT335
fluorescence intensity as a function of donor age would provide a
correlational test. Interventions that slow polyE accumulation --- TTLL5
or TTLL6 inhibitors, for instance --- could, in principle, extend the
functional lifespan of somatic stem cells.

## 7. Conclusion

The experiment described here has never been done. It is technically
feasible. The reagents are commercially available. A systematic search
of the literature --- sixty verified PMIDs --- and a personal
communication from the field's leading laboratory confirm the gap.

The converging evidence from five phyla, from asymmetric centrosome
inheritance, from Renzova's demonstration that centriole loss triggers
differentiation, and from Robichaud's discovery that the centrosome
directly initiates senescence, all point to a single question: can a
somatic cell without centrioles be reprogrammed to pluripotency?

The question is fundamental. It is answerable. It is time to answer it.

## Acknowledgments

I thank Pierre Gönczy (Swiss Institute for Experimental Cancer Research,
EPFL, Lausanne) for providing the Kalbfuss and Gönczy (2023) paper ahead
of publication and for confirming that the proposed experiment does not
appear in the literature. Every PMID cited here has been verified
against PubMed and CrossRef.

## References

\[1\] Barandun N, Meier B, Stehli G, et al. Targeted localization of the
mother centrosome in CD8+ T cells undergoing asymmetric cell division
promotes memory formation. Cell Rep. 2025;44(1):115127. PMID: 39764850.

\[2\] Bonente D, et al. Inactivation and elimination of centrioles in
Drosophila development. Cells. 2025;14(12):865. PMID: 40558492.

\[3\] Camargo Ortega G, Falk S, Johansson PA, et al. The centrosome
protein AKNA regulates neurogenesis via microtubule organization.
Nature. 2019;567(7746):113--117. PMID: 30787442.

\[4\] Croisier M, Gönczy P. Electron microscopy confirms that only seven
cells retain centrioles in C. elegans L1 larvae. MicroPubl Biol. 2025.
PMID: 40475707.

\[5\] Fong CS, Mazo G, Das T, et al. 53BP1 and USP28 mediate
p53-dependent cell cycle arrest in response to centrosome loss and
prolonged mitosis. eLife. 2016;5:e16270. PMID: 27371829.

\[6\] Guguin J, Besson A, Nait Atmane S, et al. Generation of the iPSC
line CRNLi001-A from a patient with microcephaly and harbouring the most
recurrent RTTN variant. Stem Cell Res. 2026. PMID: 41719742.

\[7\] Ishikawa H, Kubo A, Tsukita S, Tsukita S. Odf2-deficient mother
centrioles lack distal/subdistal appendages and the ability to generate
primary cilia. Nat Cell Biol. 2005;7(5):517--524. PMID: 15852003.

\[8\] Kalbfuss N, Gönczy P. Extensive programmed centriole elimination
unveiled in C. elegans embryos. Sci Adv. 2023;9(22):eadg8682. PMID:
37256957.

\[9\] Kalbfuss N, Gönczy P. Towards a molecular architecture of
centriole elimination. Open Biol. 2023;13(11):230222. PMID: 37963546.

\[10\] La Terra S, English CN, Hergert P, et al. The de novo centriole
assembly pathway in HeLa cells. J Cell Biol. 2005;168(5):713--722. PMID:
15738265.

\[11\] Li Y, et al. Characterisation of centriole biogenesis during
multiciliation in planarians. Biol Cell. 2020;112(12):398--408. PMID:
32776587.

\[12\] Li Z, Rana TM. A kinase inhibitor screen identifies
small-molecule enhancers of reprogramming and iPS cell generation. Nat
Commun. 2012;3:1085. PMID: 23011139.

\[13\] Lindhout FW, Kooistra R, Portegies S, et al. Centrosome-mediated
microtubule remodeling during axon formation in human iPSC-derived
neurons. EMBO J. 2021;40(10):e106798. PMID: 33835529.

\[14\] Meitinger F, Ohta M, Lee KY, et al. TRIM37 controls
cancer-specific vulnerability to PLK4 inhibition. Nature.
2020;585(7825):440--446. PMID: 32908304.

\[15\] Meitinger F, Anzola JV, Kaulich M, et al. 53BP1 and USP28 mediate
p53 activation and G1 arrest after centrosome loss or extended mitotic
duration. J Cell Biol. 2016;214(2):155--166. PMID: 27432897.

\[16\] Mikulenkova E, et al. NANOG/NANOGP8 localizes at the centrosome
and is spatiotemporally associated with centriole maturation. Cells.
2020;9(3):692. PMID: 32168958.

\[17\] Ohmine S, Squillace KA, Hartjes KA, et al. Aurora-A
overexpression is linked to development of aggressive teratomas derived
from human iPS cells. Oncol Rep. 2018;39(4):1725--1730. PMID: 29393405.

\[18\] Phan TP, Boateng KA, Mitchell J, et al. Upstream open reading
frames control PLK4 translation and centriole duplication in primordial
germ cells. Genes Dev. 2022;36(11--12):718--736. PMID: 35772791.

\[19\] Pimenta-Marques A, Bento I, Lopes CAM, et al. A mechanism for the
elimination of the female gamete centrosome in Drosophila melanogaster.
Science. 2016;353(6294):aaf4866. PMID: 27229142.

\[20\] Renzova T, Bohaciakova D, Esner M, et al. Inactivation of
PLK4-STIL module prevents self-renewal and triggers p53-dependent
differentiation in human pluripotent stem cells. Stem Cell Reports.
2018;11(4):959--972. PMID: 30197118.

\[21\] Robichaud JH, Zhang Y, Chen C, et al. Transiently formed
nucleus-to-cilium microtubule arrays mediate senescence initiation in a
KIFC3-dependent manner. Nat Commun. 2024;15:7954. PMID: 39266565.

\[22\] Royall LN, et al. Asymmetric centrosome inheritance maintains
neural progenitor identity in the developing human neocortex. eLife.
2023;12:e83157. PMID: 37882444.

\[23\] Srsen V, Gnadt N, Dammermann A, Merdes A. Inhibition of
centrosome protein assembly leads to p53-dependent exit from the cell
cycle. J Cell Biol. 2006;174(5):625--630. PMID: 16943179.

\[24\] Takahashi K, Yamanaka S. Induction of pluripotent stem cells from
mouse embryonic and adult fibroblast cultures by defined factors. Cell.
2006;126(4):663--676. PMID: 16904174.

\[25\] Tkemaladze J. Reduction, proliferation, and differentiation
defects of stem cells over time: a consequence of selective accumulation
of old centrioles in the stem cells? Mol Biol
Rep. 2023;50(3):2751--2761. PMID: 36583780.

\[26\] Uetake Y, Loncarek J, Nordberg JJ, et al. Cell cycle progression
and de novo centriole assembly after centrosomal removal in
untransformed human cells. J Cell Biol. 2007;176(2):173--182. PMID:
17227892.

\[27\] Wong YL, Anzola JV, Davis RL, et al. Reversible centriole
depletion with an inhibitor of Polo-like kinase 4. Science.
2015;348(6239):1155--1160. PMID: 25931445.

\[28\] Yamashita YM, Mahowald AP, Perlin JR, Fuller MT. Asymmetric
inheritance of mother versus daughter centrosome in stem cell division.
Science. 2007;315(5811):518--521. PMID: 17255513.

\[29\] Zhao X, et al. PCM1 conveys centrosome asymmetry to polarized
endosome dynamics in regulating daughter cell fate. Nat Commun.
2025;16(1):10728. PMID: 41315244.
