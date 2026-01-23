# Methodological Foundations for Time-Boxed AI Prototype Development as Valid HCI Research

**Time-boxed prototype development represents a legitimate and well-established research methodology in HCI, supported by IEEE/ISO standards, decades of Research Through Design scholarship, and explicit endorsement from premier venues like CHI.** This report synthesizes frameworks across feasibility studies, rapid prototyping, AI evaluation, voice/multimodal systems, and HCI publication conventions to establish rigorous methodological foundations for 48-hour prototype development as valid academic research. The convergence of Research Through Design traditions, hackathon outcome studies, and formal feasibility frameworks provides robust theoretical grounding, while CHI Late-Breaking Work guidelines explicitly welcome "prototypes with or without an accompanying evaluation."

---

## Standards-based foundations for feasibility studies

### IEEE and ISO frameworks define feasibility as lifecycle-appropriate assessment

Feasibility studies occupy a well-defined position within international software engineering standards. **ISO/IEC/IEEE 15288:2023** establishes the Concept phase—where feasibility assessment occurs—as a legitimate lifecycle stage preceding Development, with activities that may be "executed multiple times across multiple phases." The standard does not prescribe specific methodologies, explicitly allowing processes to be "recursed, revisited, overlapped, or iterated."

**IEEE 1012-2016** (Software Verification and Validation) directly validates prototyping as a V&V method, stating that "releasing prototypes and having users and stakeholders assess them" constitutes valid dynamic testing. This positions prototype-based feasibility assessment within formal verification frameworks. **ISO/IEC/IEEE 29148:2018** (Requirements Engineering) includes "feasible" as a required attribute for stakeholder requirements, defining feasibility as achievability within known constraints.

The **TELOS framework** provides a systematic taxonomy for feasibility assessment:
- **Technical feasibility**: Can it be built with available technology?
- **Economic feasibility**: Are costs justified by benefits?
- **Legal feasibility**: Does it comply with regulations?
- **Operational feasibility**: Will it work within organizational context?
- **Schedule feasibility**: Can it be completed within time constraints?

For time-boxed prototyping, technical and operational feasibility are primary concerns—schedule feasibility is deliberately constrained as a methodological parameter rather than assessed as an outcome.

### Formal distinctions between feasibility demonstrations

Industry and academic literature establish clear conceptual boundaries:

| Type | Purpose | Scope | Outcome |
|------|---------|-------|---------|
| **Feasibility Study** | Systematic viability analysis | Broad assessment | Decision recommendation |
| **Proof of Concept** | Technical validation | Single capability/integration | Binary feasibility determination |
| **Prototype** | Design validation, UX exploration | User-facing implementation | Design insights, interaction patterns |
| **MVP** | Market validation | Minimum functional product | User/customer feedback |

A 48-hour time-boxed development effort maps to the **Proof of Concept → Prototype** boundary—demonstrating technical feasibility while exploring design possibilities. This aligns with Koskinen et al.'s characterization of research prototypes as "done with relatively cheap materials, shortcut technology, and remain short-lived."

---

## Research Through Design legitimizes construction as inquiry

### Foundational RtD frameworks establish prototyping as knowledge production

**Zimmerman, Forlizzi, and Evenson (2007)** established Research Through Design at CHI as a method where "designers produce novel integrations of HCI research in an attempt to make the right thing: a product that transforms the world from its current state to a preferred state." RtD addresses "wicked problems" that are "by definition not approachable using scientific or engineering modes of inquiry."

**Koskinen et al. (2011)** in *Design Research Through Practice* formalize Constructive Design Research, arguing that "building, deploying and exhibiting objects such as prototypes facilitate inquiry and are as important as writing articles and books." They emphasize that research prototypes need only be "good enough for serving as a vehicle in knowledge production"—not production-ready.

Four evaluation lenses for RtD contributions provide assessment criteria:
1. **Process**: Was the design process documented and rigorous?
2. **Invention**: Does the prototype represent genuine novelty?
3. **Relevance**: Does it address real needs or problems?
4. **Extensibility**: Can findings inform future research?

