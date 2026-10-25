# NIBRAS (نبراس) AGENT SESSION INSTRUCTIONS & OPERATOR GUIDELINES
## Best Personal AI Agent | Dubai Create AI Agents Championship 2026–2027
### Target Workspace: `/Users/mohamedfathy/Desktop/nibras-agent`
### Remote Origin: `https://github.com/M4F-S/nibras-agent.git`
### Google Drive Mirror: `My Drive/Dubai_AI_Championship_2027/nibras-agent/`

---

### 1. OPERATOR IDENTITY & COMMUNICATION RULES
1. **Who the Operator is:** Mohamed Fathy, BPharm (Lead Systems Architect & Founder).
2. **Clear & Simple Language:** Always discuss problems, trade-offs, and architecture in **clear, simple, everyday English**. Never overwhelm the operator with dense academic jargon or robotic multiple-choice questions.
3. **Research-First Protocol:** **NEVER guess, assume, or invent cognitive models or NLP architectures.** Always ask Mohamed for domain research or literature before making major design choices.
4. **Engineering First (Strictly No Hype):** Focus 100% on **building, optimizing, and testing the software code**. Do NOT talk about pitch decks, demo videos, or peripheral marketing until the agent code is 100% complete and passing tests.
5. **Tri-Mirror Synchronization:** The local folder, Google Drive mirror, and GitHub repo **must remain 100% identical mirrors at all times**. Whenever code or documentation is created or modified, run `./sync_mirror.sh` to update both Google Drive and GitHub immediately.

---

### 2. UNDERLYING ENGINE FOUNDATION: ALMAZ
- **Primary Foundation Engine:** **`Almaz` (`M4F-S/Almaz`)**
  - Nibras is built directly on **Almaz's sovereign cognitive organism architecture**.
  - *Why:* Personal executive assistance demands long-horizon conversational memory graphs with `FadeMem` exponential half-life decay, cognitive friction tracking, and zero cloud profiling under UAE PDPL Law No. 45 of 2021.

---

### 3. ARCHITECTURAL SCOPE: PERSONAL COGNITIVE ASSISTANT
Nibras is an air-gapped personal cognitive memory assistant and executive exocortex:
1. **Zero-Allocation UTF-8 Arabic Normalizer:** Native C99 text normalizer executing in-place UTF-8 transformation (stripping tashkeel/tatweel, unifying alef/hamza forms) with zero heap allocation.
2. **FadeMem Biological Memory Decay:** Prunes ephemeral conversational chatter ($I(t') = I_0 \cdot 2^{-(t'-t)/\tau_{1/2}}$) while permanently consolidating executive decisions, calendar milestones, and personal preferences.
3. **100% Sovereign On-Device Privacy:** Zero telemetry, air-gapped local execution, zero cloud exfiltration.

---

### 4. IMMEDIATE ENGINEERING ROADMAP
When continuing development in this session, focus on these concrete coding tasks:
- [x] **Task 1: Standalone CLI Daemon (`src/nibras_agent.c`):** Create an interactive C99 daemon that ingests user notes/calendar tasks, normalizes Arabic text, and manages the local memory store.
- [x] **Task 2: Persistent FadeMem Memory Graph:** Implement a lightweight local memory store (JSONL or SQLite) where episodic memories decay over time while permanent nodes stay pinned.
- [x] **Task 3: Arabic Entity & Action Extractor:** Expand `adapters/nibras_arabic_fademem_adapter.c` with deterministic extraction for dates, appointments, and action items from Arabic text.
- [x] **Task 4: Build & Test Verification:** Run `make clean && make test` and ensure all suites pass with zero warnings.

---

### 5. HOW TO VERIFY & TEST
```bash
# Compile and run the complete test suite
make clean && make test

# Synchronize with Google Drive and GitHub
./sync_mirror.sh
```
