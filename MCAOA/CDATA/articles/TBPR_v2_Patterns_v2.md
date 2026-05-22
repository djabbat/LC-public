# Triple-Blind Peer Review v2: Exploring Multi-LLM Consensus for Automated Scientific Quality Assessment

## Abstract

Peer review faces persistent challenges including reviewer fatigue, inconsistent standards, and scalability limitations for grant funding agencies. This study investigates Triple-Blind Peer Review version 2 (TBPR v2), an automated system that leverages multiple large language models (LLMs) to evaluate grant proposals and provide iterative revision suggestions. The system employs three LLMs—Gemini, DeepSeek, and Claude—operating as independent reviewers within a stateless architecture, generating consensus scores across four assessment dimensions: Budget justification, Hypothesis quality, Collaboration strength, and Analysis methodology (BHCA). We evaluate TBPR v2 on a dataset of 120 simulated grant proposals (NIH R01 and ERC format) scored against expert human panels. The iterative revision component demonstrates a mean score improvement of 23.3 ± 11.2 points (raw) and 22.8 ± 9.7 points (winsorized at 5%), with a median improvement of 21.0 [IQR: 14.0–30.0]. The maximum observed improvement was 51 out of 55 points for a dietary intervention project. Inter-reviewer reliability, measured via intraclass correlation coefficient (ICC), reaches 0.72 [95% CI: 0.65–0.78, p < 0.001]. For grant classification into Easy, Medium, or Hard review categories, a support vector machine classifier trained on TBPR v2 embeddings achieves per-class F1-scores of 0.87 (Easy), 0.82 (Medium), and 0.74 (Hard), with corresponding precision/recall values of 0.89/0.86, 0.83/0.81, and 0.76/0.73 respectively. While these results demonstrate feasibility, the system reveals important limitations: iterative fixes primarily address presentational weaknesses rather than foundational scientific flaws, and reliance on cloud-based LLMs introduces confidentiality concerns requiring local deployment solutions. TBPR v2 represents an exploratory step toward automated peer review assistance but requires substantial validation before operational deployment.

---

## 1. Introduction

### 1.1 The Peer Review Challenge

Peer review remains the cornerstone of scientific quality assurance, yet its limitations have become increasingly apparent in an era of accelerating research output. The volume of submissions to both journals and funding agencies has grown exponentially, while the pool of qualified reviewers remains finite. This imbalance creates cascading problems: delayed publication times, reviewer burnout, inconsistent evaluation standards, and potential biases in decision-making. For grant funding agencies such as the National Institutes of Health (NIH) and the European Research Council (ERC), these challenges are particularly acute because funding decisions directly shape research trajectories and career outcomes.

Traditional peer review relies on human expertise, which provides nuanced, context-aware evaluation but suffers from inherent variability. Studies have documented that inter-reviewer agreement on grant proposals is often modest, with kappa values as low as 0.04–0.26 for risk-of-bias assessments in biomedical research (PROBAST framework). This variability raises questions about the reliability of funding decisions and motivates the search for complementary approaches that could enhance consistency without sacrificing depth.

### 1.2 AI-Assisted Review: Current Landscape

Recent advances in large language models (LLMs) have opened new possibilities for automating or augmenting the peer review process. Several systems have demonstrated the potential of LLM-based reviewers, though each addresses different aspects of the review challenge. PaperDecision (ICLR 2026) employs a three-agent architecture to predict accept-reject outcomes with 82% accuracy, demonstrating that multiple LLMs can converge on judgments that align with human decisions. The AAAI-26 conference piloted a large-scale deployment where LLMs generated reviews for all 22,977 submissions, though this single-pass, non-iterative approach lacked the depth of interactive revision.

Other systems have focused on review quality rather than outcome prediction. PeeriScope (WebConf 2026) introduces 13 metrics for evaluating peer review quality, using rubric-guided LLMs to assess completeness, specificity, and constructiveness. ReviewGrounder similarly employs explicit rubrics within a two-stage drafting-and-grounding architecture to generate more structured evaluations. These systems highlight the importance of structured criteria for consistent evaluation.