### Experience prototyping and the "what prototypes prototype" framework

**Houde and Hill (1997)** introduced a three-dimensional framework—Role, Look and Feel, and Implementation—arguing that prototypes should be evaluated based on "what they prototype" rather than fidelity. They recommend finding "the manifestation that, in its most economic form, will filter the qualities in which the designer is interested."

**Buchenau and Suri (2000)** define Experience Prototyping as "any kind of representation, in any medium, that is designed to understand, explore or communicate what it might be like to engage with the product, space or system we are designing." They explicitly advocate that "low-tech solutions promote the attitude that it is the design question that is important, not the tools and techniques."

---

## Time constraints as methodological parameter, not limitation

### Hackathon research validates 24-72 hour development cycles

Extensive academic literature validates constrained-timeline development as productive research methodology. **Medina Angarita and Nolte (2020)** conducted a systematic literature review of hackathon outcomes, finding that hackathons—defined as "time-bounded events where participants gather in teams to develop projects that interest them"—produce valid outcomes including prototypes, learning, and innovation.

**Pe-Than et al. (2022)** studied corporate hackathons at Microsoft, examining "how and why" hackathon outcomes inform organizational knowledge. **Porras et al. (2018)** validated hackathons as effective pedagogical methodology based on a decade of implementation in software engineering education.

### Constraints enhance rather than inhibit creative output

Research on constraints and creativity directly supports time-boxing as a methodological choice. **Rosso (2014)** in *Organization Studies* found that "teams able to accept constraints and which saw opportunity in constraints, benefited creatively from them." **Acar, Tarakci, and van Knippenberg (2018)** conducted a meta-review of **145 studies** on constraints and innovation, finding an inverted U-shaped effect where moderate constraints enhance creativity: "Too few make for a complacent team; a moderate number makes the task a challenge, encouraging experimentation and risk."

**Haught-Tromp (2017)** demonstrated in *Psychology of Aesthetics, Creativity, and the Arts* that constraints can serve as "inspiration triggers" that facilitate creative processes. The Google Ventures Design Sprint methodology, validated across organizations including Slack, Airbnb, and LEGO, demonstrates that five-day (and shorter) prototyping cycles can yield testable, meaningful results.

### Throwaway prototyping tradition supports learning-focused development

The throwaway prototyping paradigm—"quick models created to test ideas, then discarded"—supports research-oriented development where the goal is learning rather than production. **Jalote et al. (2004)** established timeboxing as a formal process model, noting that "many successful software development projects use timeboxing" and that adoption "more than tripled developer productivity at DuPont."

---

## AI system evaluation frameworks for early-stage prototypes

### NIST AI RMF provides proportional assessment guidance

The **NIST AI Risk Management Framework 1.0 (AI 100-1)** establishes four core functions—GOVERN, MAP, MEASURE, MANAGE—applicable throughout the AI lifecycle. For feasibility demonstrations, the framework emphasizes:

- **Context-dependent measurement**: How components are evaluated changes based on operational context
- **Risk-based proportionality**: TEVV (Test, Evaluation, Verification, Validation) processes should be tailored to intended use, potential impact, and associated risks
- **Practical feasibility**: Account for technical limits and budget constraints
- **Acknowledged uncertainty**: For complex AI systems, "certain characteristics are challenging or impracticable to ascertain with complete certainty"

NIST's trustworthiness characteristics—validity, reliability, safety, security, accountability, transparency, explainability, privacy, and fairness—provide a vocabulary for scoping what feasibility demonstrations address versus defer.

### Separating API integration feasibility from model capability feasibility

A critical distinction for AI prototypes:

**API Integration Feasibility** asks:
- Can the system connect to required services?
- Do data flows function correctly?
- Is technical compatibility achievable?

**Model Capability Feasibility** asks:
- Can the AI model achieve required performance?
- Does output quality meet minimum thresholds?
- Are capabilities appropriate for intended use?

Time-boxed prototypes can legitimately claim API integration feasibility while acknowledging that model capability assessment requires larger-scale evaluation.

### Non-deterministic output evaluation strategies

