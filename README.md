# Nibras (نبراس) — Sovereign Cognitive Memory & Personal AI Assistant
## Best Personal AI Agent | Dubai Create AI Agents Championship 2026–2027

---

### 1. Executive Summary
In personal cognitive management and executive daily operations across Dubai, individuals and family offices face two acute challenges:
1. **Context Window Bloat & Cognitive Drift:** Long-horizon interactions accumulate temporary operational clutter, causing hallucination and runaway token costs.
2. **Data Sovereignty & Profiling:** Cloud-based assistants profile user habits on foreign servers, violating UAE Federal Decree-Law No. 45 of 2021 (PDPL).

**Nibras** provides an air-gapped, zero-allocation personal assistant featuring:
- **Zero-Allocation UTF-8 Arabic Normalizer:** In-place stripping of diacritics, tatweel, and normalization of complex Arabic alef/hamza forms with zero dynamic memory allocation.
- **FadeMem Episodic Decay Engine:** Biological-inspired exponential half-life decay that continuously prunes transient operational noise while consolidating permanent executive decisions.
- **100% On-Device Sovereign Privacy:** Zero external network telemetry.

---

### 2. Architecture & Components
- **`src/nibras_agent.c`**: Standalone C99 CLI daemon and interactive REPL executive assistant.
- **`adapters/nibras_arabic_fademem_adapter.c`**: Zero-allocation UTF-8 Arabic normalizer, deterministic entity/action extractor, and FadeMem exponential decay engine ($I(t') = I_0 \cdot 2^{-(t'-t)/\tau_{1/2}}$).
- **`adapters/nibras_memory_store.c`**: Sovereign JSONL persistent memory store with pinned permanent nodes and full-text search.
- **`adapters/grounding_shield.c`**: Deterministic fact-checking engine and PII redactor complying with UAE PDPL Law No. 45 of 2021.
- **`tests/test_nibras_suite.c`**: 5-part end-to-end verification suite.

---

### 3. Build & CLI Usage
```bash
# Build standalone daemon, shared library, and test suites
make all

# Run complete deterministic verification suite
make test

# Run interactive CLI daemon
./build/nibras_agent

# Run realistic 7-day executive lifecycle demonstration
./build/nibras_agent --demo

# Ingest an Arabic task directly
./build/nibras_agent --add "اجتماع مع رئيس هيئة الصحة بدبي غداً الساعة 10 صباحاً"

# Advance time and prune decayed context (achieving >85% token reduction)
./build/nibras_agent --decay 7d

# Display Executive Cognitive HUD
./build/nibras_agent --hud
```