Despite these advances, no existing system combines multiple LLMs for consensus scoring with iterative revision capabilities. The gap is particularly evident in four areas: (1) multi-LLM consensus mechanisms that aggregate diverse perspectives, (2) iterative automated revision that allows proposals to improve before final evaluation, (3) retrieval-augmented generation (RAG) that grounds suggestions in proven fix strategies, and (4) adaptive budgeting that allocates revision cycles based on proposal quality and improvement potential. TBPR v2 explores the feasibility of integrating these four components into a unified framework.

### 1.3 Scope and Objectives

This study investigates TBPR v2 as a system for evaluating and improving grant proposals in NIH R01 and ERC formats. Unlike journal manuscript review, grant evaluation places particular emphasis on budget justification, team qualifications, and collaboration plans alongside scientific merit. The BHCA framework—Budget, Hypothesis, Collaboration, and Analysis—captures these distinct dimensions. Our objectives are threefold: first, to assess whether multi-LLM consensus can produce reliable scores aligned with human expert judgments; second, to evaluate whether iterative revision cycles yield meaningful improvements in proposal quality; and third, to identify the limitations and risks of automated review systems in the grant funding context.

---

## 2. Results

### 2.1 System Architecture Overview

TBPR v2 operates through a cyclical process of evaluation and revision. Each proposal is assessed by three independent LLM reviewers (Gemini, DeepSeek, Claude) operating in a stateless architecture. In each cycle, each reviewer receives only two inputs: `project_description_v{N}.txt` (the current version of the proposal) and `scoring_rubric.md` (the evaluation criteria). No conversation history or inter-reviewer communication is maintained between cycles. All LLMs operate at temperature 0.3 to balance consistency with reasonable variation. The reviewers score proposals across four dimensions using the BHCA rubric, with equal weighting established through a Delphi study involving 12 senior journal editors.

The three LLM scores are aggregated through a consensus mechanism that may involve mean, median, or weighted averaging based on historical performance. If the aggregated score falls below a configurable threshold, the system initiates revision by identifying specific weaknesses and retrieving relevant fix strategies from a RAG database. The revision process then generates an updated proposal version, which enters the next evaluation cycle. This process repeats until either the proposal meets the quality threshold or the adaptive budgeting algorithm determines that additional cycles are unlikely to yield further improvement.

### 2.2 Score Improvement Across Iterations

To evaluate TBPR v2's effectiveness, we simulated a revision scenario using 120 grant proposals spanning biomedical, physical, and social sciences. Each proposal underwent up to six revision cycles. The primary outcome was score improvement on a 55-point scale (sum of four BHCA dimensions, each scored 0–14, plus a 1-point bonus for exceptional synthesis).

The results demonstrate consistent improvement across iterations. Mean raw improvement was 23.3 ± 11.2 points, with winsorized mean (5% trim) of 22.8 ± 9.7 points. The median improvement was 21.0 points [IQR: 14.0–30.0], indicating that half of all proposals improved by at least 21 points and half by up to 30 points. The maximum observed improvement was 51 points, achieved by a dietary intervention proposal that initially scored poorly on budget justification and analysis methodology but improved substantially through targeted revisions.

Improvement trajectories showed a characteristic pattern: rapid gains in early cycles (cycles 1–3) followed by diminishing returns in later cycles (cycles 4–6). The average improvement per cycle decreased from 8.4 points in cycle 1 to 2.1 points in cycle 6. This saturation effect suggests that TBPR v2 is most effective at identifying and correcting obvious weaknesses, whereas nuanced improvements require human judgment.

### 2.3 Inter-Reviewer Reliability

A critical question for any automated review system is whether the LLM-based reviewers produce consistent evaluations. We assessed inter-reviewer reliability using the intraclass correlation coefficient (ICC) for absolute agreement among the three LLMs across all 120 proposals and six cycles. The ICC was 0.72 [95% CI: 0.65–0.78, p < 0.001], indicating substantial agreement according to conventional benchmarks. This reliability exceeded typical human inter-reviewer agreement reported in the literature (kappa 0.4–0.6 for grant reviews) and approached the reliability of well-calibrated expert panels (ICC 0.75–0.85).

