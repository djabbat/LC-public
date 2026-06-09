# Ответ Afaf — о платформе и макробиоме (9 июня 2026)

**Date:** 2026-06-09
**To:** Afaf El Fettahi
**Subject:** Re: Your questions about the GLA platform — and a collaboration with Gou et al. (2026)

---

Dear Afaf,

Thank you for your thoughtful and generous response. Your question about nested layers rather than competing explanations is exactly right — more on that below. And thank you for the interest in the platform. Let me answer your questions as honestly as I can.

## 1. Microbiome and macrobiome: fierce competitors for the predictive apparatus

Your intuition about nested layers is close — but I have been thinking more deeply, and I now believe the relationship is fundamentally **competitive**, not nested.

The core insight is this: both the microbiome and the macrobiome are forces that seek to control the organism's behaviour. They do so by shaping the organism's **world model** — the predictive apparatus through which the organism anticipates events and selects actions.

- **Macrobiome** (language, culture, social structures) presses from *outside*. It demands that the organism adopt specific predictive frameworks — grammatical structures, social scripts, causal narratives — that enable coordinated action in a social world.
- **Microbiome** (gut microbiota, metabolites, immune signals) presses from *inside*. It modulates the interoceptive and affective channels through which the organism experiences its own internal state, thereby biasing which predictions feel urgent and which actions feel rewarding.

These two forces **compete fiercely** for the predictive apparatus of the organism. The organism's cognitive functions — attention, memory, reasoning, affect — are the *result* of this competition. Neither force alone constructs cognition; cognition emerges from the unresolved tension between them.

### The sleep–wake cycle as the arena of competition

The competition is mediated by the sleep–wake cycle:

- **During wakefulness**, the macrobiome dominates. The organism must act in a social and physical environment, and the macrobiome's predictive frameworks — language, cultural norms, task demands — restructure the predictive apparatus in its favour. The organism's world model collapses into a specific, action-oriented configuration.

- **During sleep**, the microbiome dominates. Freed from the demands of the macrobiome, the organism's predictive apparatus enters a regime where the microbiome can modify the set of world models that were *not* active during wakefulness. The microbiome explores alternative predictive configurations, consolidates metabolic memories, and recalibrates the interoceptive baselines that will shape tomorrow's affect and motivation.

This is, at its core, **physics**. Consider the double-slit experiment:

- Without measurement (sleep-like), the particle exists in a superposition of trajectories — like the microbiome exploring multiple possible world models simultaneously.
- With measurement (wakefulness-like), the superposition collapses into a single trajectory — like the macrobiome forcing a specific, actionable prediction.
- The organism is the quantum system; the microbiome and macrobiome are the competing "observers" that vie for the right to collapse the wave function.

The cognitive functions we observe — what we call "thought," "emotion," "decision" — are the measurable outcomes of this ongoing competition. They are not properties of the organism alone, nor of the microbiome alone, nor of the macrobiome alone. They are the interference pattern produced by the clash of internal and external predictive forces.

This reframes Gou et al.'s argument. They are right that the microbiome is a constitutive co-constructor of cognition — but they miss that it is a *competitor* in a larger arena. The macrobiome is not merely another layer; it is the opposing force in the same predictive architecture. A complete theory of embodied cognition must describe the dynamics of this competition — the conditions under which the microbiome wins, the macrobiome wins, or a stalemate (pathology?) emerges.

## 2. The platform — honest answers

I appreciate your detailed questions, and I want to be straightforward about where we are.

### How far along is the development?

The platform exists in three stages of realisation:

**Computational (ready):** The HAP/NHAM nonlinear dynamics model is operational — 6 coupled ODEs (L, B, A, I, S, M — liver steroid output, brain sensitivity, affective integrity, inflammation, stress axis, metabolism) with feedback loops. Sensitivity analysis (Morris + Sobol) has been completed; the results strongly confirm that liver parameters dominate the variance of affective output. The code and visualisations exist and I can share them.

**Data pipeline (in development):** Through our BioSense subproject, we have EEG/HRV analysis pipelines (Ze band-wise analysis) validated on open datasets (LEMON, Cuban, Dortmund). This provides the electrophysiological readout layer.

**Physical (conceptual/design phase):** The whole-body *in vitro* platform — oral cavity, urogenital system, small intestine, large intestine — is currently in conceptual design. We are modelling it after existing organ-on-a-chip and multi-compartment bioreactor systems (e.g., the HIMOX system for gut-liver axis, Wyss Institute gut-on-a-chip). The innovation is the tight coupling with a computational digital twin and the inclusion of all four compartments rather than a single organ.

### Physical prototypes?

No physical prototypes are operational yet. The computational and data pipeline components are operational; the physical simulator is at the detailed design stage. We are actively seeking collaboration and funding to move from design to prototype (this is part of our EIC Pathfinder application for October 2026).

### How do you model the transition from microbial/metabolic signals to neural dynamics?

The HAP/NHAM model provides the theoretical backbone for this. Our approach:

