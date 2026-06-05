# Ответ Afaf — черновик (3 июня 2026)

**Тема:** Re: HAP/NHAM simulation — next steps

Dear Afaf,

Thank you so much for your thoughtful response. I am very glad that you see the potential in the nonlinear dynamics direction — your feedback means a lot, and I agree with every point you raised.

Let me address them one by one.

### 1. Simulation consistency vs empirical validation

You are absolutely right. I will present the simulations as a **formalisation of the hypothesis** — a proof that the assumed mechanisms are sufficient to produce the observed dynamics, not as an empirical confirmation. The distinction will be made explicit in the manuscript.

### 2. HAP Strong vs graded formulation

This is an important point. I see them as **two levels of analysis** that do not conflict:

- **HAP (evolutionary):** The published paper (Longevity Horizon, 2024, 2(4), DOI: 10.65649/d76f6c48) argues, based on a 56-taxa meta-analysis, that **no evolved affective circuitry exists in Bilateria without a hepatic organ**. This is a phylogenetic necessary-condition claim — the organ had to be present during the evolution of the circuitry.

- **NHAM (mechanistic):** The dynamical model describes how the liver **modulates** affective states in real time. Here, a graded formulation is indeed more appropriate: hepatic signals are crucial modulators, not binary switches.

The strong claim belongs to the evolutionary paper (already published). For the mechanistic model, I fully agree: "hepatic signals as important modulators of affective regulation" is the correct framing.

### 3. Minimal causal architecture

Agreed — before adding DDEs or SDEs, we should understand the minimal system. I suggest we:
1. Run a **global sensitivity analysis** (Sobol method) on all parameters
2. Identify which parameters and feedback loops are **necessary** vs **redundant**
3. Reduce the model to the minimal set of variables and couplings
4. Only then consider increasing complexity (DDEs for transport delays, SDEs for noise)

### 4. Global sensitivity analysis

I have already prepared a Sobol sensitivity analysis workflow using SALib. It will quantify the contribution of each parameter to the variance of each state variable — exactly what we need to identify the essential mechanisms. I can share the results once the analysis is complete.

### Next steps

I suggest:
1. **Sensitivity analysis** — I will run it this week and share the results
2. **Call** — after you have explored the model and I have the sensitivity results, we can discuss the minimal architecture and agree on the formalism
3. **Joint manuscript outline** — once the model structure is finalised

Would you be available for a call in the second half of June?

Looking forward to your detailed comments when you are ready.

Warm regards,
Jaba