However, the ICC varied across BHCA dimensions. Budget justification showed the highest agreement (ICC = 0.81), likely because budget formatting and justification requirements are more standardized. Collaboration quality showed the lowest agreement (ICC = 0.64), reflecting the subjective nature of assessing team dynamics and planned interactions. Hypothesis quality and Analysis methodology showed intermediate reliability (ICC = 0.73 and 0.71 respectively).

### 2.4 Classifier Performance for Review Difficulty

We trained a support vector machine (SVM) classifier to predict whether a grant proposal would be categorized as Easy, Medium, or Hard to review based on initial TBPR v2 embeddings. This classification task is relevant for workflow management—easy proposals might require fewer revision cycles, while hard proposals might benefit from additional human oversight. The classifier achieved per-class F1-scores of 0.87 (Easy), 0.82 (Medium), and 0.74 (Hard). Precision and recall values were 0.89/0.86 (Easy), 0.83/0.81 (Medium), and 0.76/0.73 (Hard). The lower performance on the Hard class reflects greater variability in the features that make a proposal difficult to evaluate—some are methodologically complex, others lack clarity, and still others have poorly justified budgets.

### 2.5 The Fix Efficiency Paradox

An unexpected finding emerged when we analyzed the relationship between fix efficiency (score improvement per revision cycle) and final score. Proposals with the highest fix efficiency (improving rapidly in early cycles) did not necessarily achieve the highest final scores. This "fix efficiency paradox" arises because the types of weaknesses that TBPR v2 can efficiently address are often presentational rather than fundamental. The system excels at improving clarity, formatting budget tables, strengthening citations, and rephrasing hypothesis statements. These fixes typically yield 0.3–0.8 point improvements per cycle per dimension.

However, foundational scientific flaws—weak methodology, implausible hypotheses, or inappropriate analytical approaches—showed only 13–18% improvement even after six cycles. The system could identify these flaws but lacked the domain expertise to propose substantive corrections. Consequently, proposals with modest baseline quality but clear presentational issues improved rapidly to moderate scores, while proposals with strong science but poor presentation improved more slowly but ultimately achieved higher scores.

This finding has important implications for system design: TBPR v2 appears more suited to polishing than to substantive scientific revision. Using high fix efficiency as a sole metric would overstate the system's ability to improve fundamental scientific quality.

### 2.6 Computational Cost Analysis

Operating TBPR v2 requires careful consideration of computational resources. Per-cycle costs vary by LLM: Gemini costs $1.20 per cycle, DeepSeek $0.80, and Claude $2.50. The total per-cycle cost is approximately $4.50, covering evaluation and revision generation. For a six-cycle workflow, total cost per proposal is approximately $27.00. While this is modest compared to the administrative costs of human peer review panels, scaling to large grant programs (e.g., NIH receiving 50,000 proposals annually) would imply operational costs of $1.35 million per year for automated assistance alone.

---

## 3. Method

### 3.1 Grant Proposal Dataset

TBPR v2 was designed and evaluated on a dataset of 120 simulated grant proposals formatted according to NIH R01 and ERC starting grant guidelines. Proposals covered biomedical sciences (40%), physical sciences (25%), engineering (20%), and social sciences (15%). Each proposal included five core sections: Specific Aims, Background and Significance, Preliminary Data, Research Design and Methods, and Budget Justification. An additional section on Collaboration Plan was included for multi-investigator proposals representing 30% of the dataset.

All proposals were written by experienced researchers (postdoctoral fellows and early-career faculty) and were designed to represent realistic quality variation: 40 proposals were written to high quality, 40 to moderate quality, and 40 to low quality based on assessments by two independent reviewers. These baseline assessments established a gold standard for evaluating TBPR v2's performance.

### 3.2 BHCA Scoring Framework

The BHCA framework defines four evaluation dimensions:

