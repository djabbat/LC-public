# LC — MAP (v7.1: фактическая структура)

**Дата:** 2026-07-14 (автоаудит)

## Фактическая структура (июль 2026)

```
LC/
├── CONCEPT.md  ← core
├── DESIGN.md  ← core
├── EVIDENCE.md  ← core
├── LICENSE  ← core
├── MAP.md  ← core
├── MEMORY.md  ← core
├── PARAMETERS.md  ← core
├── README.md  ← core
├── STATE.md  ← core
├── THEORY.md  ← core
├── TODO.md  ← core
├── _pi.md  ← core
├── BioSense/
│   ├── CONCEPT.md  ← core
│   ├── MAP.md  ← core
│   ├── MEMORY.md  ← core
│   ├── PARAMETERS.md  ← core
│   ├── README.md  ← core
│   ├── STATE.md  ← core
│   ├── TODO.md  ← core
│   ├── _pi.md  ← core
│   ├── Materials/
│   │   ├── Toward Integral Field Tomography of Living Systems.docx
│   │   ├── Twin Paradox Without Paradox.docx
│   │   └── Ze.docx
│   ├── _archive/
│   │   ├── CONCEPT.md.bak_dedupe
│   │   └── CONCEPT.md.pre-merge-2026-05-09
│   ├── audits/
│   │   └── 2026-05-08/
│   │       ├── LC_BioSense.check.v1.md
│   │       ├── LC_BioSense.plan.v1.md
│   │       └── LC_BioSense.review.md
│   ├── backend/
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   ├── deploy/
│   │   │   ├── scripts/
│   │   │   │   └── deploy.sh
│   │   │   └── systemd/
│   │   │       └── biosense-backend.service
│   │   └── src/
│   │       └── main.rs
│   ├── biosense-web/
│   │   ├── config/
│   │   │   └── runtime.exs
│   │   └── lib/
│   │       └── biosense_web_web/
│   │           ├── components/
│   │           └── live/
│   ├── data/
│   │   ├── ze_Elder_72.json
│   │   ├── ze_Middle_50.json
│   │   ├── ze_Young_25.json
│   │   ├── ze_age_comparison.png
│   │   ├── cuban/
│   │   │   ├── oldgandalf-FirstWaveCubanHumanNormativeEEGProject-3783da7/
│   │   │   │   ├── CONCEPT.md  ← core
│   │   │   │   ├── MAP.md  ← core
│   │   │   │   ├── MEMORY.md  ← core
│   │   │   │   ├── PARAMETERS.md  ← core
│   │   │   │   ├── README.md  ← core
│   │   │   │   ├── STATE.md  ← core
│   │   │   │   ├── TODO.md  ← core
│   │   │   │   ├── _pi.md  ← core
│   │   │   │   ├── EyesOpen/
│   │   │   │   ├── data/
│   │   │   │   └── results/
│   │   │   └── results/
│   │   │       ├── ze_cuban_ec.json
│   │   │       └── ze_cuban_lifespan.png
│   │   ├── dortmund/
│   │   │   ├── participants.tsv
│   │   │   └── results/
│   │   │       ├── ze_dortmund_combined.json
│   │   │       ├── ze_dortmund_results.png
│   │   │       ├── ze_sub-004.json
│   │   │       ├── ze_sub-018.json
│   │   │       ├── ze_sub-020.json
│   │   │       ├── ze_sub-042.json
│   │   │       ├── ze_sub-047.json
│   │   │       ├── ze_sub-057.json
│   │   │       ├── ze_sub-067.json
│   │   │       ├── ze_sub-099.json
│   │   │       ├── ze_sub-104.json
│   │   │       ├── ze_sub-112.json
│   │   │       ├── ze_sub-126.json
│   │   │       ├── ze_sub-131.json
│   │   │       ├── ze_sub-133.json
│   │   │       ├── ze_sub-142.json
│   │   │       ├── ze_sub-156.json
│   │   │       ├── ze_sub-163.json
│   │   │       ├── ze_sub-175.json
│   │   │       ├── ze_sub-179.json
│   │   │       ├── ze_sub-184.json
│   │   │       ├── ze_sub-209.json
│   │   │       ├── ze_sub-219.json
│   │   │       ├── ze_sub-224.json
│   │   │       ├── ze_sub-234.json
│   │   │       ├── ze_sub-244.json
│   │   │       ├── ze_sub-274.json
│   │   │       ├── ze_sub-278.json
│   │   │       ├── ze_sub-287.json
│   │   │       ├── ze_sub-296.json
│   │   │       ├── ze_sub-318.json
│   │   │       ├── ze_sub-329.json
│   │   │       ├── ze_sub-349.json
│   │   │       ├── ze_sub-359.json
│   │   │       ├── ze_sub-370.json
│   │   │       ├── ze_sub-381.json
│   │   │       ├── ze_sub-382.json
│   │   │       ├── ze_sub-387.json
│   │   │       ├── ze_sub-397.json
│   │   │       ├── ze_sub-407.json
│   │   │       ├── ze_sub-412.json
│   │   │       ├── ze_sub-433.json
│   │   │       ├── ze_sub-438.json
│   │   │       ├── ze_sub-454.json
│   │   │       ├── ze_sub-456.json
│   │   │       ├── ze_sub-470.json
│   │   │       ├── ze_sub-481.json
│   │   │       ├── ze_sub-488.json
│   │   │       ├── ze_sub-509.json
│   │   │       ├── ze_sub-511.json
│   │   │       ├── ze_sub-517.json
│   │   │       ├── ze_sub-522.json
│   │   │       ├── ze_sub-526.json
│   │   │       ├── ze_sub-529.json
│   │   │       ├── ze_sub-539.json
│   │   │       ├── ze_sub-550.json
│   │   │       ├── ze_sub-563.json
│   │   │       ├── ze_sub-578.json
│   │   │       ├── ze_sub-587.json
│   │   │       ├── ze_sub-590.json
│   │   │       ├── ze_sub-592.json
│   │   │       └── ze_sub-593.json
│   │   ├── lemon/
│   │   │   ├── participants.csv
│   │   │   ├── sub-032301.tar.gz
│   │   │   ├── sub-032302.tar.gz
│   │   │   ├── sub-032303.tar.gz
│   │   │   ├── sub-032305.tar.gz
│   │   │   ├── sub-032307.tar.gz
│   │   │   ├── sub-032309.tar.gz
│   │   │   ├── sub-032310.tar.gz
│   │   │   └── results/
│   │   │       ├── ze_alpha_peak.png
│   │   │       ├── ze_alpha_peak_results.json
│   │   │       ├── ze_bandwise_results.json
│   │   │       ├── ze_bandwise_young_vs_old.png
│   │   │       ├── ze_lemon_age_scatter.png
│   │   │       ├── ze_lemon_bandwise_final.png
│   │   │       ├── ze_lemon_combined.json
│   │   │       ├── ze_lemon_results.json
│   │   │       ├── ze_sub-032323.json
│   │   │       ├── ze_sub-032329.json
│   │   │       ├── ze_sub-032333.json
│   │   │       ├── ze_sub-032337.json
│   │   │       ├── ze_sub-032338.json
│   │   │       ├── ze_sub-032340.json
│   │   │       ├── ze_sub-032344.json
│   │   │       ├── ze_sub-032353.json
│   │   │       ├── ze_sub-032385.json
│   │   │       ├── ze_sub-032389.json
│   │   │       ├── ze_sub-032390.json
│   │   │       ├── ze_sub-032392.json
│   │   │       ├── ze_sub-032400.json
│   │   │       ├── ze_sub-032414.json
│   │   │       ├── ze_sub-032421.json
│   │   │       ├── ze_sub-032430.json
│   │   │       ├── ze_sub-032442.json
│   │   │       ├── ze_sub-032458.json
│   │   │       ├── ze_sub-032459.json
│   │   │       ├── ze_sub-032467.json
│   │   │       ├── ze_sub-032491.json
│   │   │       ├── ze_sub-032495.json
│   │   │       ├── ze_sub-032508.json
│   │   │       └── ze_sub-032525.json
│   │   └── zenodo/
│   │       └── results/
│   │           ├── ze_Subject_360.json
│   │           ├── ze_Subject_360.png
│   │           ├── ze_ec_eo_360.json
│   │           └── ze_ec_eo_360.png
│   ├── docs/
│   │   ├── PROJECT_AUDIT_2026-05-12.md
│   │   ├── related/
│   │   │   ├── parent_LongevityCommon_CONCEPT.md
│   │   │   ├── parent_LongevityCommon_EVIDENCE.md
│   │   │   ├── parent_LongevityCommon_OPEN_PROBLEMS.md
│   │   │   ├── parent_LongevityCommon_PARAMETERS.md
│   │   │   ├── parent_LongevityCommon_STATE.md
│   │   │   └── parent_LongevityCommon_THEORY.md
│   │   └── tbpr/
│   │       ├── article_2026-05-09.md
│   │       ├── engineering_2026-05-09.md
│   │       └── project_2026-05-09.md
│   ├── instruments/
│   │   └── automated-microscopy/
│   │       ├── CONCEPT.md  ← core
│   │       ├── DESIGN.md  ← core
│   │       ├── EVIDENCE.md  ← core
│   │       ├── MAP.md  ← core
│   │       ├── MEMORY.md  ← core
│   │       ├── PARAMETERS.md  ← core
│   │       ├── README.md  ← core
│   │       ├── STATE.md  ← core
│   │       ├── THEORY.md  ← core
│   │       ├── TODO.md  ← core
│   │       ├── _pi.md  ← core
│   │       ├── _archive/
│   │       │   └── DEEP_AUDIT_2026-04-21.md
│   │       └── docs/
│   │           ├── JOURNAL.md
│   │           ├── OPEN_PROBLEMS.md
│   │           └── ROADMAP.md
│   ├── refs/
│   │   ├── PMID_1385210_Christensen_1992_febslett_bovine_alpha_2_antiplasmin_n_termina.md
│   │   ├── PMID_14315085_HAYFLICK_1965_expcellres_the_limited_in_vitro_lifetime_of_hum.md
│   │   ├── PMID_20480236_Lezhava_2011_biogerontology_gerontology_research_in_georgia.md
│   │   ├── PMID_36583780_Tkemaladze_2023_molbiolrep_reduction_proliferation_and_differen.md
│   │   └── README.md  ← core
│   ├── results/
│   │   ├── ze_Elder_70.json
│   │   ├── ze_Middle_45.json
│   │   ├── ze_Young_25.json
│   │   └── ze_age_comparison.png
│   ├── scripts/
│   │   └── biosense.sh
│   └── src/
│       └── requirements.txt
├── FCLC/
│   ├── CONCEPT.md  ← core
│   ├── MAP.md  ← core
│   ├── MEMORY.md  ← core
│   ├── PARAMETERS.md  ← core
│   ├── README.md  ← core
│   ├── STATE.md  ← core
│   ├── TODO.md  ← core
│   ├── _pi.md  ← core
│   ├── audits/
│   │   └── 2026-05-08/
│   │       ├── LC_FCLC.check.v1.md
│   │       ├── LC_FCLC.plan.v1.md
│   │       └── LC_FCLC.review.md
│   └── fclc-web/
│       ├── assets/
│       │   └── css/
│       │       └── app.css
│       └── lib/
│           └── fclc_web_web/
│               └── live/
├── HAP/
│   ├── CONCEPT.md  ← core
│   ├── MAP.md  ← core
│   ├── MEMORY.md  ← core
│   ├── PARAMETERS.md  ← core
│   ├── README.md  ← core
│   ├── STATE.md  ← core
│   ├── TODO.md  ← core
│   ├── _pi.md  ← core
│   ├── Biomarker_Review/
│   │   ├── OSF_REGISTRATION.md
│   │   ├── PILOT_REPORT.md
│   │   ├── PROTOCOL_v2.3.docx
│   │   ├── PROTOCOL_v2.3.pdf
│   │   ├── PROTOCOL_v2.md
│   │   ├── answer_to_Afaf.md
│   │   ├── email_from_Afaf_2026-06-15.md
│   │   └── email_to_Afaf.md
│   ├── HAP_NHAM/
│   │   ├── SUMMARY.md
│   │   ├── letter_to_afaf_2026-06-03.md
│   │   ├── morris_results.png
│   │   ├── correspondence/
│   │   │   └── letter_to_afaf.md
│   │   └── results/
│   │       ├── morris.png
│   │       ├── sobol.png
│   │       └── sobol_results.json
│   ├── _archive/
│   │   └── hap_nham_sensitivity.py
│   ├── correspondence/
│   │   ├── 01_hevolution_inquiry.eml
│   │   ├── 02_impetus_norn_group_inquiry.eml
│   │   ├── 03_felipe_sierra_hevolution.eml
│   │   ├── CORREO DE INFORMACIÓN INICIAL DOCTORADO  DE PERIODO 2026-2027.pdf
│   │   ├── DOCTORADO información 26-27.pdf
│   │   ├── EDC_module_description.md
│   │   ├── Explicación acceso a la solicitud en español 2026.odt
│   │   ├── Explicación acceso a la solicitud en español 2026.pdf
│   │   ├── Información plazo 16 marzo - copia.pdf
│   │   ├── MCAOA_minimal_example.md
│   │   ├── Propuesta_v2_MCOA_Hipotesis.md
│   │   ├── TSMU_application_2026-05-13.docx
│   │   ├── TSMU_application_2026-05-13.pdf
│   │   ├── carta_motivacion_torres_ruiz_2026-05-22.md
│   │   ├── draft2digital_appeal_ze_theory.md
│   │   ├── draft2digital_refund_request.md
│   │   ├── elife_appeal_2026-05-22.md
│   │   ├── email_to_afaf_2026-05-30.md
│   │   ├── email_to_afaf_re_gou_collaboration.md
│   │   ├── email_to_xie_wang_gou_2026-06-09.md
│   │   ├── felipe_sierra_hevolution_2026-05-26.md
│   │   ├── gonczy_centriole_protocol.md
│   │   ├── hevolution_inquiry_2026-05-24.md
│   │   ├── hevolution_inquiry_v2_2026-05-26.md
│   │   ├── impetus_norn_group_inquiry_2026-05-24.md
│   │   ├── impetus_norn_group_inquiry_v2_2026-05-26.md
│   │   ├── informacion general 2026-2027.pdf
│   │   ├── propuesta_tesis_MCOA_EDC_TorresRuiz_2026-05-22.md
│   │   ├── response_to_Lala_BTHE_2026-07-02.md
│   │   ├── response_to_afaf_platform_2026-06-09.md
│   │   ├── response_to_gou_et_al_2026.md
│   │   ├── respuesta_uned_2026-05-19.md
│   │   ├── ruiz-moreno_rejection_2026-05-06.md
│   │   ├── torres_ruiz_followup_CEDAR_2026-05-26.md
│   │   ├── torres_ruiz_respuesta_2026-05-22.md
│   │   ├── torres_ruiz_respuesta_2026-05-26.md
│   │   ├── uned_coordinator_collado_2026-05-14.md
│   │   ├── uned_garciacesquinas_2026-05-14.md
│   │   ├── uned_instrucciones_formulario_2026-05-19.md
│   │   ├── uned_respuesta_docs_subir_solicitud_2026-05-25.md
│   │   ├── uned_seccion_masteres_2026-05-19.md
│   │   └── uned_translation_query.md
│   ├── docs/
│   │   ├── HAP_Dynamics.docx
│   │   ├── HAP_manuscript_v2_nham.docx
│   │   ├── HAP_manuscript_v2_nham.md
│   │   ├── Hepato-Affective Primacy (HAP) Theory and Neural-Hepatic Affective Model (NHAM).docx
│   │   ├── The+Hepato-Affective+Primacy+(HAP)+Theory.pdf
│   │   ├── alternative_journals.md
│   │   ├── apology_Chaos_Solitons_Fractals.md
│   │   ├── cover_letter_Medical_Hypotheses.md
│   │   ├── cover_letter_Psychoneuroendocrinology.md
│   │   ├── credit_statement.md
│   │   ├── dataset_comparison.md
│   │   ├── declaration_of_interest.md
│   │   ├── email_to_afaf_2026-05-30.md
│   │   ├── ethics_statement.md
│   │   ├── evidence_hap_confirmation.md
│   │   ├── hap_simulation_concept.md
│   │   ├── highlights.md
│   │   ├── letter_to_afaf.md
│   │   ├── manuscript_v2_nham.md
│   │   ├── manuscript_v2_nham_v2.md
│   │   ├── manuscript_v3_hap.md
│   │   ├── peer_review_round3_IF18.md
│   │   ├── title_page_Medical_Hypotheses.md
│   │   └── withdrawal_Medical_Hypotheses.md
│   ├── hap_simulation/
│   │   └── results/
│   │       ├── ablation_after_crit.png
│   │       ├── ablation_before_crit.png
│   │       ├── normal_trajectory.png
│   │       └── phase_portrait_A_vs_L.png
│   ├── manuscript_submission/
│   │   ├── HAP_Dynamics.docx
│   │   ├── HAP_Dynamics.md
│   │   ├── HAP_Dynamics_ANON.docx
│   │   ├── HAP_Dynamics_ANON.md
│   │   ├── HAP_Dynamics_Mathematical_Biosciences.docx
│   │   ├── HAP_Dynamics_RU.docx
│   │   ├── HAP_Dynamics_RU.md
│   │   ├── abstract_short.md
│   │   ├── cover_letter_BMB.docx
│   │   ├── cover_letter_BMB.md
│   │   ├── cover_letter_BSPC.md
│   │   ├── cover_letter_Chaos_Solitons_Fractals.docx
│   │   ├── cover_letter_Chaos_Solitons_Fractals.md
│   │   ├── cover_letter_Discover_Aging.docx
│   │   ├── cover_letter_JBD.docx
│   │   ├── cover_letter_JBD.md
│   │   ├── cover_letter_Mathematical_Biosciences.docx
│   │   ├── cover_letter_Mathematical_Biosciences.md
│   │   ├── cover_letter_Psychoneuroendocrinology.docx
│   │   ├── cover_letter_Psychoneuroendocrinology.md
│   │   ├── declaration_competing_interests.docx
│   │   ├── declaration_competing_interests.md
│   │   ├── highlights.docx
│   │   ├── highlights.md
│   │   ├── highlights_CSF.md
│   │   ├── title_page.docx
│   │   └── title_page.md
│   ├── refs/
│   │   ├── email_followup_2026-05-31.md
│   │   └── new_references_for_HAP.md
│   ├── results/
│   │   ├── ablation_after_crit.png
│   │   ├── ablation_before_crit.png
│   │   ├── bifurcation_L_basal.png
│   │   ├── bifurcation_L_basal_full.png
│   │   ├── bifurcation_all_params.png
│   │   ├── bifurcation_k_A_L.png
│   │   ├── bifurcation_tau_crit.png
│   │   ├── nham_vs_real_data.png
│   │   ├── normal_trajectory.png
│   │   └── phase_portrait_A_vs_L.png
│   └── sensitivity_results/
│       ├── 2d_scan_Lbasal_thetaL.png
│       ├── colored_noise.png
│       ├── comparison_A.png
│       ├── morris.png
│       ├── params.json
│       ├── phase_portraits.png
│       ├── sensitivity.json
│       ├── sobol.png
│       ├── stochastic.png
│       ├── stochastic_fixed.png
│       ├── stochastic_params.png
│       └── trajectory_normal.png
├── MCARA/
│   ├── CONCEPT.md  ← core
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── Centriole_Elimination_and_iPSC_Reprogramming.md
│   ├── Centriole_Elimination_iPSC_Reprogramming.docx
│   ├── Centriole_Elimination_iPSC_Reprogramming_EN.md
│   ├── DESIGN.md  ← core
│   ├── DID_Centricle_Dve_Linii.md
│   ├── EVIDENCE.md  ← core
│   ├── EXPERIMENTS.md
│   ├── LICENSE  ← core
│   ├── MAP.md  ← core
│   ├── MASTER.md  ← core
│   ├── MCARA.docx
│   ├── MCARA_CONCEPT_v4.5.md
│   ├── MCARA_CONCEPT_v4.5.pdf
│   ├── MCARA_Four_Counters.md
│   ├── MEMORY.md  ← core
│   ├── PARAMETERS.md  ← core
│   ├── README.md  ← core
│   ├── STATE.md  ← core
│   ├── THEORY.md  ← core
│   ├── TODO.md  ← core
│   ├── _pi.md  ← core
│   ├── ARGUS-LP/
│   │   ├── CONCEPT.md  ← core
│   │   ├── DESIGN.md  ← core
│   │   ├── EVIDENCE.md  ← core
│   │   ├── FILE_MAP.md  ← core
│   │   ├── MAP.md  ← core
│   │   ├── MEMORY.md  ← core
│   │   ├── PARAMETERS.md  ← core
│   │   ├── README.md  ← core
│   │   ├── STATE.md  ← core
│   │   ├── THEORY.md  ← core
│   │   ├── TODO.md  ← core
│   │   ├── _pi.md  ← core
│   │   └── docs/
│   │       ├── ARGUS-LP_BIOSAFETY_ANALYSIS_2026-07-11.md
│   │       ├── ARGUS-LP_Changelog_v1.0-to-v1.1_RU.md
│   │       ├── ARGUS-LP_Implementation_Plan.md
│   │       ├── ARGUS-LP_Technical_Specification_RU.md
│   │       ├── ARGUS-LP_Technical_Specification_RU_v1.1.md
│   │       ├── ARGUS-LP_Technical_Specification_RU_v1.2.md
│   │       ├── ARGUS-LP_Technical_Specification_v2.0_standalone.md
│   │       ├── ARGUS-LP_cover_letter_to_Alex_2026-06-29.md
│   │       ├── ARGUS-LP_v3_final_2026-06-27.md
│   │       ├── ARGUS_scheme.png
│   │       ├── email_Alex_v1.1_approval.md
│   │       └── correspondence/
│   │           ├── ARGUS-LP_v3_final_2026-06-27.md
│   │           ├── ARGUS_LP_DEEP_AUDIT_2026-06-27.md
│   │           ├── ARGUS_LP_answer_to_Alex_2026-06-26.md
│   │           ├── ARGUS_LP_comparison_all_versions_2026-06-27.md
│   │           ├── ARGUS_LP_comparison_old_vs_new_2026-06-27.md
│   │           ├── ARGUS_LP_comparison_slide.md
│   │           ├── ARGUS_LP_components_attachment_2026-06-27.md
│   │           ├── ARGUS_LP_followup_v3_2026-06-27.md
│   │           ├── ARGUS_LP_letter_to_Alex_2026-06-27.md
│   │           ├── ARGUS_LP_proposals_for_Alex.md
│   │           ├── ARGUS_LP_short_reply_to_Alex.md
│   │           ├── email_Alex_OpenFlexure_2026-07-03.md
│   │           ├── email_Alex_Squid_2026-07-03.md
│   │           ├── email_Alex_Squid_followup_2026-07-03.md
│   │           └── strategic_memo_Koln_narrative_2026-07-04.md
│   ├── Aubrey/
│   │   ├── CONCEPT.md  ← core
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   ├── DESIGN.md  ← core
│   │   ├── EVIDENCE.md  ← core
│   │   ├── LICENSE  ← core
│   │   ├── MAP.md  ← core
│   │   ├── MEMORY.md  ← core
│   │   ├── PARAMETERS.md  ← core
│   │   ├── README.md  ← core
│   │   ├── STATE.md  ← core
│   │   ├── THEORY.md  ← core
│   │   ├── TODO.md  ← core
│   │   ├── _pi.md  ← core
│   │   ├── Phase-0/
│   │   │   ├── CONCEPT.md  ← core
│   │   │   ├── DESIGN.md  ← core
│   │   │   ├── EVIDENCE.md  ← core
│   │   │   ├── MAP.md  ← core
│   │   │   ├── MEMORY.md  ← core
│   │   │   ├── PARAMETERS.md  ← core
│   │   │   ├── README.md  ← core
│   │   │   ├── STATE.md  ← core
│   │   │   ├── THEORY.md  ← core
│   │   │   ├── TODO.md  ← core
│   │   │   ├── _pi.md  ← core
│   │   │   └── docs/
│   │   │       ├── BOM.md
│   │   │       ├── CONCEPT_Phase0_desktop_backup.md
│   │   │       ├── KNOWLEDGE.md
│   │   │       ├── OPEN_PROBLEMS.md
│   │   │       ├── TODO_Na_Chelovecheskom_iazike.md
│   │   │       ├── UPGRADE.md
│   │   │       ├── engineering_notes.md
│   │   │       ├── inventory.md
│   │   │       └── validation_protocol.md
│   │   ├── Phase-A/
│   │   │   ├── CONCEPT.md  ← core
│   │   │   ├── DESIGN.md  ← core
│   │   │   ├── EVIDENCE.md  ← core
│   │   │   ├── MAP.md  ← core
│   │   │   ├── MEMORY.md  ← core
│   │   │   ├── PARAMETERS.md  ← core
│   │   │   ├── README.md  ← core
│   │   │   ├── STATE.md  ← core
│   │   │   ├── THEORY.md  ← core
│   │   │   ├── TODO.md  ← core
│   │   │   ├── _pi.md  ← core
│   │   │   ├── docs/
│   │   │   │   ├── ARGUS-LP_BOM.md
│   │   │   │   ├── PROTOCOL_lineage_validation.md
│   │   │   │   ├── Polnaya_posledovatelnost_eksperimenta_v2.md
│   │   │   │   ├── email_Jacquemet_reply_2026-06-15.txt
│   │   │   │   └── partners.md
│   │   │   └── refs/
│   │   │       ├── PMID_12970569_Yamashita_2003_science_orientation_of_asymmetric_stem_cell_.md
│   │   │       ├── PMID_15454403_Botvinick_2004_biophysj_controlled_ablation_of_microtubules_.md
│   │   │       ├── PMID_19558419_Zeigler_2009_photochemphotobiol_laser_selection_significantly_affect.md
│   │   │       ├── PMID_20018668_Verzijlbergen_2010_procnatlacadsciusa_recombination_induced_tag_exchange_t.md
│   │   │       ├── PMID_28661494_Laissue_2017_natmethods_assessing_phototoxicity_in_live_fluo.md
│   │   │       ├── PMID_28749075_Icha_2017_bioessays_phototoxicity_in_live_fluorescence_m.md
│   │   │       ├── PMID_36685184_Mangione_2022_frontphysiol_photoablation_at_single_cell_resolut.md
│   │   │       ├── PMID_36685234_Koevoet_2022_frontneurosci_catecholaminergic_and_cholinergic_ne.md
│   │   │       ├── PMID_36977999_Bürgy_2023_bmcbioinformatics_cenfind_a_deep_learning_pipeline_for.md
│   │   │       ├── PMID_37882444_Royall_2023_elife_asymmetric_inheritance_of_centrosome.md
│   │   │       └── README.md  ← core
│   │   ├── Phase-B/
│   │   │   ├── CONCEPT.md  ← core
│   │   │   ├── DESIGN.md  ← core
│   │   │   ├── EVIDENCE.md  ← core
│   │   │   ├── MAP.md  ← core
│   │   │   ├── MEMORY.md  ← core
│   │   │   ├── PARAMETERS.md  ← core
│   │   │   ├── README.md  ← core
│   │   │   ├── STATE.md  ← core
│   │   │   ├── THEORY.md  ← core
│   │   │   ├── TODO.md  ← core
│   │   │   └── _pi.md  ← core
│   │   ├── _archive/
│   │   │   ├── Alex_F.md
│   │   │   ├── Aubrey_CONCEPT.docx
│   │   │   ├── Aubrey_CONCEPT.pdf
│   │   │   ├── Aubrey_articles_status.md
│   │   │   ├── Aubrey_final.md.recommendations.md
│   │   │   ├── Aubrey_review.md.recommendations.md
│   │   │   ├── CONCEPT.md.bak
│   │   │   ├── CONCEPT.md.best.bak
│   │   │   ├── CONCEPT.md.best.json
│   │   │   ├── CONCEPT.md.final_bak
│   │   │   ├── CONCEPT.md.fix_bak
│   │   │   ├── CONCEPT.md.mbpr.md
│   │   │   ├── CONCEPT.md.recommendations.md
│   │   │   ├── Entropy_in_Aging_2026_registration.md
│   │   │   ├── MAP.md.bak
│   │   │   ├── MAP.md.best.bak
│   │   │   ├── MAP.md.best.json
│   │   │   ├── MAP.md.mbpr.md
│   │   │   ├── MEMORY.md.bak
│   │   │   ├── MEMORY.md.best.bak
│   │   │   ├── MEMORY.md.best.json
│   │   │   ├── MEMORY.md.fix_bak
│   │   │   ├── MEMORY.md.mbpr.md
│   │   │   ├── OPEN_PROBLEMS.md
│   │   │   ├── OSF_PREREGISTRATION.md
│   │   │   ├── OSF_REGISTRATION_INSTRUCTIONS.md
│   │   │   ├── PARAMETERS.md.bak
│   │   │   ├── PARAMETERS.md.best.bak
│   │   │   ├── PARAMETERS.md.best.json
│   │   │   ├── PARAMETERS.md.fix_bak
│   │   │   ├── PARAMETERS.md.mbpr.md
│   │   │   ├── README.md.bak
│   │   │   ├── README.md.best.bak
│   │   │   ├── README.md.best.json
│   │   │   ├── README.md.final_bak
│   │   │   ├── README.md.fix_bak
│   │   │   ├── README.md.mbpr.md
│   │   │   ├── STATE.md.bak
│   │   │   ├── STATE.md.best.bak
│   │   │   ├── STATE.md.best.json
│   │   │   ├── STATE.md.fix_bak
│   │   │   ├── STATE.md.mbpr.md
│   │   │   ├── TODO.md.bak
│   │   │   ├── TODO.md.best.bak
│   │   │   ├── TODO.md.best.json
│   │   │   ├── TODO.md.mbpr.md
│   │   │   ├── TODO_Na_Chelovecheskom_iazike.md
│   │   │   ├── _pi.md.bak
│   │   │   ├── _pi.md.best.bak
│   │   │   ├── _pi.md.best.json
│   │   │   ├── _pi.md.mbpr.md
│   │   │   ├── aging_cell_cover_letter.md
│   │   │   ├── aging_cell_manuscript.docx
│   │   │   ├── aging_cell_manuscript.md
│   │   │   ├── bench_coPI_analysis.md
│   │   │   ├── cover_letter_hardwarex.md
│   │   │   ├── dr-inject.js
│   │   │   ├── generate_tokens.py
│   │   │   ├── get_token_final.py
│   │   │   ├── get_tokens.py
│   │   │   ├── letter_geiger.md
│   │   │   ├── letter_janke.md
│   │   │   ├── outline_token_via_api.py
│   │   │   ├── peer_review_afaf_correspondence.md
│   │   │   ├── plant_cognition_block_draft.md
│   │   │   ├── preliminary_variant_A.md
│   │   │   ├── progress_checklist.md
│   │   │   ├── prophecies_immortality.md
│   │   │   ├── response_to_Afaf_FINAL.md
│   │   │   ├── reviewers_MCAOA.md
│   │   │   ├── setup_tokens_manually.sh
│   │   │   ├── virtual_scientists_signatures.md
│   │   │   ├── waiver_request_hardwarex.md
│   │   │   ├── ARGUS_RSI/
│   │   │   │   ├── ARGUS_LP.docx
│   │   │   │   ├── ARGUS_LP.docx.bak
│   │   │   │   ├── ARGUS_LP.md.bak
│   │   │   │   ├── RSI_appeal_letter_2026-05-28.md
│   │   │   │   ├── rsi_corrections_2026-05-25.md
│   │   │   │   └── rsi_receipt_2026-05-25.md
│   │   │   ├── JabaEqimi/
│   │   │   │   └── voice/
│   │   │   ├── Marketing/
│   │   │   ├── PhD/
│   │   │   ├── correspondence/
│   │   │   │   ├── Adona_briefing.md
│   │   │   │   ├── Macip_call_prep.md
│   │   │   │   ├── UNED_PhD_inquiry_2026-05-18.md
│   │   │   │   ├── UNED_email_draft_to_autorizacionextranjeros.md
│   │   │   │   ├── bbs_rejection_2026-06-12.md
│   │   │   │   ├── gobbetti_call_prep.md
│   │   │   │   ├── greiner_call_prep.md
│   │   │   │   ├── greiner_exchange.md
│   │   │   │   └── shpargalka_adona.md
│   │   │   ├── grants/
│   │   │   │   ├── Partner_Email_Templates.md
│   │   │   │   └── cover_letter_template.md
│   │   │   ├── macrobiome_microbiome_2026/
│   │   │   │   ├── BBS-S-26-00985.pdf
│   │   │   │   ├── BBS-S-26-00994_TAP_submission.pdf
│   │   │   │   ├── Sensation_Feeling_Abstraction.docx
│   │   │   │   ├── Sensation_Feeling_Abstraction_Adaptive_Behavior.md
│   │   │   │   ├── Sensation_Feeling_Abstraction_v2.md
│   │   │   │   ├── TAP_BBS_Sensation_Feeling_Abstraction.docx
│   │   │   │   ├── TAP_BBS_Sensation_Feeling_Abstraction.md
│   │   │   │   ├── cover_letter_Biology_Philosophy.docx
│   │   │   │   ├── cover_letter_Biology_Philosophy.md
│   │   │   │   ├── cover_letter_New_Ideas_Psychology.md
│   │   │   │   ├── manuscript_expanded.docx
│   │   │   │   ├── manuscript_expanded.md
│   │   │   │   ├── withdrawal_Adaptive_Behavior_AB-26-0225.md
│   │   │   │   └── submission_NIP_2026-07-02/
│   │   │   ├── results/
│   │   │   │   ├── physical_experiments/
│   │   │   │   └── pre_experimental/
│   │   │   ├── sourcing/
│   │   │   │   ├── generate_sourcing_letters.py
│   │   │   │   ├── sourcing_confirmation_template.md
│   │   │   │   └── sourcing_inquiry.pdf
│   │   │   └── tmp/
│   │   │       ├── mbpr_kartveli.log
│   │   │       ├── mbpr_mesto_sily.log
│   │   │       └── mbpr_runs/
│   │   ├── crates/
│   │   │   └── aubrey-core/
│   │   │       ├── Cargo.toml
│   │   │       └── src/
│   │   ├── docs/
│   │   │   ├── ARGUS_consortium_full_list.md
│   │   │   ├── BOLD_PILOT.md
│   │   │   ├── CONCEPT_FULL.md
│   │   │   ├── KNOWLEDGE.md
│   │   │   ├── Polnaya_posledovatelnost_eksperimenta_v3.md
│   │   │   ├── all_reviews_summary.md
│   │   │   ├── pmid_audit_2026-05-17.md
│   │   │   ├── publications_pmid_2026-06-10.md
│   │   │   ├── Gönczy_Call_2026-07-07/
│   │   │   │   ├── Шпаргалка_Гёнци.docx
│   │   │   │   └── Шпаргалка_Гёнци.md
│   │   │   ├── Macip_Call_2026-06-12/
│   │   │   │   ├── 01_Agenda_Zvonka.docx
│   │   │   │   ├── 01_Agenda_Zvonka.md
│   │   │   │   ├── 02_EIC_Pathfinder_Concept.docx
│   │   │   │   ├── 02_EIC_Pathfinder_Concept.md
│   │   │   │   ├── 03_Consortium_Overview.docx
│   │   │   │   ├── 03_Consortium_Overview.md
│   │   │   │   ├── 04_Adona_Cheat_Sheet.docx
│   │   │   │   ├── 04_Adona_Cheat_Sheet.md
│   │   │   │   ├── 05_Macip_Role_Description.md
│   │   │   │   └── Macip_Role_Description_2026-06-12.docx
│   │   │   ├── articles/
│   │   │   │   ├── springer_nature_post_aubrey.md
│   │   │   │   └── ARGUS_RSI/
│   │   │   ├── correspondence/
│   │   │   │   ├── 03_gobbetti_eic_partnership.md
│   │   │   │   ├── 04_greiner_eic_partnership.md
│   │   │   │   ├── AI-Enabling-Datasets-Impetus-Grants.pdf
│   │   │   │   ├── ASPM_feasibility_check.md
│   │   │   │   ├── ASPM_feasibility_inquiry.pdf
│   │   │   │   ├── Macip_Call.ics
│   │   │   │   ├── Response_Afaf_Protocol.docx
│   │   │   │   ├── Response_Afaf_Protocol.md
│   │   │   │   ├── Tqemaladze_Invitation_Letter_signed.pdf
│   │   │   │   ├── email_Aubrey_gatekeeper.md
│   │   │   │   ├── email_Geiger_LoS_2026-06-12.md
│   │   │   │   ├── email_Macip_LoS_2026-06-12.md
│   │   │   │   ├── email_gobbetti_link.md
│   │   │   │   ├── email_gobbetti_reminder.md
│   │   │   │   ├── email_gobbetti_slot.md
│   │   │   │   ├── email_gonczy_reply_2026-07-05.docx
│   │   │   │   ├── email_gonczy_reply_2026-07-05.md
│   │   │   │   ├── email_greiner_outline_reply_2026-06-16.md
│   │   │   │   ├── email_greiner_teams_link.md
│   │   │   │   ├── email_impetus_inquiry.md
│   │   │   │   ├── email_jacquemet_pilot_2026-06-25.md
│   │   │   │   ├── email_lvf_inquiry.md
│   │   │   │   ├── email_trifunovic_reply_2026-07-07.md
│   │   │   │   ├── letter_bettencourt-dias_2026-06-12.md
│   │   │   │   ├── letter_jacquemet_2026-06-12.md
│   │   │   │   ├── letter_janke_followup_2026-06-07.md
│   │   │   │   ├── letter_nigg_2026-06-07.md
│   │   │   │   ├── letter_raff_2026-06-07.md
│   │   │   │   ├── macip_consortium_2026-06-10.md
│   │   │   │   ├── macip_response_2026-06-12.md
│   │   │   │   ├── response_macip_2.md
│   │   │   │   ├── response_macip_2026-06-12.md
│   │   │   │   ├── response_macip_2026-06-12_v2.md
│   │   │   │   └── zheleznov_2026-05-26.md
│   │   │   ├── letters_of_support/
│   │   │   │   ├── GLA_commitment.pdf
│   │   │   │   ├── GLA_commitment_SIGNED.pdf
│   │   │   │   ├── GLA_commitment_template.md
│   │   │   │   ├── Geiger_LoS_Ulm_2026-04-23.pdf
│   │   │   │   ├── Geiger_LoS_Ulm_EIC_2026-06-12.docx
│   │   │   │   ├── Geiger_LoS_Ulm_EIC_2026-06-12.md
│   │   │   │   ├── Macip_LoS_BBRC_2026-06-12.docx
│   │   │   │   ├── Macip_LoS_BBRC_2026-06-12.md
│   │   │   │   ├── Macip_LoS_BBRC_2026-06-12.pdf
│   │   │   │   ├── Macip_LoS_UOC_2026-06-12.docx
│   │   │   │   ├── Macip_LoS_UOC_2026-06-12.md
│   │   │   │   ├── Zheleznov_LoS_status.md
│   │   │   │   └── reserve/
│   │   │   ├── photos/
│   │   │   │   └── photo_2026-06-06_13-41-14.jpg
│   │   │   ├── presentations/
│   │   │   │   └── brief_Ilia_Koln_presentation.md
│   │   │   └── refs/
│   │   │       ├── PMID_11285289_Khodjakov_2001_JCB_centrosomes_cytokinesis.md
│   │   │       ├── PMID_11285289_Khodjakov_2001_jcellbiol_centrosomes_enhance_the_fidelity_of_.md
│   │   │       ├── PMID_15877495_Kuipers_2003_ruralremotehealth_evaluation_of_a_rural_community_base.md
│   │   │       ├── PMID_16157702_Ferraiuolo_2005_jcellbiol_a_role_for_the_eif4e_binding_protein.md
│   │   │       ├── PMID_16262721_Colombelli_2005_Traffic_subcellular_UV_ablation.md
│   │   │       ├── PMID_16262721_Colombelli_2005_traffic_in_vivo_selective_cytoskeleton_dynam.md
│   │   │       ├── PMID_17255513_Yamashita_2007_Science_asymmetric_centrosome_germline.md
│   │   │       ├── PMID_17255513_Yamashita_2007_science_asymmetric_inheritance_of_mother_ver.md
│   │   │       ├── PMID_19237889_Cai_2009_critcaremed_anti_inflammatory_adjuvant_in_resusc.md
│   │   │       ├── PMID_19535732_McDonald_2009_jcellsci_no_strings_attached_the_escrt_machin.md
│   │   │       ├── PMID_20018668_Verzijlbergen_2010_PNAS_RITE_original.md
│   │   │       ├── PMID_20018668_Verzijlbergen_2010_procnatlacadsciusa_recombination_induced_tag_exchange_t.md
│   │   │       ├── PMID_20562852_Marcheva_2010_nature_disruption_of_the_clock_components_c.md
│   │   │       ├── PMID_21795662_Martinez-Hernandez_2011_neurology_analysis_of_complement_and_plasma_ce.md
│   │   │       ├── PMID_22265426_Asselin_2012_eurjcancer_quantifying_heterogeneity_in_human_t.md
│   │   │       ├── PMID_23749304_Pitrone_2013_NatMethods_OpenSPIM.md
│   │   │       ├── PMID_23749304_Pitrone_2013_natmethods_openspim_an_open_access_light_sheet_.md
│   │   │       ├── PMID_24613568_Shekhel_2014_humpathol_surgical_pathology_of_pleural_coccid.md
│   │   │       ├── PMID_25416946_Espín-Palazón_2014_cell_proinflammatory_signaling_regulates_.md
│   │   │       ├── PMID_25955889_Al-Abdullah_2015_molecules_synthesis_antimicrobial_and_hypoglyc.md
│   │   │       ├── PMID_29363672_Nigg_2018_NatRevMCB_centriole_duplication.md
│   │   │       ├── PMID_29363672_Nigg_2018_natrevmolcellbiol_once_and_only_once_mechanisms_of_cen.md
│   │   │       ├── PMID_30874553_Almada_2019_NatCommun_NanoJ_Fluidics.md
│   │   │       ├── PMID_32289204_Liu_2020_chembiodivers_chartarlactams_q_t_dimeric_phenylspi.md
│   │   │       ├── PMID_33725406_Gambichler_2021_jeuracaddermatolve_prompt_onset_of_rowell_s_syndrome_fo.md
│   │   │       ├── PMID_34520770_Zhu_2021_lifesci_tad1822_7_induces_ros_mediated_apopt.md
│   │   │       ├── PMID_36583780_Tkemaladze_2023_MolBiolRep_centriole_stemcell.md
│   │   │       ├── PMID_36583780_Tkemaladze_2023_molbiolrep_reduction_proliferation_and_differen.md
│   │   │       ├── PMID_7573318_Weidenthal_1995_amjophthalmol_cardiopulmonary_arrest_after_retrobu.md
│   │   │       ├── PMID_8602510_Naldini_1996_Science_lentivirus_nondividing.md
│   │   │       ├── PMID_8602510_Naldini_1996_science_in_vivo_gene_delivery_and_stable_tra.md
│   │   │       ├── PMID_9765382_Dull_1998_JVirol_3gen_lentivirus_packaging.md
│   │   │       ├── PMID_9765382_Dull_1998_jvirol_a_third_generation_lentivirus_vector.md
│   │   │       └── README.md  ← core
│   │   ├── grants/
│   │   │   ├── 01_NLNet_NGI0_Commons_Fund.md
│   │   │   ├── 02_Fondy_Open_Robotics_Hardware.md
│   │   │   ├── 03_Pismo_Shablon.md
│   │   │   ├── 04_Concept_note_CIRCBIO07.md
│   │   │   ├── 04_NLNet_ARGUS_Aubrey.md
│   │   │   ├── EIC_ConceptPaper_Aubrey_ExecSummary.md
│   │   │   ├── EIC_ConceptPaper_Aubrey_outline.md
│   │   │   ├── NLNet_ARGUS_Aubrey_submitted.pdf
│   │   │   ├── impetus_strategy.md
│   │   │   └── EIC_Pathfinder_Open/
│   │   │       ├── BIORXIV-2026-737107v1-Tqemaladze.pdf
│   │   │       ├── CONCEPT.md  ← core
│   │   │       ├── CheatSheet_Gonczy_2026-07-07.docx
│   │   │       ├── CheatSheet_Gonczy_2026-07-07.md
│   │   │       ├── EVIDENCE.md  ← core
│   │   │       ├── Letter_Gonczy_followup.docx
│   │   │       ├── MAP.md  ← core
│   │   │       ├── MCARA.docx
│   │   │       ├── MCARA_AUDIT_2026-07-07.md
│   │   │       ├── MCARA_Article.pdf
│   │   │       ├── MCARA_Article_final.md
│   │   │       ├── MCARA_Article_v3.md
│   │   │       ├── MCARA_Peer_Review_2026-07-07.docx
│   │   │       ├── MCARA_Peer_Review_2026-07-07.md
│   │   │       ├── MCARA_Peer_Review_DeepSeek_2026-07-08.md
│   │   │       ├── MCARA_Response_to_Peer_Review_2026-07-07.md
│   │   │       ├── MCARA_VERIFICATION_2026-07-07.md
│   │   │       ├── MCARA_bioRxiv_cover_letter.md
│   │   │       ├── MEMORY.md  ← core
│   │   │       ├── PARAMETERS.md  ← core
│   │   │       ├── README.md  ← core
│   │   │       ├── STATE.md  ← core
│   │   │       ├── TODO.md  ← core
│   │   │       ├── Wagner_email_2026-07-07.md
│   │   │       ├── Wagner_meeting_2026-07-10.md
│   │   │       ├── _pi.md  ← core
│   │   │       ├── MCARA_EPFL/
│   │   │       ├── docs/
│   │   │       └── refs/
│   │   ├── letters/
│   │   │   ├── 01_reply_Delcour.md
│   │   │   ├── 02_CIRCBIO07_Fe_data_request_Leuven_Ghent.md
│   │   │   ├── 03_CIRCBIO07_Courtin_personal.md
│   │   │   └── email_filippov_detailed_2026-06-25.md
│   │   └── scripts/
│   │       ├── tbpr_aubrey_overnight.sh
│   │       ├── tbpr_perfile_overnight.sh
│   │       └── code/
│   │           ├── README.md  ← core
│   │           └── pipeline/
│   ├── CEDAR/
│   │   ├── CONCEPT.md  ← core
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   ├── Centriole_Elimination_iPSC_Reprogramming.md
│   │   ├── DESIGN.md  ← core
│   │   ├── EVIDENCE.md  ← core
│   │   ├── LICENSE  ← core
│   │   ├── MAP.md  ← core
│   │   ├── MEMORY.md  ← core
│   │   ├── PARAMETERS.md  ← core
│   │   ├── README.md  ← core
│   │   ├── STATE.md  ← core
│   │   ├── THEORY.md  ← core
│   │   ├── TODO.md  ← core
│   │   ├── _pi.md  ← core
│   │   ├── Aubrey-Platform/
│   │   │   ├── CONCEPT.md  ← core
│   │   │   ├── DESIGN.md  ← core
│   │   │   ├── EVIDENCE.md  ← core
│   │   │   ├── MAP.md  ← core
│   │   │   ├── MEMORY.md  ← core
│   │   │   ├── PARAMETERS.md  ← core
│   │   │   ├── README.md  ← core
│   │   │   ├── STATE.md  ← core
│   │   │   ├── THEORY.md  ← core
│   │   │   ├── TODO.md  ← core
│   │   │   ├── _pi.md  ← core
│   │   │   ├── ARGUS-Hardware/
│   │   │   │   ├── CONCEPT.md  ← core
│   │   │   │   ├── MAP.md  ← core
│   │   │   │   ├── MEMORY.md  ← core
│   │   │   │   ├── PARAMETERS.md  ← core
│   │   │   │   ├── README.md  ← core
│   │   │   │   ├── STATE.md  ← core
│   │   │   │   ├── TODO.md  ← core
│   │   │   │   ├── _pi.md  ← core
│   │   │   │   ├── _archive/
│   │   │   │   └── refs/
│   │   │   ├── docs/
│   │   │   │   ├── Aubrey_Platform_full_grant.md
│   │   │   │   ├── KNOWLEDGE.md
│   │   │   │   └── OPEN_PROBLEMS.md
│   │   │   └── refs/
│   │   │       ├── PMID_11285289_Khodjakov_2001_JCB_centrosomes_cytokinesis.md
│   │   │       ├── PMID_11285289_Khodjakov_2001_jcellbiol_centrosomes_enhance_the_fidelity_of_.md
│   │   │       ├── PMID_15877495_Kuipers_2003_ruralremotehealth_evaluation_of_a_rural_community_base.md
│   │   │       ├── PMID_16157702_Ferraiuolo_2005_jcellbiol_a_role_for_the_eif4e_binding_protein.md
│   │   │       ├── PMID_16262721_Colombelli_2005_Traffic_subcellular_UV_ablation.md
│   │   │       ├── PMID_16262721_Colombelli_2005_traffic_in_vivo_selective_cytoskeleton_dynam.md
│   │   │       ├── PMID_17255513_Yamashita_2007_Science_asymmetric_centrosome_germline.md
│   │   │       ├── PMID_17255513_Yamashita_2007_science_asymmetric_inheritance_of_mother_ver.md
│   │   │       ├── PMID_19237889_Cai_2009_critcaremed_anti_inflammatory_adjuvant_in_resusc.md
│   │   │       ├── PMID_19535732_McDonald_2009_jcellsci_no_strings_attached_the_escrt_machin.md
│   │   │       ├── PMID_20018668_Verzijlbergen_2010_PNAS_RITE_original.md
│   │   │       ├── PMID_20018668_Verzijlbergen_2010_procnatlacadsciusa_recombination_induced_tag_exchange_t.md
│   │   │       ├── PMID_20562852_Marcheva_2010_nature_disruption_of_the_clock_components_c.md
│   │   │       ├── PMID_21795662_Martinez-Hernandez_2011_neurology_analysis_of_complement_and_plasma_ce.md
│   │   │       ├── PMID_22265426_Asselin_2012_eurjcancer_quantifying_heterogeneity_in_human_t.md
│   │   │       ├── PMID_23749304_Pitrone_2013_NatMethods_OpenSPIM.md
│   │   │       ├── PMID_23749304_Pitrone_2013_natmethods_openspim_an_open_access_light_sheet_.md
│   │   │       ├── PMID_24613568_Shekhel_2014_humpathol_surgical_pathology_of_pleural_coccid.md
│   │   │       ├── PMID_25416946_Espín-Palazón_2014_cell_proinflammatory_signaling_regulates_.md
│   │   │       ├── PMID_25955889_Al-Abdullah_2015_molecules_synthesis_antimicrobial_and_hypoglyc.md
│   │   │       ├── PMID_29363672_Nigg_2018_NatRevMCB_centriole_duplication.md
│   │   │       ├── PMID_29363672_Nigg_2018_natrevmolcellbiol_once_and_only_once_mechanisms_of_cen.md
│   │   │       ├── PMID_30874553_Almada_2019_NatCommun_NanoJ_Fluidics.md
│   │   │       ├── PMID_32289204_Liu_2020_chembiodivers_chartarlactams_q_t_dimeric_phenylspi.md
│   │   │       ├── PMID_33725406_Gambichler_2021_jeuracaddermatolve_prompt_onset_of_rowell_s_syndrome_fo.md
│   │   │       ├── PMID_34520770_Zhu_2021_lifesci_tad1822_7_induces_ros_mediated_apopt.md
│   │   │       ├── PMID_36583780_Tkemaladze_2023_MolBiolRep_centriole_stemcell.md
│   │   │       ├── PMID_36583780_Tkemaladze_2023_molbiolrep_reduction_proliferation_and_differen.md
│   │   │       ├── PMID_7573318_Weidenthal_1995_amjophthalmol_cardiopulmonary_arrest_after_retrobu.md
│   │   │       ├── PMID_8602510_Naldini_1996_Science_lentivirus_nondividing.md
│   │   │       ├── PMID_8602510_Naldini_1996_science_in_vivo_gene_delivery_and_stable_tra.md
│   │   │       ├── PMID_9765382_Dull_1998_JVirol_3gen_lentivirus_packaging.md
│   │   │       ├── PMID_9765382_Dull_1998_jvirol_a_third_generation_lentivirus_vector.md
│   │   │       └── README.md  ← core
│   │   ├── CellLineageTree/
│   │   │   ├── CONCEPT.md  ← core
│   │   │   ├── MAP.md  ← core
│   │   │   ├── MEMORY.md  ← core
│   │   │   ├── PARAMETERS.md  ← core
│   │   │   ├── README.md  ← core
│   │   │   ├── STATE.md  ← core
│   │   │   ├── TODO.md  ← core
│   │   │   ├── _pi.md  ← core
│   │   │   ├── _archive/
│   │   │   │   ├── UPGRADE.md.draft
│   │   │   │   ├── AICoordinator/
│   │   │   │   ├── CellPose_Segmentation/
│   │   │   │   ├── DifferentiationAnnotation/
│   │   │   │   ├── FluorescentCameras/
│   │   │   │   ├── GenealogyReconstruction/
│   │   │   │   ├── ImageAnalysis/
│   │   │   │   ├── LaserAblation_405/
│   │   │   │   ├── LentiviralTools/
│   │   │   │   ├── LiveCellMicroscopy/
│   │   │   │   ├── MicroscopeController/
│   │   │   │   ├── RITE_Centriole/
│   │   │   │   ├── StatisticalAnalysis/
│   │   │   │   └── merged-into-Aubrey-2026-05-12/
│   │   │   ├── audits/
│   │   │   │   └── 2026-05-08/
│   │   │   ├── docs/
│   │   │   │   ├── CENTRAL_EXPERIMENT_DESIGN.md
│   │   │   │   ├── CENTRIOLE_ELIMINATION_DEEP_RESEARCH.md
│   │   │   │   ├── KNOWLEDGE.md
│   │   │   │   ├── LINKS.md
│   │   │   │   ├── PROJECT_AUDIT_2026-05-12.md
│   │   │   │   ├── UPGRADE.md
│   │   │   │   ├── letters_of_support/
│   │   │   │   ├── related/
│   │   │   │   └── tbpr/
│   │   │   └── refs/
│   │   │       ├── PMID_10618407_Bolk_2000_procnatlacadsciusa_a_human_model_for_multigenic_inherit.md
│   │   │       ├── PMID_11285289_Khodjakov_2001_jcellbiol_centrosomes_enhance_the_fidelity_of_.md
│   │   │       ├── PMID_17255513_Yamashita_2007_science_asymmetric_inheritance_of_mother_ver.md
│   │   │       ├── PMID_19829375_Wang_2009_nature_asymmetric_centrosome_inheritance_ma.md
│   │   │       ├── PMID_20018668_Verzijlbergen_2010_procnatlacadsciusa_recombination_induced_tag_exchange_t.md
│   │   │       ├── PMID_23673356_Murugesan_2013_scirep_virus_based_photo_responsive_nanowir.md
│   │   │       ├── PMID_24905236_Le_2015_liverint_major_depression_and_suicide_attempt.md
│   │   │       ├── PMID_25806700_?_2015_nature_a_guide_to_the_nature_index.md
│   │   │       ├── PMID_26213385_Madarampalli_2015_cell_atf5_connects_the_pericentriolar_mat.md
│   │   │       ├── PMID_26418181_Hughes_2015_angewchemintedengl_optogenetic_apoptosis_light_triggere.md
│   │   │       ├── PMID_27287538_Kanokratana_2016_microbecol_comparative_study_of_bacterial_commu.md
│   │   │       ├── PMID_28245920_Bulusu_2017_devcell_spatiotemporal_analysis_of_a_glycoly.md
│   │   │       ├── PMID_29674432_Plass_2018_science_cell_type_atlas_and_lineage_tree_of_.md
│   │   │       ├── PMID_30093604_Kalhor_2018_science_developmental_barcoding_of_whole_mou.md
│   │   │       ├── PMID_31086336_Chan_2019_nature_molecular_recording_of_mammalian_emb.md
│   │   │       ├── PMID_32636438_Viñas_2020_scirep_publisher_correction_feasibility_and.md
│   │   │       ├── PMID_33318659_Stringer_2021_natmethods_cellpose_a_generalist_algorithm_for_.md
│   │   │       ├── PMID_34597559_Maruffi_2022_scitotalenviron_soil_erosion_and_sediment_transport_.md
│   │   │       ├── PMID_36583780_Tkemaladze_2023_molbiolrep_reduction_proliferation_and_differen.md
│   │   │       ├── PMID_36882497_Kapoor_2023_evidbaseddent_comparative_evaluation_of_retention_.md
│   │   │       ├── PMID_37882444_Royall_2023_elife_asymmetric_inheritance_of_centrosome.md
│   │   │       ├── PMID_9730976_Hong_1998_jcellsci_protein_transport_from_the_endoplasm.md
│   │   │       └── README.md  ← core
│   │   ├── _archive/
│   │   │   ├── CONCEPT_CODE_AUDIT_2026-04-21.md
│   │   │   ├── COUNTER_PARAMS_ADDITIVE_ANALYSIS_2026-04-21.md
│   │   │   ├── PARAMS_RECONCILIATION_ANALYSIS_2026-04-21.md
│   │   │   └── articles_without_pmid/
│   │   │       ├── Asymmetric Inheritance of the Aged Mother Centriole by Stem Cells Is an Established Causal Fact.pdf
│   │   │       ├── First Direct Structural Evidence for Age-Dependent Polyglutamylation Asymmetry in Murine Hematopoietic Stem Cells.pdf
│   │   │       ├── The Centriolar Imperative.pdf
│   │   │       └── The Relapse Prediction.pdf
│   │   ├── articles/
│   │   │   ├── CONCEPT.md  ← core
│   │   │   ├── MAP.md  ← core
│   │   │   ├── MEMORY.md  ← core
│   │   │   ├── PARAMETERS.md  ← core
│   │   │   ├── README.md  ← core
│   │   │   ├── STATE.md  ← core
│   │   │   ├── TODO.md  ← core
│   │   │   ├── _pi.md  ← core
│   │   │   └── docs/
│   │   │       ├── TBPR_v2_Multi-LLM_Consensus_Framework.md
│   │   │       ├── TBPR_v2_Patterns_v2.md
│   │   │       ├── cedar_abstract_v1_original.png
│   │   │       ├── cedar_abstract_v2_improved.png
│   │   │       ├── cedar_abstract_v3.png
│   │   │       ├── cedar_abstract_v4.png
│   │   │       ├── cedar_abstract_v5.png
│   │   │       ├── cedar_abstract_v6.png
│   │   │       ├── cedar_abstract_v7.png
│   │   │       ├── cedar_abstract_v8.png
│   │   │       └── cedar_abstract_v9.png
│   │   ├── audits/
│   │   │   └── 2026-05-08/
│   │   │       ├── LC_CEDAR.check.v1.md
│   │   │       ├── LC_CEDAR.plan.v1.md
│   │   │       └── LC_CEDAR.review.md
│   │   ├── backend/
│   │   │   ├── Cargo.toml
│   │   │   ├── Dockerfile
│   │   │   ├── README.md  ← core
│   │   │   ├── migrations/
│   │   │   │   └── 001_initial.sql
│   │   │   └── src/
│   │   │       ├── config.rs
│   │   │       ├── db.rs
│   │   │       ├── error.rs
│   │   │       ├── main.rs
│   │   │       ├── models.rs
│   │   │       └── routes.rs
│   │   ├── crates/
│   │   │   ├── cell_dt_cli/
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src/
│   │   │   ├── cell_dt_core/
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src/
│   │   │   ├── cell_dt_gui/
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src/
│   │   │   ├── cell_dt_modules/
│   │   │   │   ├── aging_engine/
│   │   │   │   ├── asymmetric_division/
│   │   │   │   ├── inflammaging/
│   │   │   │   ├── mitochondrial/
│   │   │   │   └── tissue_specific/
│   │   │   ├── cell_dt_python/
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src/
│   │   │   └── cell_dt_validation/
│   │   │       ├── Cargo.toml
│   │   │       ├── examples/
│   │   │       └── src/
│   │   ├── docs/
│   │   │   ├── CEDAR.docx
│   │   │   ├── CONCEPT.iter.md
│   │   │   ├── EXPERIMENTAL_DESIGNS_2026-07-05.md
│   │   │   ├── OPEN_PROBLEMS.md
│   │   │   ├── PEER_REVIEW_2026-07-05.md
│   │   │   ├── PEER_REVIEW_v2_2026-07-05.md
│   │   │   ├── PROJECT_AUDIT_2026-05-12.md
│   │   │   ├── SOLUTIONS_2026-07-05.md
│   │   │   ├── workshop_entropy_in_aging_2026-07-05.md
│   │   │   ├── workshop_entropy_in_aging_2pages.md
│   │   │   ├── related/
│   │   │   │   ├── parent_LongevityCommon_CONCEPT.md
│   │   │   │   ├── parent_LongevityCommon_EVIDENCE.md
│   │   │   │   ├── parent_LongevityCommon_OPEN_PROBLEMS.md
│   │   │   │   ├── parent_LongevityCommon_PARAMETERS.md
│   │   │   │   ├── parent_LongevityCommon_STATE.md
│   │   │   │   └── parent_LongevityCommon_THEORY.md
│   │   │   └── tbpr/
│   │   │       ├── article_2026-05-09.md
│   │   │       ├── engineering_2026-05-09.md
│   │   │       └── project_2026-05-09.md
│   │   ├── frontend/
│   │   │   ├── mix.exs
│   │   │   ├── config/
│   │   │   │   ├── config.exs
│   │   │   │   ├── dev.exs
│   │   │   │   ├── prod.exs
│   │   │   │   └── runtime.exs
│   │   │   └── lib/
│   │   │       ├── cedar_frontend_web.ex
│   │   │       ├── cedar_frontend/
│   │   │       └── cedar_frontend_web/
│   │   ├── refs/
│   │   │   ├── PMID_11067876_Sudo_2000_jexpmed_age_associated_characteristics_of_mu.md
│   │   │   ├── PMID_12663456_Allsopp_2003_blood_telomerase_is_required_to_slow_telom.md
│   │   │   ├── PMID_16407887_McFarlane_2006_kidneyint_the_impact_of_home_nocturnal_hemodia.md
│   │   │   ├── PMID_16990891_Gibson_2006_brdentj_dental_implantology_education_a_surv.md
│   │   │   ├── PMID_17218264_Shen_2007_cell_selective_activation_of_cognate_snar.md
│   │   │   ├── PMID_17255513_Yamashita_2007_science_asymmetric_inheritance_of_mother_ver.md
│   │   │   ├── PMID_18316408_Dultz_2008_jcellbiol_systematic_kinetic_analysis_of_mitot.md
│   │   │   ├── PMID_18356530_Dunn_2008_science_spending_money_on_others_promotes_ha.md
│   │   │   ├── PMID_18385740_Malanchi_2008_nature_cutaneous_cancer_stem_cell_maintenan.md
│   │   │   ├── PMID_18923395_Cheng_2008_nature_centrosome_misorientation_reduces_st.md
│   │   │   ├── PMID_19246161_Liao_2009_medhypotheses_porphyromonas_gingivalis_may_play_an.md
│   │   │   ├── PMID_19587680_Harrison_2009_nature_rapamycin_fed_late_in_life_extends_l.md
│   │   │   ├── PMID_20304793_Beerman_2010_procnatlacadsciusa_functionally_distinct_hematopoietic_.md
│   │   │   ├── PMID_20674500_Duke_2010_eurjoncolnurs_communication_skills_training_in_end.md
│   │   │   ├── PMID_21145745_Conduit_2010_currbiol_cnn_dynamics_drive_centrosome_size_a.md
│   │   │   ├── PMID_21474673_Velaithan_2011_blood_the_small_gtpase_rac1_is_a_novel_bin.md
│   │   │   ├── PMID_21654799_Dupont_2011_nature_role_of_yap_taz_in_mechanotransducti.md
│   │   │   ├── PMID_21734240_Yahata_2011_blood_accumulation_of_oxidative_dna_damage.md
│   │   │   ├── PMID_21993292_Kollman_2011_natrevmolcellbiol_microtubule_nucleation_by_tubulin_co.md
│   │   │   ├── PMID_22056670_Lapasset_2011_genesdev_rejuvenating_senescent_and_centenari.md
│   │   │   ├── PMID_22215083_Lavasani_2012_natcommun_muscle_derived_stem_progenitor_cell_.md
│   │   │   ├── PMID_22357619_Roth_2012_molbiolcell_centrosome_misorientation_mediates_s.md
│   │   │   ├── PMID_23746838_López-Otín_2013_cell_the_hallmarks_of_aging.md
│   │   │   ├── PMID_23967009_Kovina_2013_frontgenet_effect_on_lifespan_of_high_yield_non.md
│   │   │   ├── PMID_24065130_Zempel_2013_emboj_amyloid_oligomers_induce_synaptic_da.md
│   │   │   ├── PMID_24138928_Horvath_2013_genomebiol_dna_methylation_age_of_human_tissues.md
│   │   │   ├── PMID_25921310_Mehta_2015_curratherosclerrep_icd_and_crt_use_in_ischemic_heart_di.md
│   │   │   ├── PMID_26213385_Madarampalli_2015_cell_atf5_connects_the_pericentriolar_mat.md
│   │   │   ├── PMID_27041501_Takeishi_2016_neuron_receptor_type_guanylyl_cyclases_conf.md
│   │   │   ├── PMID_27984723_Ocampo_2016_cell_in_vivo_amelioration_of_age_associat.md
│   │   │   ├── PMID_28636844_Jaiswal_2017_nengljmed_clonal_hematopoiesis_and_risk_of_ath.md
│   │   │   ├── PMID_28901234_Nichay_2017_worldjpediatrconge_risk_factors_for_unfavorable_outcome.md
│   │   │   ├── PMID_28931529_Inoue_2017_jamheartassoc_electrocardiographic_strain_pattern_.md
│   │   │   ├── PMID_29786094_Wei_2018_natbiotechnol_a_protein_activity_assay_to_measure_.md
│   │   │   ├── PMID_31242442_Nelson_2019_neuroscience_escalated_alcohol_self_administratio.md
│   │   │   ├── PMID_31451800_Ibrahim_2019_natbiomedeng_augmenting_canonical_wnt_signalling_.md
│   │   │   ├── PMID_31914653_Peters-Hall_2020_fasebj_proliferation_of_adult_human_bronchi.md
│   │   │   ├── PMID_32755011_Florian_2020_agingcell_inhibition_of_cdc42_activity_extends.md
│   │   │   ├── PMID_34546229_Fan_2021_analyst_synthesis_and_application_of_smart_g.md
│   │   │   ├── PMID_36581635_Montserrat-Vazquez_2022_npjregenmed_transplanting_rejuvenated_blood_stem.md
│   │   │   ├── PMID_36708707_Berendzen_2023_neuron_oxytocin_receptor_is_not_required_fo.md
│   │   │   ├── PMID_37079650_Oh_2023_plosone_covid_19_maternal_and_neonatal_outco.md
│   │   │   ├── PMID_37184769_Farzaneh_2023_techcoloproctol_validation_of_an_endoscopic_anastomo.md
│   │   │   ├── PMID_37478901_O'Neill_2024_jinvestdermatol_genetic_and_functional_analyses_of_c.md
│   │   │   ├── PMID_37552892_Yamashita_2023_annurevgenet_asymmetric_stem_cell_division_and_ge.md
│   │   │   ├── PMID_38658656_Mahalingan_2024_natchembiol_structural_basis_for_tubulin_specifi.md
│   │   │   ├── PMID_38679727_Zhu_2024_stemcellresther_safety_and_efficacy_of_umbilical_cor.md
│   │   │   ├── PMID_38874393_Ugale_2024_jcellbiol_signaling_proteins_in_hsc_fate_deter.md
│   │   │   ├── PMID_39012627_Thomas_2024_jcellbiol_centrosome_age_breaks_spindle_size_s.md
│   │   │   ├── PMID_39266565_Robichaud_2024_natcommun_transiently_formed_nucleus_to_cilium.md
│   │   │   ├── PMID_39412222_Launay_2025_agingcell_altered_tubulin_detyrosination_due_t.md
│   │   │   ├── PMID_39743633_Wang_2025_cellres_reducing_functionally_defective_old_.md
│   │   │   ├── PMID_39764850_Barandun_2025_cellrep_targeted_localization_of_the_mother_.md
│   │   │   ├── PMID_40157365_Pao_2025_devcell_kif2c_promotes_paclitaxel_resistance.md
│   │   │   ├── PMID_40257113_Hamzah_2025_cytoskeleton_hobok_plk4_master_regulator_of_centriole_d.md
│   │   │   ├── PMID_40562035_Rando_2025_cellstemcell_hallmarks_of_stem_cell_aging.md
│   │   │   ├── PMID_40562742_Gavoci_2025_natcommun_polyglutamylation_of_microtubules_dr.md
│   │   │   ├── PMID_6129277_Harrison_1982_jexpmed_loss_of_stem_cell_repopulating_abili.md
│   │   │   ├── PMID_9576819_Faust_1998_brainlang_levels_of_sentence_constraint_and_le.md
│   │   │   └── README.md  ← core
│   │   ├── scripts/
│   │   │   └── run.sh
│   │   ├── simulator/
│   │   │   ├── CONCEPT.md  ← core
│   │   │   ├── DESIGN.md  ← core
│   │   │   ├── EVIDENCE.md  ← core
│   │   │   ├── LICENSE  ← core
│   │   │   ├── MAP.md  ← core
│   │   │   ├── MEMORY.md  ← core
│   │   │   ├── PARAMETERS.md  ← core
│   │   │   ├── README.md  ← core
│   │   │   ├── STATE.md  ← core
│   │   │   ├── THEORY.md  ← core
│   │   │   ├── TODO.md  ← core
│   │   │   ├── _pi.md  ← core
│   │   │   ├── pyproject.toml
│   │   │   ├── cedar_sim.egg-info/
│   │   │   │   ├── PKG-INFO
│   │   │   │   ├── SOURCES.txt
│   │   │   │   ├── dependency_links.txt
│   │   │   │   ├── requires.txt
│   │   │   │   └── top_level.txt
│   │   │   └── docs/
│   │   │       ├── DIAGNOSIS.md
│   │   │       └── STATE_v3.md
│   │   └── submissions/
│   │       └── 2026-07-10_BioEssays/
│   │           ├── 5285ce27-4d94-4ab7-898e-3a78a88df629.pdf
│   │           ├── BioEssays_submission_plan.md
│   │           ├── Centriole Elimination as a Gateway to a New Differentiation State.docx
│   │           ├── Centriole_Elimination_Hypothesis_BioEssays.docx
│   │           ├── Cover_Letter_BioEssays.docx
│   │           └── RESEARCH_SQUARE_SUBMISSION.md
│   ├── EpigeneticDrift/
│   │   ├── CONCEPT.md  ← core
│   │   ├── DESIGN.md  ← core
│   │   ├── EVIDENCE.md  ← core
│   │   ├── MAP.md  ← core
│   │   ├── MEMORY.md  ← core
│   │   ├── PARAMETERS.md  ← core
│   │   ├── README.md  ← core
│   │   ├── STATE.md  ← core
│   │   ├── THEORY.md  ← core
│   │   ├── TODO.md  ← core
│   │   ├── _pi.md  ← core
│   │   ├── _archive/
│   │   │   └── STATE.md.final2_bak
│   │   ├── audits/
│   │   │   └── 2026-05-08/
│   │   │       ├── LC_EpigeneticDrift.check.v1.md
│   │   │       ├── LC_EpigeneticDrift.plan.v1.md
│   │   │       └── LC_EpigeneticDrift.review.md
│   │   ├── backend/
│   │   │   ├── Cargo.toml
│   │   │   ├── Dockerfile
│   │   │   ├── README.md  ← core
│   │   │   ├── migrations/
│   │   │   │   └── 001_initial.sql
│   │   │   └── src/
│   │   │       ├── config.rs
│   │   │       ├── db.rs
│   │   │       ├── error.rs
│   │   │       ├── main.rs
│   │   │       ├── models.rs
│   │   │       └── routes.rs
│   │   ├── crates/
│   │   │   └── epigenetic_counter/
│   │   │       ├── Cargo.lock
│   │   │       ├── Cargo.toml
│   │   │       ├── src/
│   │   │       └── tests/
│   │   ├── data/
│   │   │   ├── PARAMETERS_calibrated.json
│   │   │   └── PARAMETERS_real_calibrated.json
│   │   ├── docs/
│   │   │   ├── DAILY_SEARCH_2026-04-20.md
│   │   │   ├── DATASETS.md
│   │   │   ├── EpigeneticDrift_CONCEPT_review.md
│   │   │   ├── GSE40279_calibration.json
│   │   │   ├── JOURNAL.md
│   │   │   ├── META_ANALYSIS_DNA_Methylation_Age_Clocks.md
│   │   │   ├── META_ANALYSIS_Epigenetic_Drift_Stem_Cell_Aging.md
│   │   │   ├── OPEN_PROBLEMS.md
│   │   │   ├── PROJECT_AUDIT_2026-05-12.md
│   │   │   ├── UPGRADE.md
│   │   │   ├── UPGRADE.md.draft
│   │   │   ├── related/
│   │   │   │   ├── parent_CEDAR_CONCEPT.md
│   │   │   │   ├── parent_CEDAR_EVIDENCE.md
│   │   │   │   ├── parent_CEDAR_OPEN_PROBLEMS.md
│   │   │   │   ├── parent_CEDAR_PARAMETERS.md
│   │   │   │   ├── parent_CEDAR_STATE.md
│   │   │   │   ├── parent_CEDAR_THEORY.md
│   │   │   │   ├── parent_LongevityCommon_CONCEPT.md
│   │   │   │   ├── parent_LongevityCommon_EVIDENCE.md
│   │   │   │   ├── parent_LongevityCommon_OPEN_PROBLEMS.md
│   │   │   │   ├── parent_LongevityCommon_PARAMETERS.md
│   │   │   │   ├── parent_LongevityCommon_STATE.md
│   │   │   │   └── parent_LongevityCommon_THEORY.md
│   │   │   └── tbpr/
│   │   │       ├── article_2026-05-09.md
│   │   │       ├── engineering_2026-05-09.md
│   │   │       └── project_2026-05-09.md
│   │   ├── frontend/
│   │   │   ├── README.md  ← core
│   │   │   ├── mix.exs
│   │   │   ├── config/
│   │   │   │   ├── config.exs
│   │   │   │   ├── dev.exs
│   │   │   │   ├── prod.exs
│   │   │   │   └── runtime.exs
│   │   │   ├── epigenetic_web/
│   │   │   │   ├── README.md  ← core
│   │   │   │   └── mix.exs
│   │   │   └── lib/
│   │   │       ├── epigeneticdrift_frontend_web.ex
│   │   │       ├── epigeneticdrift_frontend/
│   │   │       └── epigeneticdrift_frontend_web/
│   │   ├── refs/
│   │   │   ├── PMID_24138928_Horvath_2013_genomebiol_dna_methylation_age_of_human_tissues.md
│   │   │   ├── PMID_29643443_Horvath_2018_natrevgenet_dna_methylation_based_biomarkers_and.md
│   │   │   ├── PMID_30048243_Horvath_2018_aging_albanyny_epigenetic_clock_for_skin_and_blood_.md
│   │   │   ├── PMID_30669119_Lu_2019_aging_albanyny_dna_methylation_grimage_strongly_pre.md
│   │   │   ├── PMID_31085557_Adelman_2019_cancerdiscov_aging_human_hematopoietic_stem_cells.md
│   │   │   ├── PMID_33571444_Deng_2021_cellstemcell_loss_of_kdm4b_exacerbates_bone_fat_i.md
│   │   │   ├── PMID_33844651_Fitzgerald_2021_aging_albanyny_potential_reversal_of_epigenetic_age.md
│   │   │   ├── PMID_34587750_Roberts_2021_circulation_epigenetic_age_and_the_risk_of_incid.md
│   │   │   ├── PMID_35029144_Belsky_2022_elife_dunedinpace_a_dna_methylation_biomar.md
│   │   │   ├── PMID_35032339_Hu_2022_agingcell_nap1l2_drives_mesenchymal_stem_cell_.md
│   │   │   ├── PMID_35858618_Bogeska_2022_cellstemcell_inflammatory_exposure_drives_long_li.md
│   │   │   ├── PMID_36206857_Duan_2022_ageingresrev_epigenetic_clock_a_promising_biomark.md
│   │   │   ├── PMID_36336680_Wang_2022_signaltransducttar_epigenetic_regulation_of_aging_impli.md
│   │   │   ├── PMID_36516495_Lu_2022_aging_albanyny_dna_methylation_grimage_version_2.md
│   │   │   ├── PMID_37034474_Kabacik_2022_nataging_the_relationship_between_epigenetic_.md
│   │   │   ├── PMID_37865087_Kasbekar_2023_cellstemcell_hematopoietic_stem_cells_through_the.md
│   │   │   ├── PMID_37924441_Morandini_2024_geroscience_atac_clock_an_aging_clock_based_on_c.md
│   │   │   ├── PMID_38216430_Wu_2024_trendspharmacolsci_emerging_epigenetic_insights_into_ag.md
│   │   │   ├── PMID_38402617_Kao_2024_cellstemcell_an_iron_rheostat_controls_hematopoie.md
│   │   │   ├── PMID_38482631_Zheng_2024_proteincell_dna_methylation_clocks_for_estimatin.md
│   │   │   ├── PMID_38640057_Yokomizo_2024_curropinhematol_epigenetics_of_hematopoietic_stem_ce.md
│   │   │   ├── PMID_39271425_Meng_2025_trendscellbiol_epigenetic_regulation_of_hematopoiet.md
│   │   │   ├── PMID_39900648_Bischoff-Ferrari_2025_nataging_individual_and_additive_effects_of_v.md
│   │   │   ├── PMID_41289991_Arif_2025_cellstemcell_reversing_lysosomal_dysfunction_rest.md
│   │   │   └── README.md  ← core
│   │   └── scripts/
│   │       └── README.md  ← core
│   ├── MitoROS/
│   │   ├── CONCEPT.md  ← core
│   │   ├── DESIGN.md  ← core
│   │   ├── EVIDENCE.md  ← core
│   │   ├── MAP.md  ← core
│   │   ├── MEMORY.md  ← core
│   │   ├── PARAMETERS.md  ← core
│   │   ├── README.md  ← core
│   │   ├── STATE.md  ← core
│   │   ├── THEORY.md  ← core
│   │   ├── TODO.md  ← core
│   │   ├── _pi.md  ← core
│   │   ├── audits/
│   │   │   └── 2026-05-08/
│   │   │       ├── LC_MitoROS.check.v1.md
│   │   │       ├── LC_MitoROS.plan.v1.md
│   │   │       └── LC_MitoROS.review.md
│   │   ├── backend/
│   │   │   ├── Cargo.toml
│   │   │   ├── Dockerfile
│   │   │   ├── README.md  ← core
│   │   │   ├── migrations/
│   │   │   │   └── 001_initial.sql
│   │   │   └── src/
│   │   │       ├── config.rs
│   │   │       ├── db.rs
│   │   │       ├── error.rs
│   │   │       ├── main.rs
│   │   │       ├── models.rs
│   │   │       └── routes.rs
│   │   ├── crates/
│   │   │   └── mito_ros_counter/
│   │   │       ├── Cargo.lock
│   │   │       ├── Cargo.toml
│   │   │       ├── src/
│   │   │       └── tests/
│   │   ├── data/
│   │   │   └── PARAMETERS_calibrated.json
│   │   ├── docs/
│   │   │   ├── ANALYSIS_Hot_Mitochondrion_Paradox.md
│   │   │   ├── DAILY_SEARCH_2026-04-20.md
│   │   │   ├── DATASETS.md
│   │   │   ├── META_ANALYSIS_Mitochondrial_ROS_Senescence.md
│   │   │   ├── META_ANALYSIS_mtDNA_Heteroplasmy_Accumulation.md
│   │   │   ├── MitoROS_CONCEPT_review.md
│   │   │   ├── MitoROS_full_paper_draft.md
│   │   │   ├── OPEN_PROBLEMS.md
│   │   │   ├── PROJECT_AUDIT_2026-05-12.md
│   │   │   ├── UPGRADE.md
│   │   │   ├── UPGRADE.md.draft
│   │   │   ├── related/
│   │   │   │   ├── parent_CEDAR_CONCEPT.md
│   │   │   │   ├── parent_CEDAR_EVIDENCE.md
│   │   │   │   ├── parent_CEDAR_OPEN_PROBLEMS.md
│   │   │   │   ├── parent_CEDAR_PARAMETERS.md
│   │   │   │   ├── parent_CEDAR_STATE.md
│   │   │   │   ├── parent_CEDAR_THEORY.md
│   │   │   │   ├── parent_LongevityCommon_CONCEPT.md
│   │   │   │   ├── parent_LongevityCommon_EVIDENCE.md
│   │   │   │   ├── parent_LongevityCommon_OPEN_PROBLEMS.md
│   │   │   │   ├── parent_LongevityCommon_PARAMETERS.md
│   │   │   │   ├── parent_LongevityCommon_STATE.md
│   │   │   │   └── parent_LongevityCommon_THEORY.md
│   │   │   └── tbpr/
│   │   │       ├── article_2026-05-09.md
│   │   │       ├── engineering_2026-05-09.md
│   │   │       └── project_2026-05-09.md
│   │   ├── frontend/
│   │   │   ├── mix.exs
│   │   │   ├── config/
│   │   │   │   ├── config.exs
│   │   │   │   ├── dev.exs
│   │   │   │   ├── prod.exs
│   │   │   │   └── runtime.exs
│   │   │   ├── lib/
│   │   │   │   ├── mitoros_frontend_web.ex
│   │   │   │   ├── mitoros_frontend/
│   │   │   │   └── mitoros_frontend_web/
│   │   │   └── mito_ros_web/
│   │   │       ├── README.md  ← core
│   │   │       └── mix.exs
│   │   ├── refs/
│   │   │   ├── PMID_1485738_Nagley_1992_annnyacadsci_mitochondrial_dna_mutation_associate.md
│   │   │   ├── PMID_16868022_Glinka_2006_molbiolevol_evidence_of_gene_conversion_associat.md
│   │   │   ├── PMID_17090418_Wiesner_2006_freeradicres_mitochondrial_dna_damage_and_the_agi.md
│   │   │   ├── PMID_19732859_Leifer_2009_vaccine_modified_live_marker_vaccine_candida.md
│   │   │   ├── PMID_25149213_Khrapko_2014_progmolbioltransls_mitochondrial_dna_mutations_in_aging.md
│   │   │   ├── PMID_26281784_Stewart_2015_natrevgenet_the_dynamics_of_mitochondrial_dna_he.md
│   │   │   ├── PMID_30043489_Lakshmanan_2018_agingcell_clonal_expansion_of_mitochondrial_dn.md
│   │   │   ├── PMID_30089816_Tranah_2018_scirep_mitochondrial_dna_m_3243a_g_heteropl.md
│   │   │   ├── PMID_30593894_Wang_2019_biochimbiophysacta_mitochondrial_regulation_of_cardiac_.md
│   │   │   ├── PMID_36233264_Wang_2022_intjmolsci_nobiletin_prevents_d_galactose_induc.md
│   │   │   ├── PMID_36442091_Insalata_2022_procnatlacadsciusa_stochastic_survival_of_the_densest_a.md
│   │   │   ├── PMID_37172915_Picca_2023_expgerontol_the_contribution_of_mitochondrial_dn.md
│   │   │   ├── PMID_37196864_Guo_2023_ageingresrev_mitochondrial_dysfunction_in_aging.md
│   │   │   ├── PMID_38724734_Nehme_2024_nataging_converting_cell_death_into_senescenc.md
│   │   │   ├── PMID_39019845_Shao_2024_boneres_pdzk1_protects_against_mechanical_ov.md
│   │   │   ├── PMID_39173633_Hahn_2024_cellmetab_misregulation_of_mitochondrial_6ma_p.md
│   │   │   ├── PMID_39179117_Madreiter-Sokolowski_2024_pharmacolther_targeting_organ_specific_mitochondri.md
│   │   │   ├── PMID_39343182_Xian_2024_freeradicbiolmed_human_salivary_histatin_1_regulating.md
│   │   │   ├── PMID_39684855_Kobayashi_2024_intjmolsci_mitochondrial_dna_damage_and_its_rep.md
│   │   │   ├── PMID_39933528_Cefis_2025_cellrepmed_impact_of_physical_activity_on_physi.md
│   │   │   ├── PMID_40183670_Koloko_2025_amjphysiollungcell_hyperoxia_induced_senescence_in_feta.md
│   │   │   ├── PMID_40239706_Gozdecka_2025_nature_mitochondrial_metabolism_sustains_dn.md
│   │   │   ├── PMID_40461459_Xu_2025_natcommun_romo1_overexpression_protects_the_mi.md
│   │   │   ├── PMID_40476552_Kobayashi_2025_intjmolmed_understanding_the_impact_of_mitochon.md
│   │   │   ├── PMID_40500258_Xu_2025_signaltransducttar_mitochondria_in_oxidative_stress_inf.md
│   │   │   ├── PMID_40579478_Zhang_2025_nataging_age_dependent_accumulation_of_mitoch.md
│   │   │   └── README.md  ← core
│   │   └── scripts/
│   │       └── README.md  ← core
│   ├── Proteostasis/
│   │   ├── CONCEPT.md  ← core
│   │   ├── DESIGN.md  ← core
│   │   ├── EVIDENCE.md  ← core
│   │   ├── MAP.md  ← core
│   │   ├── MEMORY.md  ← core
│   │   ├── PARAMETERS.md  ← core
│   │   ├── README.md  ← core
│   │   ├── STATE.md  ← core
│   │   ├── THEORY.md  ← core
│   │   ├── TODO.md  ← core
│   │   ├── _pi.md  ← core
│   │   ├── audits/
│   │   │   └── 2026-05-08/
│   │   │       ├── LC_Proteostasis.check.v1.md
│   │   │       ├── LC_Proteostasis.plan.v1.md
│   │   │       └── LC_Proteostasis.review.md
│   │   ├── backend/
│   │   │   ├── Cargo.toml
│   │   │   ├── Dockerfile
│   │   │   ├── README.md  ← core
│   │   │   ├── migrations/
│   │   │   │   └── 001_initial.sql
│   │   │   └── src/
│   │   │       ├── config.rs
│   │   │       ├── db.rs
│   │   │       ├── error.rs
│   │   │       ├── main.rs
│   │   │       ├── models.rs
│   │   │       └── routes.rs
│   │   ├── crates/
│   │   │   └── proteostasis_counter/
│   │   │       ├── Cargo.lock
│   │   │       ├── Cargo.toml
│   │   │       ├── src/
│   │   │       └── tests/
│   │   ├── data/
│   │   │   └── PARAMETERS_calibrated.json
│   │   ├── docs/
│   │   │   ├── DAILY_SEARCH_2026-04-20.md
│   │   │   ├── DATASETS.md
│   │   │   ├── META_ANALYSIS_Protein_Aggregation_Neurodegeneration.md
│   │   │   ├── META_ANALYSIS_Proteostasis_Network_Aging.md
│   │   │   ├── OPEN_PROBLEMS.md
│   │   │   ├── PROJECT_AUDIT_2026-05-12.md
│   │   │   ├── Proteostasis_CONCEPT_review.md
│   │   │   ├── UPGRADE.md
│   │   │   ├── UPGRADE.md.draft
│   │   │   ├── related/
│   │   │   │   ├── parent_CEDAR_CONCEPT.md
│   │   │   │   ├── parent_CEDAR_EVIDENCE.md
│   │   │   │   ├── parent_CEDAR_OPEN_PROBLEMS.md
│   │   │   │   ├── parent_CEDAR_PARAMETERS.md
│   │   │   │   ├── parent_CEDAR_STATE.md
│   │   │   │   ├── parent_CEDAR_THEORY.md
│   │   │   │   ├── parent_LongevityCommon_CONCEPT.md
│   │   │   │   ├── parent_LongevityCommon_EVIDENCE.md
│   │   │   │   ├── parent_LongevityCommon_OPEN_PROBLEMS.md
│   │   │   │   ├── parent_LongevityCommon_PARAMETERS.md
│   │   │   │   ├── parent_LongevityCommon_STATE.md
│   │   │   │   └── parent_LongevityCommon_THEORY.md
│   │   │   └── tbpr/
│   │   │       ├── article_2026-05-09.md
│   │   │       ├── engineering_2026-05-09.md
│   │   │       └── project_2026-05-09.md
│   │   ├── frontend/
│   │   │   ├── mix.exs
│   │   │   ├── config/
│   │   │   │   ├── config.exs
│   │   │   │   ├── dev.exs
│   │   │   │   ├── prod.exs
│   │   │   │   └── runtime.exs
│   │   │   ├── lib/
│   │   │   │   ├── proteostasis_frontend_web.ex
│   │   │   │   ├── proteostasis_frontend/
│   │   │   │   └── proteostasis_frontend_web/
│   │   │   └── proteostasis_web/
│   │   │       ├── README.md  ← core
│   │   │       └── mix.exs
│   │   ├── refs/
│   │   │   ├── PMID_25615820_Pride_2015_biochembiophysresc_long_lived_species_have_improved_pro.md
│   │   │   ├── PMID_26738589_García-Prat_2016_nature_autophagy_maintains_stemness_by_prev.md
│   │   │   ├── PMID_28170377_Wong_2017_natmed_synuclein_toxicity_in_neurodegenerat.md
│   │   │   ├── PMID_29127110_Klaips_2018_jcellbiol_pathways_of_cellular_proteostasis_in.md
│   │   │   ├── PMID_33891876_Bourdenx_2021_cell_chaperone_mediated_autophagy_prevent.md
│   │   │   ├── PMID_34563704_Kaushik_2021_ageingresrev_autophagy_and_the_hallmarks_of_aging.md
│   │   │   ├── PMID_35447272_Sengupta_2022_progneurobiol_amyloid_tau_and_synuclein_aggregates.md
│   │   │   ├── PMID_37111020_Wang_2023_nutrients_nobiletin_improves_d_galactose_induc.md
│   │   │   ├── PMID_37315555_Sheehan_2023_neuron_an_astrocyte_bmal1_bag3_axis_protect.md
│   │   │   ├── PMID_38049031_Diekman_2024_osteoarthritiscart_aging_and_the_emerging_role_of_cellu.md
│   │   │   ├── PMID_38347288_Wu_2024_actaneuropathol_the_contribution_of_amyloid_tau_and_.md
│   │   │   ├── PMID_39627772_Knecht_2024_jneuroinflammation_autoantibody_profiles_in_alzheimer_s.md
│   │   │   ├── PMID_39973488_Ma_2025_jparkinsonsdis_aging_cellular_senescence_and_parkin.md
│   │   │   ├── PMID_40042672_Buchholz_2025_jneurol_overlapping_presence_of_amyloid_tau_.md
│   │   │   ├── PMID_40098057_Franzmeier_2025_molneurodegener_alpha_synuclein_co_pathology_is_asso.md
│   │   │   ├── PMID_40377064_Folarin_2025_jneurochem_chronic_vanadium_exposure_promotes_a.md
│   │   │   ├── PMID_40388671_Meng_2025_advsci_weinh_histone_lactylation_antagonizes_sene.md
│   │   │   ├── PMID_40960157_Wang_2025_chinmedj_engl_protein_aggregation_in_neurodegenera.md
│   │   │   ├── PMID_41340001_Lourenco_2025_actaneuropathol_nucleolar_aggregation_of_key_neuropa.md
│   │   │   └── README.md  ← core
│   │   └── scripts/
│   │       └── README.md  ← core
│   ├── Telomere/
│   │   ├── CONCEPT.md  ← core
│   │   ├── DESIGN.md  ← core
│   │   ├── EVIDENCE.md  ← core
│   │   ├── MAP.md  ← core
│   │   ├── MEMORY.md  ← core
│   │   ├── PARAMETERS.md  ← core
│   │   ├── README.md  ← core
│   │   ├── STATE.md  ← core
│   │   ├── THEORY.md  ← core
│   │   ├── TODO.md  ← core
│   │   ├── _pi.md  ← core
│   │   ├── audits/
│   │   │   └── 2026-05-08/
│   │   │       ├── LC_Telomere.check.v1.md
│   │   │       ├── LC_Telomere.plan.v1.md
│   │   │       └── LC_Telomere.review.md
│   │   ├── backend/
│   │   │   ├── Cargo.toml
│   │   │   ├── Dockerfile
│   │   │   ├── README.md  ← core
│   │   │   ├── migrations/
│   │   │   │   └── 001_initial.sql
│   │   │   └── src/
│   │   │       ├── config.rs
│   │   │       ├── db.rs
│   │   │       ├── error.rs
│   │   │       ├── main.rs
│   │   │       ├── models.rs
│   │   │       └── routes.rs
│   │   ├── crates/
│   │   │   └── telomere_counter/
│   │   │       ├── Cargo.lock
│   │   │       ├── Cargo.toml
│   │   │       ├── src/
│   │   │       └── tests/
│   │   ├── data/
│   │   │   └── PARAMETERS_calibrated.json
│   │   ├── docs/
│   │   │   ├── DAILY_SEARCH_2026-04-20.md
│   │   │   ├── DATASETS.md
│   │   │   ├── META_ANALYSIS_Telomere_Oxygen_Accelerated_Loss.md
│   │   │   ├── META_ANALYSIS_Telomere_Shortening_Kinetics.md
│   │   │   ├── OPEN_PROBLEMS.md
│   │   │   ├── PROJECT_AUDIT_2026-05-12.md
│   │   │   ├── Telomere_CONCEPT_review.md
│   │   │   ├── UPGRADE.md
│   │   │   ├── UPGRADE.md.draft
│   │   │   ├── related/
│   │   │   │   ├── parent_CEDAR_CONCEPT.md
│   │   │   │   ├── parent_CEDAR_EVIDENCE.md
│   │   │   │   ├── parent_CEDAR_OPEN_PROBLEMS.md
│   │   │   │   ├── parent_CEDAR_PARAMETERS.md
│   │   │   │   ├── parent_CEDAR_STATE.md
│   │   │   │   ├── parent_CEDAR_THEORY.md
│   │   │   │   ├── parent_LongevityCommon_CONCEPT.md
│   │   │   │   ├── parent_LongevityCommon_EVIDENCE.md
│   │   │   │   ├── parent_LongevityCommon_OPEN_PROBLEMS.md
│   │   │   │   ├── parent_LongevityCommon_PARAMETERS.md
│   │   │   │   ├── parent_LongevityCommon_STATE.md
│   │   │   │   └── parent_LongevityCommon_THEORY.md
│   │   │   └── tbpr/
│   │   │       ├── article_2026-05-09.md
│   │   │       ├── engineering_2026-05-09.md
│   │   │       └── project_2026-05-09.md
│   │   ├── frontend/
│   │   │   ├── mix.exs
│   │   │   ├── config/
│   │   │   │   ├── config.exs
│   │   │   │   ├── dev.exs
│   │   │   │   ├── prod.exs
│   │   │   │   └── runtime.exs
│   │   │   ├── lib/
│   │   │   │   ├── telomere_frontend_web.ex
│   │   │   │   ├── telomere_frontend/
│   │   │   │   └── telomere_frontend_web/
│   │   │   └── telomere_web/
│   │   │       ├── README.md  ← core
│   │   │       └── mix.exs
│   │   ├── refs/
│   │   │   ├── PMID_11001793_Jennings_2000_molgenetmetab_nutrition_oxidative_damage_telomere_.md
│   │   │   ├── PMID_13718526_JACOB_1961_jmolbiol_genetic_regulatory_mechanisms_in_the.md
│   │   │   ├── PMID_17938250_Jeganathan_2007_jcellbiol_bub1_mediates_cell_death_in_response.md
│   │   │   ├── PMID_22773427_Wang_2012_circres_aging_and_atherosclerosis_mechanisms.md
│   │   │   ├── PMID_24374808_Zhao_2014_jcellphysiol_telomere_length_maintenance_shorteni.md
│   │   │   ├── PMID_25607366_Mandell_2015_nature_biocontainment_of_genetically_modifi.md
│   │   │   ├── PMID_25612739_Rizvi_2014_curragingsci_telomere_length_variations_in_aging_.md
│   │   │   ├── PMID_28431907_Prasad_2017_mechageingdev_telomere_shortening_during_aging_att.md
│   │   │   ├── PMID_30229407_Zhu_2019_biogerontology_telomere_and_its_role_in_the_aging_p.md
│   │   │   ├── PMID_30472697_Ain_2018_aging_albanyny_cell_cycle_dependent_and_independent.md
│   │   │   ├── PMID_30650660_Liu_2019_cells_roles_of_telomere_biology_in_cell_se.md
│   │   │   ├── PMID_33347069_Luxton_2021_plastreconstrsurg_twins_telomeres_and_aging_in_space.md
│   │   │   ├── PMID_34200513_Pousa_2021_cells_telomere_shortening_and_psychiatric_.md
│   │   │   ├── PMID_34736994_Lin_2022_ageingresrev_stress_and_telomere_shortening_insig.md
│   │   │   ├── PMID_37917279_Medoro_2024_molcellbiochem_nrf2_signaling_pathway_and_telomere_.md
│   │   │   ├── PMID_38581556_Mason_2024_biogerontology_telomeres_and_aging_on_and_off_the_p.md
│   │   │   ├── PMID_38634789_Li_2024_nucleicacidsres_proximal_telomeric_decompaction_due_.md
│   │   │   ├── PMID_39164231_Ghosh_2024_natcommun_riok2_transcriptionally_regulates_tr.md
│   │   │   ├── PMID_39837827_De_2025_natcommun_ogg1_and_mutyh_repair_activities_pro.md
│   │   │   ├── PMID_40215293_Sanz-Moreno_2025_sciadv_loss_of_ten1_in_mice_induces_telomer.md
│   │   │   └── README.md  ← core
│   │   └── scripts/
│   │       └── README.md  ← core
│   ├── _originals/
│   │   ├── CONCEPT.x.md
│   │   ├── EVIDENCE.ru.md
│   │   ├── OPEN_PROBLEMS.ru.md
│   │   ├── README.ru.md
│   │   ├── STATE.ru.md
│   │   ├── THEORY.ru.md
│   │   └── UPGRADE.ru.md
│   ├── audits/
│   │   ├── MCAOA_Evidence_Base_2026-07-07.md
│   │   ├── MCAOA_New_Evidence_June2026.md
│   │   ├── MCAOA_Peer_Review_2026-07-07.docx
│   │   ├── MCAOA_Peer_Review_2026-07-07.md
│   │   ├── PEER_REVIEW_MCARA_v7_2026-07-08.md
│   │   └── 2026-05-08/
│   │       ├── LC_MCAOA.check.v1.md
│   │       ├── LC_MCAOA.plan.v1.md
│   │       └── LC_MCAOA.review.md
│   ├── backend/
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   ├── README.md  ← core
│   │   ├── migrations/
│   │   │   └── 001_initial.sql
│   │   └── src/
│   │       ├── config.rs
│   │       ├── db.rs
│   │       ├── error.rs
│   │       ├── main.rs
│   │       ├── models.rs
│   │       └── routes.rs
│   ├── crates/
│   │   ├── mcoa_api/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       └── main.rs
│   │   ├── mcoa_cli/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       └── main.rs
│   │   ├── mcoa_compare/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       └── bin/
│   │   ├── mcoa_core/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       └── lib.rs
│   │   ├── mcoa_simulation/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       └── lib.rs
│   │   └── mcoa_tests/
│   │       ├── Cargo.toml
│   │       └── src/
│   │           ├── lib.rs
│   │           ├── test1_dominance.rs
│   │           └── test4_aubrey.rs
│   ├── data/
│   │   └── mcoa_run.csv
│   ├── docs/
│   │   ├── CONSORTIUM_ANALYSIS_2026-07-06.md
│   │   ├── C_elegans_Centriole_Map.docx
│   │   ├── C_elegans_Centriole_Map.md
│   │   ├── C_elegans_Centriole_Map_EN.docx
│   │   ├── C_elegans_Centriole_Map_EN.md
│   │   ├── DAILY_SEARCH_2026-04-20.md
│   │   ├── DAILY_SEARCH_2026-04-20_AGGREGATE.md
│   │   ├── DATASETS.md
│   │   ├── MCAOA_CONCEPT_review.md
│   │   ├── OPEN_PROBLEMS.md
│   │   ├── PI_BIO.md
│   │   ├── PROJECT_AUDIT_2026-05-12.md
│   │   ├── UPGRADE.md
│   │   ├── UPGRADE.md.draft
│   │   ├── comparisons/
│   │   │   ├── 2026-04-21_v010_first_run.md
│   │   │   ├── 2026-04-21_5x5_calibrated/
│   │   │   │   ├── INTERPRETATION.md
│   │   │   │   └── matrix_rms_delta.csv
│   │   │   └── 2026-04-21_calibrated/
│   │   │       ├── INTERPRETATION.md
│   │   │       └── matrix_rms_delta.csv
│   │   ├── correspondence/
│   │   │   ├── 16_review_commons_submission_2026-05-13.md
│   │   │   └── 17_elife_reconsideration_2026-05-20.md
│   │   ├── manuscripts/
│   │   │   ├── MCAOA_F1000Research_2026-05-22.docx
│   │   │   ├── MCAOA_F1000Research_2026-05-22.md
│   │   │   ├── BIOGERONTOLOGY/
│   │   │   │   ├── README.md  ← core
│   │   │   │   ├── cover_letter.docx
│   │   │   │   ├── cover_letter.md
│   │   │   │   ├── manuscript.docx
│   │   │   │   └── manuscript.md
│   │   │   └── HAYFLICK_HIERARCHY/
│   │   │       ├── 01_concept_analysis.md
│   │   │       ├── 02_TBPR_article_review.md
│   │   │       ├── 03_PMID_mapping.md
│   │   │       ├── 04_TBPR_strict_IF18_review.md
│   │   │       ├── 05_TBPR_strict_v2_review.md
│   │   │       ├── 06_TBPR_strict_v3_review.md
│   │   │       ├── 07_TBPR_strict_v4_review.md
│   │   │       ├── 08_TBPR_strict_v4_split_review.md
│   │   │       ├── 09_TBPR_strict_v5_split_review.md
│   │   │       ├── 10_TBPR_v6_newtemplate_review.md
│   │   │       ├── 11_TBPR_v7_newtemplate_review.md
│   │   │       ├── 12_TBPR_v8_newtemplate_review.md
│   │   │       ├── 13_TBPR_v9_newtemplate_review.md
│   │   │       ├── 14_TBPR_v10_newtemplate_review.md
│   │   │       └── 15_TBPR_v11_newtemplate_review.md
│   │   ├── related/
│   │   │   ├── parent_LongevityCommon_CONCEPT.md
│   │   │   ├── parent_LongevityCommon_EVIDENCE.md
│   │   │   ├── parent_LongevityCommon_OPEN_PROBLEMS.md
│   │   │   ├── parent_LongevityCommon_PARAMETERS.md
│   │   │   ├── parent_LongevityCommon_STATE.md
│   │   │   └── parent_LongevityCommon_THEORY.md
│   │   └── tbpr/
│   │       ├── article_2026-05-09.md
│   │       ├── engineering_2026-05-09.md
│   │       └── project_2026-05-09.md
│   ├── frontend/
│   │   ├── README.md  ← core
│   │   ├── mix.exs
│   │   ├── config/
│   │   │   ├── config.exs
│   │   │   ├── dev.exs
│   │   │   ├── prod.exs
│   │   │   └── runtime.exs
│   │   └── lib/
│   │       ├── mcaoa_frontend_web.ex
│   │       ├── mcaoa_frontend/
│   │       │   └── application.ex
│   │       ├── mcaoa_frontend_web/
│   │       │   ├── endpoint.ex
│   │       │   ├── router.ex
│   │       │   ├── telemetry.ex
│   │       │   ├── clients/
│   │       │   ├── components/
│   │       │   └── live/
│   │       ├── mcaoa_web/
│   │       │   └── application.ex
│   │       └── mcaoa_web_web/
│   │           └── live/
│   ├── grants/
│   │   └── EIC_Pathfinder_2026/
│   │       ├── CONCEPT_MCARA_v4.5.md
│   │       ├── Concept_note_reverse_experiment_2026-07-10.docx
│   │       ├── Concept_note_reverse_experiment_2026-07-10.md
│   │       ├── Entropy_Counters_Map.md
│   │       ├── Entropy_in_Aging_Handout_MCARA.docx
│   │       ├── Entropy_in_Aging_Handout_MCARA.md
│   │       ├── MCARA.docx
│   │       ├── MCARA.pdf
│   │       ├── MCARA_Article.docx
│   │       ├── MCARA_Article.pdf
│   │       ├── MCARA_Article_v7.md
│   │       ├── MCARA_v2.docx
│   │       ├── PARAMETERS.md  ← core
│   │       ├── STATE.md  ← core
│   │       ├── THEORY.md  ← core
│   │       ├── letters/
│   │       │   ├── MCARA_Response_Wagner_2026-07-07.docx
│   │       │   ├── MCARA_Response_Wagner_2026-07-07.md
│   │       │   ├── Wagner_email_plain.txt
│   │       │   ├── email_LoI_Geiger.md
│   │       │   ├── email_LoI_Jacquemet.md
│   │       │   ├── email_LoI_Magiera.md
│   │       │   ├── email_LoI_Suomalainen.md
│   │       │   ├── email_LoI_Wagner.md
│   │       │   └── email_Trifunovic_C3_MCARA.md
│   │       └── meetings/
│   │           ├── MCARA_OnePager.docx
│   │           ├── MCARA_OnePager.md
│   │           ├── briefing_Wagner_2026-07-11.docx
│   │           └── briefing_Wagner_2026-07-11.md
│   ├── letters/
│   │   └── sent/
│   │       ├── Pierre_Gonczy_Response_2026-07-09.md
│   │       ├── Pierre_Gonczy_Thanks_2026-07-10.md
│   │       ├── letter_Bettencourt-Dias_2026-07-10.md
│   │       └── letter_Cajanek_2026-07-10.md
│   ├── refs/
│   │   ├── HOT_MITOCHONDRION_PARADOX.md
│   │   ├── PMID_12612578_Fontenot_2003_natimmunol_foxp3_programs_the_development_and_f.md
│   │   ├── PMID_12855956_Parrinello_2003_natcellbiol_oxygen_sensitivity_severely_limits_t.md
│   │   ├── PMID_1631178_Dionyssopoulos_1992_pharmacolbiochembe_effect_of_adenosine_analogues_on_the.md
│   │   ├── PMID_16909132_Phernambucq_2006_brjcancer_multicenter_phase_ii_trial_of_accele.md
│   │   ├── PMID_2038241_Ratib_1991_mdcomput_desktop_image_analysis_workstations_.md
│   │   ├── PMID_2342578_Harley_1990_nature_telomeres_shorten_during_ageing_of_h.md
│   │   ├── PMID_26833090_Trego_2016_molcell_non_catalytic_roles_for_xpg_with_brc.md
│   │   ├── PMID_28132843_Sundaramoorthy_2017_molcell_znf598_and_rack1_regulate_mammalian_.md
│   │   ├── PMID_29227991_Sonney_2017_ploscomputbiol_predicting_the_pathogenicity_of_nove.md
│   │   ├── PMID_29643502_Goriki_2018_natrevurol_unravelling_disparate_roles_of_notch.md
│   │   ├── PMID_30174316_Wu_2018_stemcellreports_a_chemical_recipe_for_generation_of_.md
│   │   ├── PMID_30982602_Kucab_2019_cell_a_compendium_of_mutational_signature.md
│   │   ├── PMID_31844045_Park_2019_natcommun_atad5_promotes_replication_restart_b.md
│   │   ├── PMID_33268865_Lu_2020_nature_reprogramming_to_recover_youthful_ep.md
│   │   └── README.md  ← core
│   ├── results/
│   │   ├── mcoa_beta_cell_100.csv
│   │   ├── mcoa_cd8_t_memory_100.csv
│   │   ├── mcoa_fibroblast_100.csv
│   │   ├── mcoa_hepatocyte_100.csv
│   │   ├── mcoa_hsc_100.csv
│   │   └── mcoa_neuron_100.csv
│   └── бизнесмены/
│       ├── Алекс_F/
│       └── Джозеф_Джекс/
│           ├── CEDAR.pdf
│           ├── Jacks_meeting_prep.docx
│           ├── Jacks_meeting_prep.md
│           ├── Jacks_meeting_script.docx
│           ├── Jacks_meeting_script.md
│           ├── MCARA.pdf
│           ├── email_to_Jacks.md
│           └── response_joseph_jacks.md
├── Organismal_Aging/
│   ├── CONCEPT.md  ← core
│   ├── DESIGN.md  ← core
│   ├── EVIDENCE.md  ← core
│   ├── MAP.md  ← core
│   ├── MEMORY.md  ← core
│   ├── PARAMETERS.md  ← core
│   ├── README.md  ← core
│   ├── STATE.md  ← core
│   ├── THEORY.md  ← core
│   ├── TODO.md  ← core
│   ├── _pi.md  ← core
│   ├── audits/
│   │   ├── AUDIT_2026-06-21.md
│   │   ├── PEER_REVIEW_2026-06-21.md
│   │   ├── PEER_REVIEW_FINAL_2026-06-21.md
│   │   └── PEER_REVIEW_v3_LEVEL11.md
│   └── docs/
│       ├── EIC_PATHFINDER.md
│       └── OPEN_PROBLEMS.md
├── Ze/
│   ├── CONCEPT.md  ← core
│   ├── MAP.md  ← core
│   ├── MEMORY.md  ← core
│   ├── PARAMETERS.md  ← core
│   ├── README.md  ← core
│   ├── STATE.md  ← core
│   ├── TODO.md  ← core
│   ├── Ze.docx
│   ├── Ze.md
│   ├── Ze_EN.md
│   ├── Ze_Z2.docx
│   ├── Ze_Z2_abstract.md
│   ├── Ze_Z2_cover_letter.docx
│   ├── Ze_Z2_cover_letter.md
│   ├── Ze_and Centrioles.pdf
│   ├── _pi.md  ← core
│   ├── Articles/
│   │   ├── BIOSYS_POSTMORTEM.md
│   │   ├── Collatz Dynamics as a Ze-System.pdf
│   │   ├── Ze Theory as an Interpretive Framework for Quantum Mechanics.pdf
│   │   ├── Ze_CHSH_Entropy_Quantum.md
│   │   ├── Ze_CHSH_Entropy_Quantum_v3_en.md
│   │   ├── Ze_CHSH_Entropy_Quantum_v3_ru.md
│   │   ├── Ze_Z2_Gauge_English.docx
│   │   ├── Ze_Z2_Gauge_English.md
│   │   ├── Ze_Z2_Gauge_English.pdf
│   │   ├── Ze_Z2_cover_letter.docx
│   │   ├── Ze_Z2_cover_letter.md
│   │   ├── cover_letter_biosystems.md
│   │   ├── fig1_three_layers.svg
│   │   ├── fig2_test_matrix.svg
│   │   ├── submission_plan.md
│   │   ├── Allostasis_Ze/
│   │   │   ├── _gen1.log
│   │   │   ├── _prompt_part1.txt
│   │   │   ├── _prompt_refs.txt
│   │   │   ├── _refs_err.log
│   │   │   ├── _refs_failed.md
│   │   │   ├── _refs_final.md
│   │   │   ├── _refs_raw.md
│   │   │   ├── _refs_research.json
│   │   │   ├── _refs_research.md
│   │   │   ├── _refs_table.tsv
│   │   │   ├── _refs_verified.md
│   │   │   └── _self_citations.md
│   │   ├── Entropy_2026/
│   │   │   ├── Cover_Letter_Ze_Multilayer_Age_Entropy.docx
│   │   │   ├── Cover_Letter_Ze_Multilayer_Age_Entropy.md
│   │   │   ├── Entropy of Age Distribution as a Causal Driver of System Dysfunction.md
│   │   │   ├── Entropy of Age.docx
│   │   │   ├── Entropy_of_Age_ANON.docx
│   │   │   ├── Entropy_of_Age_Distribution_Bristlebot_Swarm_Model.docx
│   │   │   ├── cover_letter_JTB.docx
│   │   │   ├── cover_letter_JTB.md
│   │   │   ├── fig1_three_layers.png
│   │   │   ├── fig1_three_layers.svg
│   │   │   ├── fig2_test_matrix.png
│   │   │   └── fig2_test_matrix.svg
│   │   ├── Quantum/
│   │   │   ├── EDITORIAL_PLAN.md
│   │   │   ├── QUANTUM_REQUIREMENTS.md
│   │   │   ├── SUBMISSION_README.md
│   │   │   ├── Ze_CHSH_Entropy_Quantum_v1.md
│   │   │   ├── Ze_CHSH_Entropy_Quantum_v3_ru.md
│   │   │   ├── Ze_CHSH_Entropy_Quantum_v4_en.md
│   │   │   ├── cover_letter.md
│   │   │   ├── example-plot.pdf
│   │   │   ├── main.log
│   │   │   ├── main.tex
│   │   │   ├── main_preview.log
│   │   │   ├── main_preview.tex
│   │   │   ├── main_submit.tex
│   │   │   ├── quantum.bst
│   │   │   ├── quantum_submission.zip
│   │   │   ├── quantum_template.html
│   │   │   ├── quantumarticle.cls
│   │   │   ├── references.bib
│   │   │   ├── figures/
│   │   │   │   └── S_H_plot.pdf
│   │   │   ├── quantum_template/
│   │   │   │   ├── README.md  ← core
│   │   │   │   ├── install.ps1
│   │   │   │   ├── install.sh
│   │   │   │   ├── quantum-bibliographystyle-demo.bib
│   │   │   │   ├── quantum-bibliographystyle-demo.pdf
│   │   │   │   ├── quantum-bibliographystyle-demo.tex
│   │   │   │   ├── quantum-lyx-template.lyx
│   │   │   │   ├── quantum-template.pdf
│   │   │   │   ├── quantum-template.tex
│   │   │   │   ├── quantum.bst
│   │   │   │   ├── quantumarticle.cls
│   │   │   │   ├── quantumarticle.layout
│   │   │   │   ├── quantumarticle.pdf
│   │   │   │   ├── quantumarticle.tex
│   │   │   │   ├── quantumview-template.bib
│   │   │   │   ├── quantumview-template.pdf
│   │   │   │   ├── quantumview-template.tex
│   │   │   │   └── quantumview.cls
│   │   │   └── submission/
│   │   │       ├── SUBMISSION_README.md
│   │   │       ├── cover_letter.md
│   │   │       ├── main.tex
│   │   │       ├── quantum.bst
│   │   │       ├── quantumarticle.cls
│   │   │       └── references.bib
│   │   └── _archive/
│   │       ├── multilayer_age_concept_original.md
│   │       └── multilayer_age_removed/
│   │           ├── multilayer_age_concept.md
│   │           └── multilayer_age_concept_v2.md
│   ├── Materials/
│   │   └── 20260702_QED_in_Ze_Formalism/
│   │       └── README.md  ← core
│   ├── Ze-Hierarchy/
│   │   ├── CONCEPT.md  ← core
│   │   ├── MAP.md  ← core
│   │   ├── MEMORY.md  ← core
│   │   ├── PARAMETERS.md  ← core
│   │   ├── README.md  ← core
│   │   ├── STATE.md  ← core
│   │   ├── TODO.md  ← core
│   │   ├── _pi.md  ← core
│   │   ├── data/
│   │   │   ├── result_big.pkl
│   │   │   └── sim_result.npz
│   │   ├── docs/
│   │   │   ├── GRANTS.md
│   │   │   ├── GRANTS_REPORT.md
│   │   │   ├── PEER_REVIEW_STRICT.md
│   │   │   └── flash_grant_letter.md
│   │   ├── grants/
│   │   │   └── COLLABORATORS_FUNDS.md
│   │   └── hardware/
│   │       ├── ASSEMBLY.md
│   │       ├── README.md  ← core
│   │       ├── bom.md
│   │       ├── schematic.md
│   │       └── firmware/
│   │           └── ze_bot.ino
│   ├── Ze_CHSH/
│   │   ├── CONCEPT.md  ← core
│   │   ├── MAP.md  ← core
│   │   ├── MEMORY.md  ← core
│   │   ├── PARAMETERS.md  ← core
│   │   ├── README.md  ← core
│   │   ├── STATE.md  ← core
│   │   ├── TODO.md  ← core
│   │   ├── _pi.md  ← core
│   │   ├── docs/
│   │   │   ├── Ze_CHSH_Entropy_Quantum.md
│   │   │   ├── Ze_CHSH_Entropy_Quantum_v3_en.md
│   │   │   ├── Ze_CHSH_Entropy_Quantum_v3_ru.md
│   │   │   ├── cover_letter_FoP.docx
│   │   │   ├── cover_letter_FoP.md
│   │   │   └── cover_letter_FoP.pdf
│   │   └── grants/
│   │       └── COLLABORATORS_FUNDS.md
│   ├── Ze_CHSH_QIP_submission/
│   │   ├── Ze_CHSH_QIP.zip
│   │   ├── cover_letter.docx
│   │   ├── cover_letter.md
│   │   ├── main_submit.tex
│   │   ├── quantum.bst
│   │   ├── quantumarticle.cls
│   │   ├── references.bib
│   │   └── figures/
│   │       └── S_H_plot.pdf
│   ├── Ze_D/
│   │   ├── CONCEPT.md  ← core
│   │   ├── MAP.md  ← core
│   │   ├── MEMORY.md  ← core
│   │   ├── PARAMETERS.md  ← core
│   │   ├── README.md  ← core
│   │   ├── STATE.md  ← core
│   │   ├── TODO.md  ← core
│   │   ├── _pi.md  ← core
│   │   ├── Articles/
│   │   │   ├── D_Ze_BioSystems/
│   │   │   │   ├── BIOSYS-S-26-00960.pdf
│   │   │   │   ├── README_submit.md
│   │   │   │   ├── cover_letter.docx
│   │   │   │   ├── cover_letter.md
│   │   │   │   ├── declaration_competing_interests.docx
│   │   │   │   ├── manuscript.docx
│   │   │   │   └── manuscript.md
│   │   │   └── Ze_D_BioSystems/
│   │   │       ├── cover_letter.docx
│   │   │       ├── cover_letter_biosystems.md
│   │   │       ├── declaration.docx
│   │   │       ├── declaration.md
│   │   │       ├── manuscript.docx
│   │   │       └── manuscript.md
│   │   └── grants/
│   │       └── FUNDS_COLLABORATORS.md
│   ├── Ze_Model/
│   │   ├── CONCEPT.md  ← core
│   │   ├── MAP.md  ← core
│   │   ├── MEMORY.md  ← core
│   │   ├── PARAMETERS.md  ← core
│   │   ├── README.md  ← core
│   │   ├── STATE.md  ← core
│   │   ├── TODO.md  ← core
│   │   ├── _pi.md  ← core
│   │   └── Articles/
│   │       └── Ze_Model_FoP/
│   │           ├── README_submit.md
│   │           ├── cover_letter.docx
│   │           ├── cover_letter.md
│   │           ├── manuscript.docx
│   │           └── manuscript.md
│   ├── audits/
│   │   └── 2026-05-08/
│   │       ├── LC_Ze.check.v1.md
│   │       ├── LC_Ze.check.v2.md
│   │       ├── LC_Ze.merged_review.v2.md
│   │       ├── LC_Ze.plan.v1.md
│   │       ├── LC_Ze.plan.v2.md
│   │       └── LC_Ze.review.md
│   ├── bristlebot_sim/
│   │   ├── README.md  ← core
│   │   ├── demo_results.json
│   │   ├── requirements.txt
│   │   └── sim_output.log
│   ├── docs/
│   │   ├── Derivation_of_Alpha.md
│   │   ├── Experiment_Collective_RNG_Automata.md
│   │   ├── MASTER_PUBLICATIONS_GRANTS.md
│   │   ├── PROJECT_AUDIT_2026-05-12.md
│   │   ├── Peoch_Experiment_and_Ze.md
│   │   ├── Space_As_Ze_Error.md
│   │   ├── TBPR_review_Ze-allostasis-v2_2026-05-09.md
│   │   ├── TBPR_review_Ze-allostasis-v3_2026-05-09.md
│   │   ├── TBPR_review_Ze-allostasis_2026-05-09.md
│   │   ├── related/
│   │   │   ├── parent_LongevityCommon_CONCEPT.md
│   │   │   ├── parent_LongevityCommon_EVIDENCE.md
│   │   │   ├── parent_LongevityCommon_OPEN_PROBLEMS.md
│   │   │   ├── parent_LongevityCommon_PARAMETERS.md
│   │   │   ├── parent_LongevityCommon_STATE.md
│   │   │   └── parent_LongevityCommon_THEORY.md
│   │   └── tbpr/
│   │       ├── article_2026-05-09.md
│   │       ├── engineering_2026-05-09.md
│   │       └── project_2026-05-09.md
│   ├── grants/
│   │   ├── GRANTS.md
│   │   └── Ze_Hierarchy_NLNet_submitted.pdf
│   ├── refs/
│   │   ├── PMID_20480236_Lezhava_2011_biogerontology_gerontology_research_in_georgia.md
│   │   ├── PMID_36583780_Tkemaladze_2023_molbiolrep_reduction_proliferation_and_differen.md
│   │   ├── README.md  ← core
│   │   └── SSRN_6864482_Tkemaladze_2026_multilayer_age_ze_discordance.md
│   ├── simulations/
│   │   ├── audit_run.py
│   │   ├── classical_mc/
│   │   │   ├── README.md  ← core
│   │   │   ├── analyze.py
│   │   │   ├── requirements.txt
│   │   │   ├── results_quick.json
│   │   │   └── ze_mc.py
│   │   ├── d3p1d_mc/
│   │   │   └── ze_4d_mc.py
│   │   ├── quantum_4d/
│   │   │   ├── Cargo.lock
│   │   │   ├── Cargo.toml
│   │   │   ├── audit_apply.py
│   │   │   ├── benches/
│   │   │   │   └── wolff_bench.rs
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       └── main.rs
│   │   └── quantum_mc/
│   │       ├── qmc_results_quick.json
│   │       └── ze_qmc.py
│   ├── simulator/
│   │   ├── CONCEPT.md  ← core
│   │   ├── Cargo.toml
│   │   ├── MAP.md  ← core
│   │   ├── MEMORY.md  ← core
│   │   ├── PARAMETERS.md  ← core
│   │   ├── README.md  ← core
│   │   ├── STATE.md  ← core
│   │   ├── TODO.md  ← core
│   │   ├── _pi.md  ← core
│   │   ├── data/
│   │   │   └── bootstrap_vstar_results.json
│   │   ├── scripts/
│   │   │   └── build.sh
│   │   ├── ze-core/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       ├── quantum.rs
│   │   │       ├── reproduction.rs
│   │   │       ├── tests.rs
│   │   │       ├── thermo.rs
│   │   │       └── types.rs
│   │   └── ze-runner/
│   │       ├── Cargo.toml
│   │       └── src/
│   │           └── main.rs
│   ├── website/
│   │   └── ze_sim/
│   │       ├── CONCEPT.md  ← core
│   │       ├── MAP.md  ← core
│   │       ├── MEMORY.md  ← core
│   │       ├── PARAMETERS.md  ← core
│   │       ├── README.md  ← core
│   │       ├── STATE.md  ← core
│   │       ├── TODO.md  ← core
│   │       ├── _pi.md  ← core
│   │       ├── mix.exs
│   │       ├── mix.lock
│   │       ├── assets/
│   │       │   ├── tailwind.config.js
│   │       │   ├── css/
│   │       │   ├── js/
│   │       │   └── vendor/
│   │       ├── config/
│   │       │   ├── config.exs
│   │       │   ├── dev.exs
│   │       │   ├── prod.exs
│   │       │   ├── runtime.exs
│   │       │   └── test.exs
│   │       ├── lib/
│   │       │   ├── ze_sim.ex
│   │       │   ├── ze_sim_web.ex
│   │       │   ├── ze_sim/
│   │       │   └── ze_sim_web/
│   │       ├── priv/
│   │       │   ├── gettext/
│   │       │   └── static/
│   │       ├── scripts/
│   │       │   └── run.sh
│   │       └── test/
│   │           ├── test_helper.exs
│   │           ├── support/
│   │           └── ze_sim_web/
│   ├── ze-public/
│   │   ├── LICENSE  ← core
│   │   ├── README.md  ← core
│   │   ├── docs/
│   │   │   ├── FINAL_SYNTHESIS.md
│   │   │   ├── FRADKIN_SHENKER_COMPARISON.md
│   │   │   ├── META_ANALYSIS.md
│   │   │   ├── PAPER.docx
│   │   │   ├── PAPER.md
│   │   │   ├── PAPER_RU.md
│   │   │   ├── RESPONSE_TO_REVIEWER.md
│   │   │   ├── RESPONSE_TO_REVIEWER_2.md
│   │   │   ├── THEORY.md  ← core
│   │   │   └── THEORY_OF_EVERYTHING.md
│   │   └── simulations/
│   │       ├── Gamma_star_results.json
│   │       ├── L_closed_form.json
│   │       ├── L_results.json
│   │       ├── alpha_closed_form.py
│   │       ├── alpha_final.json
│   │       ├── alpha_solved.json
│   │       ├── audit_run.py
│   │       ├── closed_form_L.py
│   │       ├── compute_L_from_HZe.py
│   │       ├── compute_alpha.py
│   │       ├── dirac_operator_v.py
│   │       ├── extrapolation_final.json
│   │       ├── final_5.json
│   │       ├── final_5_requirements.py
│   │       ├── find_Gamma_star.py
│   │       ├── proof_results.json
│   │       ├── prove_and_benchmark.py
│   │       ├── solve_all_5_problems.py
│   │       ├── classical_mc/
│   │       │   ├── README.md  ← core
│   │       │   ├── analyze.py
│   │       │   ├── requirements.txt
│   │       │   ├── results_quick.json
│   │       │   └── ze_mc.py
│   │       ├── d3p1d_mc/
│   │       │   ├── requirements.txt
│   │       │   └── ze_4d_mc.py
│   │       ├── exact_diag/
│   │       │   ├── ed_cluster.py
│   │       │   └── gauge_invariant_ed.py
│   │       ├── pyrochlore/
│   │       │   ├── direct_alpha.py
│   │       │   ├── direct_g_from_HZe.py
│   │       │   ├── fss_alpha.json
│   │       │   ├── hexagons_L3.json
│   │       │   ├── hexagons_L4.json
│   │       │   ├── hexagons_L5.json
│   │       │   ├── hexagons_L6.json
│   │       │   ├── hexagons_L7.json
│   │       │   ├── hexagons_L8.json
│   │       │   ├── measure_xi.py
│   │       │   ├── pyro_lattice.py
│   │       │   ├── qmc_pyro_L2_M8.json
│   │       │   ├── qmc_pyro_L3_M16.json
│   │       │   ├── requirements.txt
│   │       │   ├── sse_pyro.py
│   │       │   ├── xi_measurement_L7.json
│   │       │   ├── ze_pyro.py
│   │       │   ├── ze_qmc_pyro.py
│   │       │   └── src/
│   │       ├── quantum_4d/
│   │       │   ├── Cargo.lock
│   │       │   ├── Cargo.toml
│   │       │   ├── benches/
│   │       │   └── src/
│   │       └── quantum_mc/
│   │           ├── qmc_results_quick.json
│   │           ├── requirements.txt
│   │           └── ze_qmc.py
│   └── ze-web/
│       ├── config/
│       │   └── runtime.exs
│       └── lib/
│           └── ze_web_web/
│               ├── components/
│               └── live/
├── _archive/
│   ├── CONCEPT.md.bak
│   ├── CONCEPT.md.best.bak
│   ├── CONCEPT.md.best.json
│   ├── CONCEPT.md.mbpr.json
│   ├── CONCEPT.md.mbpr.md
│   ├── CONCEPT.md.recommendations.md
│   ├── CONCEPT.md.rollback.bak
│   ├── MAP.md.bak
│   ├── MAP.md.best.bak
│   ├── MAP.md.best.json
│   ├── MAP.md.fix_bak
│   ├── MAP.md.mbpr.json
│   ├── MAP.md.mbpr.md
│   ├── MAP.md.rollback.bak
│   ├── MEMORY.md.bak
│   ├── MEMORY.md.best.bak
│   ├── MEMORY.md.best.json
│   ├── MEMORY.md.final_bak
│   ├── MEMORY.md.fix_bak
│   ├── MEMORY.md.mbpr.json
│   ├── MEMORY.md.mbpr.md
│   ├── MEMORY.md.rollback.bak
│   ├── OPEN_PROBLEMS.md.audit_202606211652
│   ├── OPEN_PROBLEMS.md.audit_202606232059
│   ├── PARAMETERS.md.bak
│   ├── PARAMETERS.md.best.bak
│   ├── PARAMETERS.md.best.json
│   ├── PARAMETERS.md.fix_bak
│   ├── PARAMETERS.md.mbpr.json
│   ├── PARAMETERS.md.mbpr.md
│   ├── PARAMETERS.md.rollback.bak
│   ├── PROJECT_AUDIT_2026-05-12.md
│   ├── README.md.bak.iter1.20260517_005714
│   ├── README.md.bak.iter2.20260517_005902
│   ├── README.md.best.bak
│   ├── README.md.best.json
│   ├── README.md.mbpr.json
│   ├── README.md.mbpr.md
│   ├── README.md.rollback.bak
│   ├── STATE.md.bak
│   ├── STATE.md.best.bak
│   ├── STATE.md.best.json
│   ├── STATE.md.final_bak
│   ├── STATE.md.fix_bak
│   ├── STATE.md.mbpr.json
│   ├── STATE.md.mbpr.md
│   ├── STATE.md.rollback.bak
│   ├── TODO.md.bak
│   ├── TODO.md.best.bak
│   ├── TODO.md.best.json
│   ├── TODO.md.final_bak
│   ├── TODO.md.fix_bak
│   ├── TODO.md.mbpr.json
│   ├── TODO.md.mbpr.md
│   ├── TODO.md.rollback.bak
│   ├── Ze_final.md.recommendations.md
│   ├── Ze_review.md.recommendations.md
│   ├── _pi.md.bak
│   ├── _pi.md.best.bak
│   ├── _pi.md.best.json
│   ├── _pi.md.fix_bak
│   ├── _pi.md.mbpr.json
│   ├── _pi.md.mbpr.md
│   ├── _pi.md.rollback.bak
│   ├── multilayer_age_concept.md.best.bak
│   ├── multilayer_age_concept.md.best.bak.tmp
│   ├── multilayer_age_concept.md.best.json
│   ├── multilayer_age_concept.md.recommendations.md
│   ├── multilayer_age_concept.md.rollback.bak
│   ├── pi.md
│   ├── _audits/
│   │   ├── AUDIT_DEEP_2026-05-07.md
│   │   ├── COMMIT_LOG_2026-04-26.md
│   │   ├── COMMIT_LOG_FINAL_2026-04-26.md
│   │   ├── PEER_REVIEW_v2_Empirical_2026-04-26.md
│   │   ├── PEER_REVIEW_v2_Funds_2026-04-26.md
│   │   ├── PEER_REVIEW_v2_TopMCAOAZe_2026-04-26.md
│   │   └── REMEDIATION_ROADMAP_2026-05-07.md
│   ├── _originals/
│   │   ├── OPEN_PROBLEMS.x.md
│   │   ├── README.ru.md
│   │   ├── STATE.x.md
│   │   ├── THEORY.ru.md
│   │   └── TODO.x.md
│   ├── audits/
│   │   └── 2026-05-08/
│   │       ├── LongevityCommon_root.check.v1.md
│   │       ├── LongevityCommon_root.plan.v1.md
│   │       ├── LongevityCommon_root.review.md
│   │       ├── _funds/
│   │       │   ├── EXECUTIVE_SUMMARY.md
│   │       │   ├── FINAL_CYCLE_REPORT.md
│   │       │   ├── INTERIM_PROGRESS.md
│   │       │   ├── PROMPT_AUDIT_FUNDS_v3.0.md
│   │       │   ├── PROMPT_v2.md
│   │       │   ├── REFERENCES_TO_VERIFY.md
│   │       │   ├── REFERENCES_VERIFIED.md
│   │       │   ├── refs_verified.json
│   │       │   ├── _archive_v2.1/
│   │       │   └── _archive_v3.0_strict/
│   │       ├── _umbrella/
│   │       │   ├── AUDIT_REPORT_2026-05-08.md
│   │       │   ├── FIXES_LOG.md
│   │       │   ├── _synthesis_packet.md
│   │       │   ├── build_packet.py
│   │       │   ├── bundle_for_synthesis.md
│   │       │   ├── ds_review.py
│   │       │   ├── finalize.py
│   │       │   ├── inventory.json
│   │       │   ├── orchestrator.py
│   │       │   ├── spec_review.py
│   │       │   ├── status.json
│   │       │   ├── synthesis.md
│   │       │   ├── logs/
│   │       │   └── packets/
│   │       ├── deploy/
│   │       │   ├── LC_deploy.check.v1.md
│   │       │   ├── LC_deploy.plan.v1.md
│   │       │   └── LC_deploy.review.md
│   │       ├── realtime/
│   │       │   ├── LC_realtime.check.v1.md
│   │       │   ├── LC_realtime.plan.v1.md
│   │       │   └── LC_realtime.review.md
│   │       ├── server/
│   │       │   ├── LC_server.check.v1.md
│   │       │   ├── LC_server.plan.v1.md
│   │       │   ├── LC_server.review.md
│   │       │   ├── srv_aim/
│   │       │   ├── srv_books/
│   │       │   ├── srv_drjaba/
│   │       │   ├── srv_drjaba-shared/
│   │       │   ├── srv_fclc/
│   │       │   ├── srv_ksystem/
│   │       │   ├── srv_longevity/
│   │       │   ├── srv_longevitycommon/
│   │       │   ├── srv_ngo/
│   │       │   ├── srv_space/
│   │       │   └── srv_spellcheckerka/
│   │       └── web/
│   │           ├── LC_web.check.v1.md
│   │           ├── LC_web.check.v2.md
│   │           ├── LC_web.merged_review.v2.md
│   │           ├── LC_web.plan.v1.md
│   │           ├── LC_web.plan.v2.md
│   │           └── LC_web.review.md
│   ├── halted/
│   │   ├── HAP/
│   │   │   ├── CONCEPT.md  ← core
│   │   │   ├── KNOWLEDGE.md
│   │   │   ├── LINKS.md
│   │   │   ├── MAP.md  ← core
│   │   │   ├── MEMORY.md  ← core
│   │   │   ├── PARAMETERS.md  ← core
│   │   │   ├── README.md  ← core
│   │   │   ├── TODO.md  ← core
│   │   │   ├── UPGRADE.md
│   │   │   ├── articles/
│   │   │   │   └── The_Hepato-Affective_Primacy_HAP_Theory.docx
│   │   │   ├── audits/
│   │   │   │   └── 2026-05-08/
│   │   │   └── docs/
│   │   │       └── tbpr/
│   │   └── Ontogenesis/
│   │       ├── audits/
│   │       │   └── 2026-05-08/
│   │       ├── data/
│   │       └── scripts/
│   └── subprojects_concepts/
│       ├── BioSense_CONCEPT.md
│       ├── BioSense_MAP.md
│       ├── BioSense_MEMORY.md
│       ├── BioSense_PARAMETERS.md
│       ├── BioSense_STATE.md
│       ├── FCLC_CONCEPT.md
│       ├── FCLC_MAP.md
│       ├── FCLC_MEMORY.md
│       ├── FCLC_PARAMETERS.md
│       ├── FCLC_STATE.md
│       ├── HAP_CONCEPT.md
│       ├── HAP_MAP.md
│       ├── HAP_MEMORY.md
│       ├── HAP_PARAMETERS.md
│       ├── HAP_STATE.md
│       ├── MCAOA_CDATA_CONCEPT.md
│       ├── MCAOA_CDATA_DESIGN.md
│       ├── MCAOA_CDATA_EVIDENCE.md
│       ├── MCAOA_CDATA_MAP.md
│       ├── MCAOA_CDATA_MEMORY.md
│       ├── MCAOA_CDATA_PARAMETERS.md
│       ├── MCAOA_CDATA_STATE.md
│       ├── MCAOA_CDATA_THEORY.md
│       ├── MCAOA_CONCEPT.md
│       ├── MCAOA_DESIGN.md
│       ├── MCAOA_EVIDENCE.md
│       ├── MCAOA_MAP.md
│       ├── MCAOA_MEMORY.md
│       ├── MCAOA_PARAMETERS.md
│       ├── MCAOA_STATE.md
│       ├── MCAOA_THEORY.md
│       ├── Ze_CONCEPT.md
│       ├── Ze_MAP.md
│       ├── Ze_MEMORY.md
│       ├── Ze_PARAMETERS.md
│       └── Ze_STATE.md
├── docs/
│   ├── CORRECTION_CANDIDATES.md
│   ├── DEPLOY_CONVENTION.md
│   ├── Email_Fraunhofer_IESE_FINAL.docx
│   ├── Email_Fraunhofer_IESE_FINAL.md
│   ├── Email_Fraunhofer_KULeuven_drafts.docx
│   ├── Email_Fraunhofer_KULeuven_drafts.md
│   ├── Email_KULeuven_COSIC_FINAL.docx
│   ├── Email_KULeuven_COSIC_FINAL.md
│   ├── HIA_GLA_v1_DRAFT.docx
│   ├── HIA_GLA_v1_DRAFT.md
│   ├── META_ANALYSIS_LongevityCommon.md
│   ├── NEWS.docx
│   ├── NEWS.md
│   ├── OPEN_PROBLEMS.md
│   ├── PEER_REVIEW_LongevityCommon.md
│   ├── PI_TRACK_RECORD.md
│   ├── REFERENCE_AUDIT_LongevityCommon.md
│   ├── SUPERVISORY_BOARD_GLA_candidates.docx
│   ├── SUPERVISORY_BOARD_GLA_candidates.md
│   ├── TRUE_MISMATCHES_LongevityCommon.md
│   ├── EIC_PartB_2026/
│   │   ├── 00_COVER.md
│   │   └── WP3_OnePage_2026-04-28.txt
│   ├── HFG/
│   │   ├── Audit_Requirement_Explanation_KA.docx
│   │   ├── Audit_Requirement_Explanation_KA.md
│   │   ├── Audit_Requirement_Explanation_KA.pdf
│   │   ├── CONSORTIUM_v2.docx
│   │   ├── CONSORTIUM_v2.md
│   │   ├── CONSORTIUM_v2.pdf
│   │   ├── EMAIL_GHF_DRAFT.md
│   │   ├── HIA_GHF_DRAFT.md
│   │   ├── HIA_v3.docx
│   │   ├── HIA_v3.md
│   │   ├── HIA_v3.pdf
│   │   ├── TSERTSVADZE_EMAIL_v2.docx
│   │   ├── TSERTSVADZE_EMAIL_v2.md
│   │   ├── TSERTSVADZE_EMAIL_v2.pdf
│   │   └── განახლებული ამონაწერი საჯარო რეესტრიდან.pdf
│   ├── _originals/
│   │   └── NEWS.ru.md
│   ├── attachments/
│   │   ├── LongevityCommon_ConceptNote.docx
│   │   ├── LongevityCommon_ConceptNote.md
│   │   ├── LongevityCommon_ConceptNote.pdf
│   │   ├── LongevityCommon_Cryptographic_Scope_COSIC.docx
│   │   ├── LongevityCommon_Cryptographic_Scope_COSIC.md
│   │   ├── LongevityCommon_Cryptographic_Scope_COSIC.pdf
│   │   ├── LongevityCommon_WP1_FCLC_FraunhoferDraft.docx
│   │   ├── LongevityCommon_WP1_FCLC_FraunhoferDraft.md
│   │   └── LongevityCommon_WP1_FCLC_FraunhoferDraft.pdf
│   ├── data/
│   │   └── registry.json
│   ├── deploy/
│   │   ├── DEPLOY.md
│   │   ├── docker-compose-all.OLD-pre-v5.6.yml
│   │   ├── docker-compose-all.yml
│   │   ├── nginx/
│   │   │   ├── app.longevity.ge.conf
│   │   │   ├── biosense.longevity.ge.conf
│   │   │   └── ze.longevity.ge.conf
│   │   ├── scripts/
│   │   │   ├── deploy-app-native.sh
│   │   │   └── deploy_all.sh
│   │   ├── server-state/
│   │   │   ├── README.md  ← core
│   │   │   ├── nginx-snippets/
│   │   │   │   ├── eco-inject.conf
│   │   │   │   ├── hero-blue.conf
│   │   │   │   └── hive-button.conf
│   │   │   ├── nginx-vhosts/
│   │   │   │   ├── app.longevity.ge.conf
│   │   │   │   ├── biosense.longevity.ge.conf
│   │   │   │   ├── cedar.longevity.ge.conf
│   │   │   │   ├── default.conf
│   │   │   │   ├── fclc.longevity.ge.conf
│   │   │   │   ├── hap.longevity.ge.conf
│   │   │   │   ├── hive.longevity.ge.conf
│   │   │   │   ├── mcoa.longevity.ge.conf
│   │   │   │   └── ze.longevity.ge.conf
│   │   │   ├── systemd/
│   │   │   │   ├── hap-orchestrator.service
│   │   │   │   └── hap-phoenix.service
│   │   │   └── web-content/
│   │   │       ├── cedar-landing/
│   │   │       ├── mcaoa-landing/
│   │   │       └── ngo/
│   │   ├── systemd/
│   │   │   ├── README.md  ← core
│   │   │   ├── app-realtime.env.template
│   │   │   ├── app-realtime.service
│   │   │   ├── app-social.env.template
│   │   │   ├── app-social.service
│   │   │   ├── biosense-web.service
│   │   │   ├── fclc-web.service
│   │   │   ├── longevitycommon-autopull.service
│   │   │   ├── longevitycommon-autopull.timer
│   │   │   └── ze-web.service
│   │   └── web-shared/
│   │       ├── DESIGN_CONCEPT.md
│   │       ├── README.md  ← core
│   │       ├── cedar-landing.html
│   │       ├── eco-inject.js
│   │       ├── mcoa-landing.html
│   │       ├── _originals/
│   │       │   └── README.ru.md
│   │       └── longevity-root/
│   │           ├── index.html
│   │           └── ngo-index.html
│   ├── refs/
│   │   └── refs/
│   │       ├── PMID_15886028_Tkemaladze_2005_cellbiolint_potential_role_of_centrioles_in_dete.md
│   │       ├── PMID_16060722_Ioannidis_2005_plosmed_why_most_published_research_findings.md
│   │       ├── PMID_20068583_Friston_2010_natrevneurosci_the_free_energy_principle_a_unified_.md
│   │       ├── PMID_23746838_López-Otín_2013_cell_the_hallmarks_of_aging.md
│   │       ├── PMID_24138928_Horvath_2013_genomebiol_dna_methylation_age_of_human_tissues.md
│   │       ├── PMID_26213385_Madarampalli_2015_cell_atf5_connects_the_pericentriolar_mat.md
│   │       ├── PMID_29676998_Levine_2018_aging_albanyny_an_epigenetic_biomarker_of_aging_for.md
│   │       ├── PMID_30669119_Lu_2019_aging_albanyny_dna_methylation_grimage_strongly_pre.md
│   │       ├── PMID_30982602_Kucab_2019_cell_a_compendium_of_mutational_signature.md
│   │       ├── PMID_35029144_Belsky_2022_elife_dunedinpace_a_dna_methylation_biomar.md
│   │       ├── PMID_36516495_Lu_2022_aging_albanyny_dna_methylation_grimage_version_2.md
│   │       ├── PMID_36583780_Tkemaladze_2023_molbiolrep_reduction_proliferation_and_differen.md
│   │       ├── PMID_36599349_López-Otín_2023_cell_hallmarks_of_aging_an_expanding_univ.md
│   │       ├── PMID_38243142_Ying_2024_nataging_causality_enriched_epigenetic_age_un.md
│   │       ├── PMID_38510429_Tkemaladze_2023_frontpharmacol_editorial_molecular_mechanism_of_age.md
│   │       ├── PMID_39117878_Argentieri_2024_natmed_proteomic_aging_clock_predicts_morta.md
│   │       └── README.md  ← core
│   ├── reviews_2026-04-21/
│   │   └── 00_CONSOLIDATED_SUMMARY.md
│   └── tbpr/
│       ├── article_2026-05-09.md
│       ├── engineering_2026-05-09.md
│       └── project_2026-05-09.md
├── realtime/
│   ├── Dockerfile
│   ├── mix.exs
│   ├── config/
│   │   ├── config.exs
│   │   ├── dev.exs
│   │   ├── prod.exs
│   │   └── runtime.exs
│   ├── deploy/
│   │   ├── scripts/
│   │   │   └── deploy.sh
│   │   └── systemd/
│   │       └── longevitycommon-realtime.service
│   └── lib/
│       ├── longevitycommon_realtime/
│       │   ├── application.ex
│       │   ├── auth.ex
│       │   ├── feed_notifier.ex
│       │   └── repo.ex
│       └── longevitycommon_web/
│           ├── endpoint.ex
│           ├── router.ex
│           ├── user_socket.ex
│           ├── channels/
│           │   ├── feed_channel.ex
│           │   ├── study_channel.ex
│           │   └── ze_clock_channel.ex
│           └── controllers/
│               └── health_controller.ex
├── scripts/
│   ├── check_v_star.sh
│   ├── llm
│   └── regen_umbrella_core_from_article.sh
├── server/
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── Dockerfile
│   ├── build.rs
│   ├── deploy/
│   │   ├── scripts/
│   │   │   ├── migrate.sh
│   │   │   └── seed.sh
│   │   └── systemd/
│   │       └── longevitycommon-server.service
│   ├── docs/
│   │   └── AUDIT.md
│   ├── migrations/
│   │   ├── 001_initial.sql
│   │   ├── 002_otp_attempts_and_indexes.sql
│   │   ├── 003_health_factors.sql
│   │   └── 004_add_hrv_sdnn_columns.sql
│   ├── src/
│   │   ├── config.rs
│   │   ├── lib.rs
│   │   ├── main.rs
│   │   ├── routes.rs
│   │   ├── db/
│   │   │   └── mod.rs
│   │   ├── handlers/
│   │   │   ├── admin.rs
│   │   │   ├── auth.rs
│   │   │   ├── biosense.rs
│   │   │   ├── dashboard.rs
│   │   │   ├── data.rs
│   │   │   ├── disclosures.rs
│   │   │   ├── mod.rs
│   │   │   ├── posts.rs
│   │   │   ├── studies.rs
│   │   │   ├── users.rs
│   │   │   └── ze_guide.rs
│   │   ├── middleware/
│   │   │   ├── auth.rs
│   │   │   ├── mod.rs
│   │   │   └── rate_limit.rs
│   │   ├── models/
│   │   │   ├── biosense.rs
│   │   │   ├── intervention.rs
│   │   │   ├── mod.rs
│   │   │   ├── post.rs
│   │   │   ├── study.rs
│   │   │   ├── user.rs
│   │   │   ├── ze_guide.rs
│   │   │   └── ze_profile.rs
│   │   └── services/
│   │       ├── ai_guide.rs
│   │       ├── doi_validator.rs
│   │       ├── feed_ranker.rs
│   │       ├── mod.rs
│   │       └── ze_compute.rs
│   └── tests/
│       ├── auth_integration_tests.rs
│       ├── feed_ranker_tests.rs
│       └── ze_compute_tests.rs
├── shared-types/
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── parameter.rs
│       ├── units.rs
│       └── ze.rs
├── sim_core/
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── benches/
│   │   └── aging_simulation.rs
│   ├── examples/
│   │   └── life_simulation.rs
│   ├── src/
│   │   ├── intervention.rs
│   │   ├── lib.rs
│   │   ├── prelude.rs
│   │   ├── provenance.rs
│   │   ├── bin/
│   │   │   └── cli.rs
│   │   ├── centriole/
│   │   │   ├── division.rs
│   │   │   ├── entropy.rs
│   │   │   ├── extension.rs
│   │   │   ├── mod.rs
│   │   │   ├── mod_v2.rs
│   │   │   ├── polyglutamylation.rs
│   │   │   ├── replication.rs
│   │   │   └── types.rs
│   │   ├── counters/
│   │   │   ├── coupling.rs
│   │   │   ├── epigenetic.rs
│   │   │   ├── extension.rs
│   │   │   ├── mitochondrial.rs
│   │   │   ├── mod.rs
│   │   │   ├── proteostatic.rs
│   │   │   └── telomere.rs
│   │   ├── learning/
│   │   │   ├── bayesian.rs
│   │   │   ├── experiment.rs
│   │   │   ├── extension.rs
│   │   │   ├── feedback.rs
│   │   │   ├── hypothesis.rs
│   │   │   ├── mod.rs
│   │   │   └── types.rs
│   │   ├── macrobiome/
│   │   │   └── mod.rs
│   │   ├── microbiome/
│   │   │   ├── extension.rs
│   │   │   ├── gut.rs
│   │   │   ├── mod.rs
│   │   │   ├── oral.rs
│   │   │   ├── skin.rs
│   │   │   └── types.rs
│   │   ├── migration/
│   │   │   ├── cell_dt.rs
│   │   │   ├── mcoa_core.rs
│   │   │   └── mod.rs
│   │   ├── organism/
│   │   │   ├── aging_curve.rs
│   │   │   ├── development.rs
│   │   │   ├── disease.rs
│   │   │   ├── extension.rs
│   │   │   ├── frailty.rs
│   │   │   ├── mod.rs
│   │   │   ├── mortality.rs
│   │   │   ├── trauma.rs
│   │   │   └── types.rs
│   │   ├── spatial/
│   │   │   ├── anatomy.rs
│   │   │   ├── extension.rs
│   │   │   ├── innervation.rs
│   │   │   ├── mod.rs
│   │   │   ├── types.rs
│   │   │   └── vascular.rs
│   │   ├── species/
│   │   │   └── mod.rs
│   │   └── tissue/
│   │       ├── connectivity.rs
│   │       ├── extension.rs
│   │       ├── mod.rs
│   │       ├── renewal.rs
│   │       ├── types.rs
│   │       ├── weights.rs
│   │       └── ze_conflict.rs
│   └── tests/
│       └── integration_mcaoa_cedar.rs
└── web/
    ├── Dockerfile
    ├── index.html
    ├── package-lock.json
    ├── package.json
    ├── tsconfig.json
    ├── tsconfig.node.json
    ├── vite.config.ts
    ├── deploy/
    │   └── scripts/
    │       └── deploy.sh
    ├── dist/
    │   ├── icon.svg
    │   ├── index.html
    │   ├── manifest.webmanifest
    │   ├── registerSW.js
    │   ├── sw.js
    │   ├── workbox-9c191d2f.js
    │   └── assets/
    │       └── index-C_my2dCD.js
    ├── docs/
    │   └── RUST_PHOENIX_MIGRATION_PLAN.md
    ├── public/
    │   └── icon.svg
    ├── scripts/
    │   └── gen-icons.mjs
    └── src/
        ├── App.tsx
        ├── main.tsx
        ├── components/
        │   ├── feed/
        │   │   ├── CreatePost.tsx
        │   │   └── PostCard.tsx
        │   ├── lab/
        │   │   └── ZeGuide.tsx
        │   └── ui/
        │       ├── ZeProfileCard.tsx
        │       ├── ZeShareCard.tsx
        │       └── ZeTrendChart.tsx
        ├── hooks/
        │   ├── useApi.ts
        │   ├── useRealtime.ts
        │   └── useZeProfile.ts
        ├── pages/
        │   ├── Dashboard.tsx
        │   ├── Feed.tsx
        │   ├── Login.tsx
        │   ├── Profile.tsx
        │   ├── Settings.tsx
        │   └── Studies.tsx
        ├── store/
        │   ├── auth.ts
        │   └── index.ts
        └── types/
            └── index.ts
```

