# AICoordinator — UPGRADE

## U1. Multi-rig coordination
- One AIC instance coordinates N microscopes in parallel
- Essential step toward FCLC federated lineage atlas

## U2. Self-updating policy
- AIC reads outcomes of past runs → proposes PROMPT.md diffs for human review
- Version-controlled policy evolution

## U3. Fine-tuned local LLM fallback
- Distill PROMPT.md-driven behaviour into a local 7B–14B model
- Eliminates API latency + cloud dependency for decision loop

## U4. RLHF from expert annotations
- Dr. Tkemaladze labels "good" vs "bad" decisions on replayed runs
- Use DPO to refine the decision policy

## U5. Integration with CDATA
- AIC queries CDATA's biological-age model in real time to tag cells as "old-centriole equivalents"
- Closes loop between theory (CDATA) and experimental observation (tree)

## U6. Natural-language interface for lab staff
- Staff can type "next experiment: knock out CEP152, track for 48 h, abort if division rate < 0.3/day"
- AIC generates full PROMPT + useq-schema YAML

## U7. Ethical / safety board
- Formal policy-approval workflow for risky experiments
- PROMPT.md changes require sign-off from PI + safety officer

## U8. Scientific-reasoning agent
- AIC drafts results sections + figures directly from event logs
- Auto-composes methods paragraph with correct parameters from PARAMETERS.md chain