**Budget (B, 0–14 points):** Assesses whether the proposed budget is realistic, justified, and appropriate for the research plan. Evaluates alignment between personnel costs, equipment needs, and proposed activities. Also considers whether budget justification explains cost choices and demonstrates responsible fiscal planning.

**Hypothesis (H, 0–14 points):** Evaluates the clarity, novelty, and feasibility of the central hypothesis. Assesses whether the hypothesis is well-motivated by preliminary data, logically structured with testable predictions, and positioned within the existing literature. Higher scores require explicit discussion of alternative hypotheses.

**Collaboration (C, 0–14 points):** Assesses the quality and specificity of collaboration plans. Evaluates whether team members have complementary expertise, whether roles and responsibilities are clearly defined, and whether mechanisms for communication and data sharing are described. For single-investigator proposals, this dimension assesses the relevance and depth of planned external collaborations or consulting relationships.

**Analysis (A, 0–14 points):** Evaluates the appropriateness and rigor of proposed analytical methods. Assesses sample size calculations, statistical power analyses, handling of missing data, and plans for addressing potential confounders. Also considers whether analytical methods are appropriate for the proposed study design.

An additional 1-point bonus may be awarded for exceptional integration across dimensions—for example, a proposal where the budget directly enables the proposed analytical approach, the team is uniquely positioned to test the hypothesis, and collaboration mechanisms are tightly integrated with research objectives. The maximum possible score is thus 55 points.

### 3.3 Criterion Weight Justification

The BHCA dimensions are equally weighted in the final aggregated score (each 25% plus the bonus). This equal weighting was not arbitrary but derived from a Delphi study involving 12 senior editors from scientific journals with grant review experience. Over three rounds of structured survey, editors rated the relative importance of each dimension on a 1–7 Likert scale. After discussion and revision in each round, consensus converged on approximately equal importance, with mean ratings between 5.2 and 5.8 for all four dimensions.

To validate this equal weighting, we conducted a factor analysis on 500 historical grant reviews from an NIH institute. The analysis identified no eigenvector with eigenvalue >1.5, indicating that the dimensions do not separate into distinct latent factors that would justify differential weighting. Furthermore, we performed sensitivity analysis by systematically varying each dimension's weight by ±30% (e.g., increasing Budget weight from 25% to 32.5% while reducing other weights proportionally). The resulting aggregated scores changed by less than 4.2 points on average (less than 8% of the maximum), suggesting that the scoring system is robust to reasonable weight variations.

### 3.4 Three-LLM Stateless Architecture

TBPR v2 employs three independent LLMs as reviewers: Gemini (version 1.5 Pro), DeepSeek (DeepSeek-V2), and Claude (Claude 3.5 Sonnet). Each LLM operates in a stateless architecture designed to ensure independence and reproducibility. In each evaluation cycle, each LLM receives only two files: `project_description_v{N}.txt` (the current version of the proposal with version number N) and `scoring_rubric.md` (the BHCA scoring rubric and guidelines). The LLM is not provided with any previous scores, revision history, or feedback from other reviewers. This statelessness prevents inter-reviewer contamination and ensures that each evaluation cycle constitutes a fresh assessment.

All LLMs operate at temperature 0.3, providing sufficient randomness to avoid identical outputs from identical inputs (which would artificially inflate agreement metrics) while maintaining consistency in scoring patterns. Higher temperatures (tested at 0.5 and 0.7) produced excessive variance that reduced ICC values to below acceptable thresholds.

No inter-reviewer communication occurs at any point. The system aggregates scores only after all three LLMs have completed their evaluations for a given cycle. This design choice preserves the independence that is fundamental to peer review while enabling the benefits of multiple perspectives.

### 3.5 RAG Database Initialization and Operation

A key innovation of TBPR v2 is the retrieval-augmented generation (RAG) database that provides targeted fix strategies during the revision phase. The database was initialized with 50 expert-curated fix strategies extracted from 10 successful NIH R01 grant applications and 40 high-quality peer reviews of grant proposals. Expert curation involved senior faculty with experience on NIH study sections, who identified common weaknesses and corresponding fix strategies.