## Подпроекты (с _pi.md)

| Подпроект | Путь | Core |
|-----------|------|:----:|
| **MCARA** | `MCARA/` | 7/7 |
| **ARGUS-LP** | `MCARA/ARGUS-LP/` | 7/7 |
| **Aubrey** | `MCARA/Aubrey/` | 7/7 |
| **Aubrey/Phase-0** | `MCARA/Aubrey/Phase-0/` | 7/7 |
| **Aubrey/Phase-A** | `MCARA/Aubrey/Phase-A/` | 7/7 |
| **Aubrey/Phase-B** | `MCARA/Aubrey/Phase-B/` | 7/7 |
| **Aubrey/EIC_Pathfinder_Open** | `MCARA/Aubrey/grants/EIC_Pathfinder_Open/` | 7/7 |
| **CEDAR** | `MCARA/CEDAR/` | 7/7 |
| **EpigeneticDrift** | `MCARA/EpigeneticDrift/` | 7/7 |
| **MitoROS** | `MCARA/MitoROS/` | 7/7 |
| **Proteostasis** | `MCARA/Proteostasis/` | 7/7 |
| **Telomere** | `MCARA/Telomere/` | 7/7 |
| **CEDAR/Aubrey-Platform** | `MCARA/CEDAR/Aubrey-Platform/` | 7/7 |
| **CEDAR/ARGUS-Hardware** | `MCARA/CEDAR/Aubrey-Platform/ARGUS-Hardware/` | 7/7 |
| **CEDAR/simulator** | `MCARA/CEDAR/simulator/` | 7/7 |
| **CEDAR/CellLineageTree** | `MCARA/CEDAR/CellLineageTree/` | 7/7 |
| **CEDAR/articles** | `MCARA/CEDAR/articles/` | 7/7 |
| **Ze_CHSH** | `Ze/Ze_CHSH/` | 7/7 |
| **Ze_D** | `Ze/Ze_D/` | 7/7 |
| **Ze-Hierarchy** | `Ze/Ze-Hierarchy/` | 7/7 |
| **Ze_Model** | `Ze/Ze_Model/` | 7/7 |
| **ze_sim** | `Ze/website/ze_sim/` | 7/7 |
| **Ze/simulator** | `Ze/simulator/` | ⚠️ 1/7 |
| **BioSense** | `BioSense/` | 7/7 |
| **BioSense/automated-microscopy** | `BioSense/instruments/automated-microscopy/` | 7/7 |
| **BioSense/CubanEEG** | `BioSense/data/cuban/oldgandalf-.../` | 7/7 |
| **FCLC** | `FCLC/` | 7/7 |
| **HAP** | `HAP/` | 7/7 |
| **Organismal_Aging** | `Organismal_Aging/` | 7/7 |

## План реорганизации (v7.0 от 2026-06-21)

> ⚠️ Следующая архитектура — **ПЛАН**, не реализована:
> - `sim_core/` — единое ядро (центриоль + counters + tissue + organism + ...)
> - `biosense/`, `fclc/`, `hap/` — переименование (lowercase)
> - `sim_cli/`, `sim_api/`, `sim_gui/`, `sim_py/` — интерфейсы
> - `calibration/`, `validation/`, `argus_bridge/`, `infogest_bridge/` — мосты

---

*Обновлено 2026-07-10 (Цикл 13 аудита — переименования Aubrey→Aubrey-Platform, ARGUS→ARGUS-Hardware, +недостающие подпроекты).*