AI systems with variable outputs require adapted evaluation approaches:

1. **Acceptance bands**: Define acceptable quality ranges rather than exact expected outputs
2. **LLM-as-Judge**: Use models to score outputs on defined criteria (coherence, factual accuracy)
3. **Semantic similarity**: Check if outputs are semantically equivalent rather than exact matches
4. **Property-based testing**: Define properties that should hold for all inputs rather than specific expected results
5. **Multiple-run analysis**: Statistical analysis across repeated evaluations with confidence intervals

**Best practices** include versioning all components, using fixed random seeds where possible, and implementing threshold-based acceptance criteria.

### Model Cards and documentation standards

**Mitchell et al. (2019)** established Model Cards as a documentation framework including: model details, intended use, factors affecting performance, metrics, evaluation data, ethical considerations, and caveats. For prototypes, abbreviated documentation should clearly state prototype status, known limitations, evaluation methodology, and gaps in testing coverage.

---

## Voice interface evaluation appropriate for prototype assessment

### SASSI provides validated subjective assessment

**Hone and Graham (2000)** developed the **Subjective Assessment of Speech System Interfaces (SASSI)** questionnaire, a 34-item validated instrument measuring six factors:

1. **System Response Accuracy** (9 items)
2. **Likeability** (9 items)
3. **Cognitive Demand** (5 items)
4. **Annoyance** (5 items)
5. **Habitability** (4 items)
6. **Speed** (2 items)

SASSI captures subjective user experience independent of objective system performance—particularly valuable for prototypes where objective metrics may be unstable.

### Speech recognition metrics and their appropriate scope

**Word Error Rate (WER)** = (Substitutions + Insertions + Deletions) / Reference Words

WER is appropriate for benchmarking ASR capabilities but has limitations: treats all errors equally, is insensitive to semantic impact, and doesn't capture task success. Alternatives include:
- **Semantic Distance (SemDist)**: When meaning preservation matters more than exact wording
- **Task Success Rate**: Percentage of interactions achieving user goals
- **Slot Fill Rate**: Percentage of required information correctly captured

For prototype evaluation, **task completion metrics** provide more meaningful assessment than raw recognition accuracy.

### Latency evaluation methodology

Rather than prescribing thresholds, evaluation should establish:
1. Task criticality (real-time conversation vs. query response)
2. Baseline human-human timing for comparable tasks
3. User satisfaction correlated with observed latencies
4. Identification of thresholds where satisfaction degrades

Component latencies to consider: ASR/STT time, turn-taking detection, LLM time-to-first-token, and TTS synthesis latency.

---

## Multimodal system assessment frameworks

### CARE properties for multimodal design evaluation

**Coutaz and Nigay (1995)** established the CARE framework:
- **Complementarity**: Do modalities complement each other?
- **Assignment**: Is each modality assigned appropriately?
- **Redundancy**: Is information available across modalities?
- **Equivalence**: Can users achieve goals through different modalities?

### Component versus system evaluation strategies

| Level | Metrics | Prototype Scope | Production Scope |
|-------|---------|-----------------|------------------|
| **Component** | WER, vision accuracy, intent classification | Core capability validation | Performance optimization |
| **System** | End-to-end task success, user satisfaction | Binary task completion | Statistically-powered measurement |

For feasibility demonstrations, component-level evaluation establishes technical viability while system-level task completion provides holistic validation.

### PARADISE framework for dialogue assessment

**Walker et al. (1997)** established PARADISE (PARAdigm for DIalogue System Evaluation), combining:
- **Task Success**: Measured via attribute-value matrix comparison
- **Dialogue Costs**: Turns, time, repair rate, help requests
- **User Satisfaction**: Survey-based subjective assessment

The framework supports proportional evaluation—prototypes can report task success and satisfaction without comprehensive cost optimization.

---

## CHI LBW and DIS Pictorials explicitly welcome prototype research

### CHI Late-Breaking Work guidelines endorse un-evaluated prototypes