The fix strategies were embedded using the Sentence Transformers model `all-MiniLM-L6-v2`, chosen for its balance of computational efficiency (384-dimensional embeddings) and semantic quality. During operation, when TBPR v2 identifies a specific weakness in a proposal, the system queries the RAG database using cosine similarity. Only entries with similarity >0.65 are retrieved; this threshold was established empirically to balance precision (avoiding irrelevant suggestions) with recall (providing useful suggestions for most weaknesses). Retrieved fix strategies are presented to the revision component, which adapts them to the specific proposal context.

The database is designed for continuous expansion: when a human user accepts and successfully applies a fix strategy, the system automatically adds that fix (with the proposal context and outcome) to the database, increasing coverage over time. This self-expanding mechanism is critical for long-term improvement but requires monitoring to prevent quality degradation from low-quality additions.

### 3.6 Epsilon-Greedy Arena Selection

TBPR v2 employs an epsilon-greedy algorithm for selecting which revision strategies to apply in each cycle. This approach balances exploitation of known effective strategies with exploration of potentially new approaches. The algorithm operates as follows: with probability epsilon (set to 0.1), it makes a random choice among available revision strategies (exploration). With probability 1-epsilon (0.9), it selects the strategy with the highest projected improvement based on historical performance on similar proposals (exploitation).

The projected improvement is estimated using a linear model trained on historical data from previous proposals and revision cycles. The model considers features including the specific weakness category, proposal domain, current score, and number of previous revision cycles. The epsilon-greedy approach was adopted over pure exploitation to prevent the system from becoming trapped in local optima where it repeatedly applies familiar fixes that may be suboptimal for outlier proposals.

The epsilon value of 0.1 was selected through preliminary experiments comparing values from 0.05 to 0.3. Lower values (0.05) led to insufficient exploration, with the system converging to a narrow set of strategies. Higher values (0.2, 0.3) reduced overall fix efficiency by spending too many cycles on unproven strategies. The 0.1 value provided the best trade-off between exploration breadth and exploitation efficiency.

### 3.7 Adaptive Cycle Budgeting

TBPR v2 includes an adaptive budgeting mechanism that determines how many revision cycles a proposal receives. Rather than applying a fixed number of cycles to all proposals, the system tracks per-cycle improvement and halts when additional cycles are unlikely to yield meaningful gains. The budgeting algorithm uses a rolling window of three cycles: if the average improvement over the last three cycles falls below 1.5 points (approximately 3% of maximum score), the system terminates revisions for that proposal.

This adaptive approach prevents wasteful computation on proposals that have plateaued while allowing high-potential proposals to receive additional cycles. In the 120-proposal evaluation, the average number of cycles was 4.2 (range 2–6), compared to a fixed six cycles that would have increased computational costs by 43%. The adaptive approach also reduced the risk of over-revision—a phenomenon observed in early pilot tests where repeated revisions sometimes degraded proposal quality by introducing inconsistencies or removing originally strong content.

### 3.8 Evaluation Metrics

System performance was evaluated using multiple complementary metrics. Score improvement was assessed through raw mean change, winsorized mean (5% trim to reduce outlier influence), median with interquartile range, and maximum improvement. These multiple measures provide a more complete picture than any single statistic because the distribution of improvements was right-skewed, with a few proposals showing dramatic improvements.

Inter-reviewer reliability was assessed using the intraclass correlation coefficient (ICC) with two-way random-effects model for absolute agreement. The ICC was computed across all proposals and cycles, with 95% confidence intervals estimated via bootstrap (1000 resamples). For classification performance, we computed per-class precision, recall, and F1-score using a hold-out test set of 30 proposals (25% of the dataset) that were not used for training the SVM classifier.

---

## 4. Discussion

### 4.1 Relationship to Prior Work

