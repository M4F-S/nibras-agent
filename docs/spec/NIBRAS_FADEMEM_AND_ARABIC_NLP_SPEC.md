# NIBRAS COGNITIVE MEMORY & ARABIC NLP ARCHITECTURE SPECIFICATION
## Sovereign Personal Assistant & Lifelong Cognitive Exocortex
### Best Personal AI Agent | Dubai Create AI Agents Championship 2026–2027

---

### 1. Executive Mission
Nibras (نبراس) is an air-gapped, zero-allocation personal cognitive memory assistant and executive exocortex designed for UAE executives, knowledge workers, and family offices. It eliminates the two core failure modes of personal AI assistants: **context window saturation (cognitive drift)** and **cloud privacy profiling**.

```
                        [Incoming Multi-Turn Interaction]
                                        │
                                        ▼
                   ┌──────────────────────────────────────────┐
                   │   ZERO-ALLOCATION UTF-8 ARABIC NORMALIZER│
                   │   - In-place diacritic / tatweel strip   │
                   │   - Alef / hamza / ta-marbuta unification│
                   │   - Zero dynamic heap allocation         │
                   └────────────────────┬─────────────────────┘
                                        │
                                        ▼
                   ┌──────────────────────────────────────────┐
                   │    FADEMEM EPISODIC CONTEXT DECAY        │
                   │   - Exponential half-life decay function │
                   │   - Prunes transient operational chatter │
                   │   - Consolidates permanent decisions     │
                   └────────────────────┬─────────────────────┘
                                        │
                                        ▼
                   ┌──────────────────────────────────────────┐
                   │     SOVEREIGN ON-DEVICE MEMORY GRAPH     │
                   │   - 100% air-gapped (UAE PDPL compliant) │
                   │   - > 85% token reduction at inference   │
                   └──────────────────────────────────────────┘
```

---

### 2. Core Architectural Pillars

#### 2.1 Zero-Allocation UTF-8 Arabic Normalizer
- **In-Place Transformation:** Operates on raw byte buffers in C99 without calling `malloc()` or generating heap fragmentation.
- **Diacritic (Tashkeel) Stripping:** Removes fat-ha, damma, kasra, sukun, shadda, and tanween (`0xD9 0x8B` through `0xD9 0x92`).
- **Tatweel (Kashida) Stripping:** Removes typographic elongation (`0xD9 0x80`).
- **Letter Unification:** Normalizes complex alef/hamza variants (`أ`, `إ`, `آ` $\rightarrow$ `ا`), `ة` $\rightarrow$ `ه`, and `ى` $\rightarrow$ `ي` with full multi-byte lead-byte and continuation-byte preservation.

#### 2.2 FadeMem Biological Episodic Memory Decay
- **The Context Bloat Problem:** Standard LLM chat histories grow linearly, accumulating irrelevant operational chatter (meeting confirmations, temporary links, casual greetings), diluting key executive preferences.
- **Mathematical Decay Model:**
  $$I(t') = I_0 \cdot 2^{-(t' - t) / \tau_{1/2}}$$
  Where:
  - $I_0$: Initial salience score assigned to the episodic memory node.
  - $t' - t$: Elapsed operational time.
  - $\tau_{1/2}$: Memory half-life parameter.
- **Permanent Node Pinning:** Executive decisions, permanent calendar commitments, and core personal preferences are flagged as permanent, bypassing decay.
- **Token Efficiency:** Achieves $> 85\%$ token reduction on multi-day conversational threads without cognitive drift.

#### 2.3 UAE PDPL Sovereign Privacy
- Built to comply with **UAE Federal Decree-Law No. 45 of 2021 on Personal Data Protection (PDPL)**.
- Operates 100% on-premises / on-device with zero telemetry, zero cloud exfiltration, and no profiling on foreign servers.
