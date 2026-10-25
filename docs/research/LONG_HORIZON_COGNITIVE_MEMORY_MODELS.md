# LONG-HORIZON COGNITIVE MEMORY & ARABIC NLP BENCHMARKS
## Research Foundations for Nibras (نبراس)
### Best Personal AI Agent | Dubai Create AI Agents Championship 2026–2027

---

### 1. Biological Memory Foundations in Autonomous Agents
1. **The Ebbinghaus Forgetting Curve:** Human memory decays logarithmically unless actively consolidated by spaced repetition. AI agent architectures that retain 100% of raw conversational history suffer from catastrophic attention dilution, prompt injection vulnerability, and extreme KV-cache memory bloat.
2. **Episodic vs Semantic Memory Separation:**
   - *Episodic Memory:* Temporary context nodes representing recent events, interactions, and tasks. Decays exponentially via `FadeMem`.
   - *Semantic Memory:* Stable, permanent knowledge graph nodes representing invariant facts, identity, preferences, and corporate policies. Consolidated during low-activity intervals.

---

### 2. Arabic Natural Language Processing at the Edge
1. **Unicode UTF-8 Complexities in Arabic:**
   - Arabic characters occupy 2 bytes in UTF-8 (`0xD8 0x80` through `0xD9 0xBF`).
   - Naive single-byte string manipulations cause byte-boundary splits and mojibake corruption.
   - Nibras's C99 normalizer implements strict UTF-8 lead-byte (`0xC0–0xDF`) and continuation-byte (`0x80–0xBF`) decoding to guarantee deterministic normalization without heap allocation.
2. **Impact on Embeddings and LLM Tokenizers:**
   - Arabic text with diacritics generates up to 3× more subword tokens than normalized text in standard foundation model tokenizers (e.g. Llama, Mistral, Jais).
   - In-place normalization reduces token consumption, speeds up prefill latency, and dramatically improves vector retrieval accuracy.