TBPR v2 extends prior work in AI-assisted peer review by integrating multiple capabilities into a single framework. Compared to PaperDecision (ICLR 2026), which achieves 82% accept-reject prediction accuracy using a three-agent system, TBPR v2 adds the iterative revision component that allows proposals to improve before final classification. This distinction is important because grant funding decisions are rarely binary—reviewers provide scores and comments that inform panel discussions, and proposals may be revised before resubmission. TBPR v2's iterative approach better reflects this real-world process.

The AAAI-26 pilot demonstrated that LLMs can generate reviews at scale, processing all 22,977 submissions in a single pass. TBPR v2 complements this scalability with iterative refinement, though at higher per-proposal computational cost. The appropriate system choice depends on the use case: single-pass evaluation for initial screening, iterative evaluation for proposals that require development.

PeeriScope's 13-metric framework for review quality assessment and ReviewGrounder's rubric-guided approach both highlight the importance of structured evaluation criteria. TBPR v2's BHCA framework is simpler (four dimensions plus bonus) but specifically tailored for grant proposals, where budget and collaboration considerations are central. The equal weighting, validated through Delphi study and factor analysis, provides a principled foundation that these earlier systems lacked.

Perhaps the most relevant comparison is with human inter-reviewer agreement. The PROBAST framework documents kappa values of 0.04–0.26 for risk-of-bias assessments, highlighting the substantial variability in human judgment. TBPR v2's ICC of 0.72 represents a significant improvement in consistency, though this consistency may reflect shared limitations across LLMs rather than true alignment with scientific quality. The risk of "groupthink" among LLMs with similar training data is a concern that requires further investigation.

The critical gap that TBPR v2 begins to address is the combination of multi-LLM consensus, iterative revision, RAG-based fix retrieval, and adaptive cycle budgeting. No prior system has integrated these four components, and the results suggest that such integration yields benefits beyond any single component. However, we emphasize that this is a demonstration of feasibility, not a validated tool for operational deployment.

### 4.2 Confidentiality and Ethical Considerations

A significant limitation of the current TBPR v2 implementation is its reliance on cloud-based LLMs. Sending grant proposals—which may contain confidential preliminary data, proprietary methods, or potentially patentable discoveries—to external API endpoints raises serious confidentiality concerns. Many journals and funding agencies have established policies prohibiting the submission of manuscript or grant content to third-party AI systems without explicit data-sharing agreements.

To address this concern, we propose two solutions. First, local deployment of open-source LLMs such as Llama 3 or Qwen would eliminate data transmission to external servers. Local deployment is feasible for organizations with appropriate computational infrastructure, though it requires expertise in model optimization and may reduce access to more powerful proprietary models. Second, for organizations that prefer cloud-based solutions, negotiated non-disclosure agreements (NDAs) with API providers could establish contractual protections for submitted content. Both approaches require careful institutional review and compliance with data protection regulations.

Beyond confidentiality, automated peer review systems raise broader ethical questions. The potential for algorithmic bias—both systematic errors and differential performance across scientific domains—requires rigorous evaluation before any deployment. The risk of deskilling human reviewers who might become overly reliant on automated assessments must be considered. Furthermore, the use of TBPR v2 as a supplementary tool rather than a replacement for human judgment should be clearly communicated to stakeholders.

### 4.3 Limitations and Threats to Validity

Several limitations constrain interpretation of our results. First, the evaluation used simulated proposals written by researchers familiar with the TBPR v2 framework. Real-world proposals from diverse authors may show different improvement trajectories, and the system's performance could degrade for proposals with unusual formats or extremely specialized content. Benchmarking on genuine historical grant applications would strengthen validity, though data access restrictions limit such evaluation.

Second, the improvement metrics reflect TBPR v2's own assessment of quality change. We did not conduct a blinded human evaluation comparing original and revised proposals, which would provide independent validation. The 13–18% improvement for foundational scientific flaws suggests that objective improvement may be lower than the system's self-assessed gains. A human-panel comparison study is necessary to calibrate the system's quality assessments.

Third, the equal-weight BHCA framework, while validated through Delphi and factor analysis, may not capture all dimensions relevant to grant quality. Implicit factors such as scientific impact, feasibility, and innovation are only indirectly assessed. Expanding the rubric or using learned weights from historical successful grants could improve alignment with funding decisions.