**CHI 2025 LBW guidelines** explicitly welcome "an original and innovative technology, technique, or **prototype with or without an accompanying evaluation**." The review criteria focus on:
- **Contribution**: Does it stimulate interesting conversation?
- **Significance**: How important is the problem?
- **Originality**: Does it make a novel contribution?
- **Validity**: How well are methods **described and justified**?

**CHI 2026** merged LBW into a unified Posters track emphasizing "the **nascent nature of work** within this track" and explicitly welcoming "paper that presents a **prototype that deserves visibility and feedback**."

### Wobbrock and Kientz's artifact contribution framework

**Wobbrock and Kientz (2016)** defined **Artifact Contributions** as prototypes that "reveal new possibilities, enable new explorations, facilitate new insights, or compel us to consider new possible futures." Critically, they state: "Artifact research contributions are often accompanied by empirical studies **but do not have to be, and sometimes should not be**."

Systems and tools are "evaluated holistically according to what they make possible and how they do so"—not solely through user study metrics.

### Olsen's systems evaluation criteria reject premature empirical demands

**Olsen (2007)** in "Evaluating User Interface Systems Research" argues that traditional usability testing assumptions often don't apply to systems research:
- New tools require expertise users don't yet have
- Complex systems lack comparable standardized tasks
- Realistic tests require prohibitive resources

He proposes alternative criteria: STU context clarity, problem importance, generality, solution viscosity reduction, and enabling new design participants. His "Fatal Flaw Fallacy" acknowledges that "the existence of a fatal flaw is a given" for research systems—evaluation should ask "Has important progress been made?" not "Is this production-ready?"

### Greenberg and Buxton on appropriate evaluation timing

**Greenberg and Buxton (2008)** argue in "Usability Evaluation Considered Harmful (Some of the Time)" that "evaluation can be ineffective and even harmful if naively done 'by rule' rather than 'by thought.'" Premature evaluation demands can "mute creative ideas that do not conform to current interface norms." The choice of evaluation methodology "must arise and be appropriate for the actual problem, research question or product under consideration."

---

## Success criteria definition for time-boxed feasibility

### What "feasibility demonstrated" means

For a 48-hour prototype, feasibility claims should be scoped to:

1. **Technical integration feasibility**: The system can connect required services and process data flows
2. **Core functionality feasibility**: The primary use case can be executed end-to-end
3. **Interaction design feasibility**: The proposed interaction model is implementable
4. **Identified constraints**: Known limitations and failure modes are documented

Feasibility does **not** require: optimized performance, comprehensive error handling, scalability validation, or statistically-validated user satisfaction.

### Pre-defined success criteria template

Before development, define:
- **Functional criteria**: "The system successfully [action] for [percentage] of attempts on [defined test cases]"
- **Integration criteria**: "The system connects to [services] and returns valid responses within [timeframe]"
- **Interaction criteria**: "A user can complete [primary task] without critical errors"
- **Documentation criteria**: "Limitations in [areas] are documented with proposed mitigations"

### Minimum evaluation for feasibility claims

| Evaluation Type | Minimum for Feasibility | Enhanced (Optional) |
|----------------|------------------------|---------------------|
| **Functional testing** | Core scenarios demonstrated | Edge case coverage |
| **User evaluation** | Expert walkthrough (1-3 experts) | User study (5-15 participants) |
| **Performance** | Acceptable latency observed | Statistical measurement |
| **Reliability** | Successful demonstration runs | Systematic failure analysis |

---

## Honest limitations framing that maintains research validity

### Limitations strengthen rather than undermine credibility

Appropriate limitation acknowledgment demonstrates methodological rigor. Frame limitations as:

**Scope delimitations** (what the work does not attempt):
- "This feasibility study focuses on technical integration; model capability optimization is deferred to future work"
- "As a 48-hour prototype, we prioritized [core features] over [secondary features]"

**Identified constraints** (discovered during development):
- "The prototype exhibits [limitation] under [conditions], suggesting [design implication]"
- "Response latency of [observed value] indicates [technical consideration]"

**Future work opportunities** (limitations as research directions):
- "The [limitation] presents opportunity for future research on [direction]"
- "Production deployment would require addressing [specific gaps]"