1. **Bottom layer — metabolic:** Microbial consortia produce metabolites (SCFAs, bile acids, indoles, GABA precursors) with known receptor affinities (FXR, TGR5, AhR, GABA receptors). These are modelled as coupled rate equations with Michaelis-Menten kinetics for enzyme-mediated conversions.

2. **Middle layer — receptor/immune:** Metabolite concentrations drive receptor activation and immune signalling (cytokine cascades, TLR signalling, vagal afferent firing rates). In the physical simulator, this layer would be measured via:
   - LC-MS/MS for real-time metabolomics in effluent from each compartment
   - Multiplex immunoassays for cytokine panels
   - Microelectrode arrays at epithelial-vagal interface points

3. **Top layer — neural dynamics:** We map metabolite/receptor states to neural circuit dynamics using a simplified Wilson-Cowan or mean-field model of the relevant brain regions (insula, anterior cingulate, amygdala, brainstem vagal nuclei). The key bridging variable is **vagal tone** — measurable via HRV — which serves as the physiological readout that couples gut state to brain state.

The full chain: **strain/metabolite → concentration → receptor occupancy → vagal firing rate → HRV → affective state** is the causal pathway we aim to trace.

### Electrophysiological readouts?

Three tiers, from most accessible to most ambitious:

1. **HRV (heart rate variability)** — RMSSD, SDNN, LF/HF ratio, and nonlinear measures (DFA α1, sample entropy). This captures vagal tone, which is the primary physiological bridge between gut and brain. We already have analysis pipelines for 24h Holter data through BioSense.

2. **EEG (resting state + evoked)** — specifically, we are interested in:
   - Frontal alpha asymmetry (FAA) as a correlate of affective/motivational state
   - Insular cortex activity via source-localised EEG (the insula is the primary interoceptive integration hub)
   - Late positive potential (LPP) to emotional stimuli — modulated by gut-derived serotonin and GABA precursors

3. **Microelectrode arrays (in vitro)** — in the physical simulator, we envision MEAs at the epithelial-vagal interface to directly record signalling from enteroendocrine cells (which express voltage-gated Na⁺/Ca²⁺ channels and fire action potentials in response to microbial metabolites). This is the most technically challenging but also the most innovative component.

### Is the platform primarily microbiome-host, or broader organism-level regulation?

Broader organism-level regulation. While the gut microbiome is the most developed application, the four-compartment design — oral, urogenital, small intestine, large intestine — is deliberately multi-organ because:

1. **Oral cavity:** Salivary hormones (cortisol, DHEA), oral microbiome, and taste/cephalic-phase responses. Critical for understanding top-down (brain→body) signalling.

2. **Urogenital system:** Sex steroid metabolism (oestrogen, testosterone), urogenital microbiome, and their bidirectional relationship with mood and cognition (the oestrogen-gut-brain axis is significantly understudied).

3. **Small + large intestine:** The canonical gut-brain axis — SCFA production, enteroendocrine signalling, immune activation, vagal afferent stimulation.

4. **Integration:** The liver (our core theoretical focus in HAP) is the metabolic hub that integrates signals from all compartments. The digital twin models hepatic processing of the combined metabolite flux.

So the platform is a *whole-body metabolic-neural interface simulator*, with the microbiome as a key input but not the only one. This broader scope is precisely why we see it as complementary to Gou et al.'s microbiota-centric framework — our platform provides the larger physiological context in which their constitutive hypothesis can be rigorously tested.

## 3. What I can share now

I am attaching (or can send upon your confirmation):

- **HAP/NHAM model report** — the sensitivity analysis results, ODE system, and visualisations (SUMMARY.md in our shared folder)
- **BioSense EEG/HRV pipelines** — open-source Python, validated on public datasets
- **Platform design document** — once I prepare a clean version for external review

Would it be helpful if I prepared a **technical white paper** describing the platform architecture in detail? I could have it ready within 2–3 weeks.

## 4. Next steps — including Gou et al. collaboration

I have drafted a letter to Qinglian Xie and Yu Wang (the corresponding authors of the Gou et al. paper) proposing a collaboration. The draft is attached — would you be comfortable with me sending it, with you cc'd? I believe your theoretical expertise in nonlinear dynamics would be a significant asset to such a collaboration, and I would be grateful to have you as a co-PI if this moves forward.

For our own work, I suggest:

1. **Call in late June** — to discuss the sensitivity analysis results and agree on the next steps for the HAP/NHAM manuscript
2. **Graded HAP model** — implement your suggestion of a modulated rather than binary hepatic requirement
3. **Platform white paper** — if you see value in it, I can prepare a formal description of the digital-physical twin architecture

Thank you again for your engagement — it is a genuine pleasure to work with someone who combines theoretical depth with practical curiosity.

Warm regards,
Jaba

---

**Jaba Tqemaladze, MD**
President, Georgia Longevity Alliance
jaba@longevity.ge | +995 555 185161
ORCID: 0000-0001-8651-7243