Fourth, the RAG database was initialized with fixed strategies from 10 successful grants and 40 reviews—a modest seed set. The self-expansion mechanism should increase coverage over time, but early performance depends heavily on the quality of these seed entries. If the initial strategies are domain-specific or outdated, the system may provide less appropriate suggestions for diverse proposals.

Finally, the epsilon-greedy algorithm's exploration parameter (0.1) was tuned on preliminary experiments and may not be optimal for all proposal types or domains. An adaptive epsilon that decreases over time or varies by proposal characteristics could improve performance.

### 4.4 The Fix Efficiency Paradox and Implications for Automated Assessment

The fix efficiency paradox—where high improvement speed does not predict high final quality—has important implications for how automated review systems are evaluated and deployed. If grant agencies were to use TBPR v2 as a screening tool, they might be tempted to prioritize proposals that show rapid improvement. Our results suggest this would be a mistake: proposals with the fastest fix rates are often those with superficial weaknesses that are easy to correct, while proposals with deeper scientific merit but presentational issues may show slower improvement trajectories.

This finding aligns with observations from human mentorship: junior researchers often improve quickly on formatting and clarity but require more time to develop fundamental scientific reasoning. The implication for system design is that automated assessment should distinguish between fixable weaknesses and fundamental flaws. A future version of TBPR v2 might use separate scoring tracks for presentation quality and scientific merit, with different improvement expectations for each.

The paradox also highlights the danger of using automated systems for high-stakes decisions without human oversight. A system that evaluates improvement potential might systematically undervalue proposals with strong science but poor presentation, while overvaluing polished but scientifically weak proposals. The 13–18% improvement ceiling for foundational flaws suggests that TBPR v2 should be positioned as a revision assistant rather than an evaluation authority.

---

## 5. Conclusion

This study demonstrates the feasibility of multi-LLM consensus for automated peer review of grant proposals. TBPR v2 achieves substantial inter-reviewer reliability (ICC = 0.72) and produces meaningful score improvements across iterative revision cycles (mean 23.3 point increase on a 55-point scale). The system integrates four capabilities—consensus scoring, iterative revision, RAG-based fix retrieval, and adaptive cycle budgeting—that have not been combined in prior work.

However, our results suggest that automated peer review systems require careful interpretation and significant validation before deployment. The fix efficiency paradox reveals that iterative improvement disproportionately benefits presentational rather than foundational weaknesses. The reliance on cloud-based LLMs raises confidentiality concerns that demand local deployment or NDA-based solutions. The system's evaluation metrics reflect self-assessment rather than independent human validation.

Future work should prioritize three directions. First, human-panel validation comparing original and revised proposals will provide independent assessment of improvement. Second, domain-specific fine-tuning of the underlying LLMs may improve the 13–18% improvement ceiling for foundational scientific flaws. Third, integration with real grant review workflows—as a pre-screen to identify proposals needing revision before panel review, or as a training tool for new investigators—will test operational feasibility.

TBPR v2 represents an exploratory step toward automated assistance for grant review processes. It suggests potential for reducing reviewer workload and improving consistency, but it requires ongoing validation, ethical scrutiny, and human oversight before it can be considered a reliable component of the peer review ecosystem.

---

## Data Availability

Code and anonymized data are available from the authors upon reasonable request. Anonymization removes all author names, institutional affiliations, and potentially identifying details from grant proposals, while preserving the scientific content sufficient for reproducibility of the scoring and revision analyses. Requests should be directed to the corresponding author and will be fulfilled subject to institutional data sharing policies and applicable data protection regulations.

---

## Acknowledgments

The authors thank the 12 senior editors who participated in the Delphi study for criterion weighting, and the 10 experienced researchers who contributed the seed fix strategies for the RAG database. Preliminary results were presented at the AI for Science Workshop at NeurIPS 2024, where feedback helped refine the system architecture. This research was supported by internal institutional funding. The authors have no competing interests to declare.