### Single-developer, time-boxed development conventions

Standard acknowledgments for constrained development:
- "This research prototype was developed by a single researcher over 48 hours to demonstrate technical feasibility"
- "The time-boxed methodology follows Research Through Design traditions where construction serves as the research method"
- "Evaluation scope is appropriate to the prototype stage per Greenberg and Buxton's guidance on premature evaluation"

---

## Framework synthesis for CHI LBW submission

### Recommended methodological positioning

1. **Frame as Research Through Design**: Position the work within RtD methodology (Zimmerman et al., 2007; Koskinen et al., 2011) where construction is inquiry
2. **Invoke feasibility study standards**: Reference ISO/IEC 15288 Concept phase and TELOS framework for technical/operational feasibility
3. **Cite constraint-creativity research**: Support time-boxing with Rosso (2014) and Acar et al. (2018) meta-review
4. **Apply proportional AI evaluation**: Use NIST AI RMF proportionality principles for stage-appropriate assessment
5. **Adopt established voice/multimodal metrics**: Apply SASSI factors, task completion metrics, and PARADISE elements as appropriate
6. **Invoke Wobbrock's artifact contribution**: Position as artifact contribution that "reveals new possibilities"
7. **Reference Olsen and Greenberg on evaluation scope**: Justify evaluation decisions against premature-evaluation critiques

### Key citations for methodology section

| Domain | Essential Citations |
|--------|-------------------|
| **RtD Methodology** | Zimmerman et al. (2007), Koskinen et al. (2011), Frayling (1993) |
| **Prototyping Theory** | Houde & Hill (1997), Buchenau & Suri (2000) |
| **Time Constraints** | Rosso (2014), Acar et al. (2018), Jalote et al. (2004) |
| **AI Evaluation** | NIST AI 100-1 (2023), Mitchell et al. (2019), Amershi et al. (2019) |
| **Voice Evaluation** | Hone & Graham (2000), Walker et al. (1997), ITU-T P.800 |
| **HCI Contribution Types** | Wobbrock & Kientz (2016), Olsen (2007), Greenberg & Buxton (2008) |
| **Feasibility Standards** | ISO/IEC/IEEE 15288:2023, IEEE 1012-2016, ISO/IEC/IEEE 29148:2018 |

### Validity argument structure

A CHI LBW submission on 48-hour time-boxed feasibility can claim validity through:

1. **Methodological precedent**: RtD and constructive design research establish construction as legitimate inquiry
2. **Standards alignment**: ISO/IEC 15288 Concept phase validates feasibility assessment as lifecycle-appropriate
3. **Venue fit**: CHI LBW explicitly welcomes prototypes "with or without accompanying evaluation"
4. **Contribution framing**: Wobbrock's artifact contributions "do not have to be" accompanied by empirical studies
5. **Evaluation appropriateness**: Greenberg and Buxton support deferring comprehensive evaluation for early-stage creative work
6. **Constraint-creativity support**: Meta-analytic evidence demonstrates moderate constraints enhance rather than inhibit creative outcomes
7. **Proportional assessment**: NIST AI RMF supports risk-based, context-appropriate evaluation depth

---

## Conclusion: Time-boxed prototyping as rigorous methodology

This synthesis establishes that 48-hour time-boxed prototype development represents methodologically sound HCI research, not a compromise. The convergence of Research Through Design traditions, ISO feasibility standards, creativity research on constraints, and explicit CHI venue guidelines provides robust theoretical and practical grounding. **Feasibility demonstrations occupy a legitimate position within software engineering lifecycles, and rapid prototyping aligns with decades of constructive design research tradition.**

The key to rigorous presentation lies in appropriate scoping: clearly defining what feasibility means for the specific prototype, pre-specifying success criteria, applying proportional evaluation frameworks, and honestly framing limitations as scope delimitations rather than failures. When positioned correctly—as Research Through Design that demonstrates technical feasibility and explores design possibilities—time-boxed prototypes make valuable contributions to HCI knowledge that CHI Late-Breaking Work is explicitly designed to showcase.
