# Nibras — implementation plan (repo-local)

**Owner:** the implementation agent assigned to `nibras-agent`. This file is the complete brief; you do not need any external document.
**Category:** Best Personal AI Agent.
**Rounds:** 1 = blocking, do now. 2 = memory (this is your product). 3 = RAG.

---

## Ground rules

- **You own this repo only.** Never edit `qala-agent`, `taryaq-agent`, or the umbrella.
- **Vendored files you must never edit** (other repos copy them; you are the canonical source):
  `adapters/nibras_arabic_fademem_adapter.c`, `adapters/nibras_adapters.h`, `adapters/nibras_memory_store.c/.h`.
  `adapters/grounding_shield.c/.h` is canonical in `qala-agent` — copy, do not edit.
- **Green on macOS is not evidence.** `-Wformat-truncation` / `-Wstringop-truncation` are GCC diagnostics. Every acceptance run must include:
  `docker run --rm -v "$PWD":/w -w /w gcc:13 sh -c "make clean && make test"`
- **Definition of done** (every round): `make clean && make test` exits 0 **twice consecutively**, the container run passes, no `build/`, `*.o`, `*.so`, `*.jsonl`, `__pycache__` tracked.
- **Claims policy.** Every README claim needs a proof command. Banned until proven: `100 % on-device`, `zero external`, `tamper-evident`, `AF_UNIX`, `Laya 322M`, `Jais 3`, `< 0.02 ms`, `zero dependencies`.

---

# Round 1 — make it runnable and honest (~3.5 h)

## N-1 · Build fix (blocking, 75 min)

Four diagnostics under `-O2 -Werror`. **The obvious fix for the first two is wrong** — read the note.

### (a) `adapters/nibras_memory_store.c:251,257`

`snprintf(node.summary, sizeof(node.summary), "%.64s (%.48s)", …)` compiles clean but **truncates mid-codepoint**. On a mixed Arabic/English title it emits invalid UTF-8 (`0xd9` dangling at byte 63), and the summary is written to a JSONL store that Python reads as strict UTF-8 — the store would stop loading.

Add and use:
```c
/* longest prefix of s, at most cap bytes, that does not split a UTF-8 sequence */
static size_t nib_utf8_prefix(const char *s, size_t cap) {
    if (!s || cap == 0) return 0;
    size_t n = 0;
    while (n < cap && s[n] != '\0') n++;
    while (n > 0 && ((unsigned char)s[n] & 0xC0) == 0x80) n--;
    return n;
}

/* "title (datetime)" into dst, truncating on codepoint boundaries */
static void nib_summary(char *dst, size_t cap, const char *title, const char *dt) {
    if (!dst || cap == 0) return;
    size_t tl = nib_utf8_prefix(title ? title : "", cap > 4 ? 64 : 0);
    size_t dl = nib_utf8_prefix(dt ? dt : "", 48);
    if (dl && tl + 3 + dl >= cap) dl = (tl + 3 < cap) ? cap - tl - 4 : 0;
    size_t w = 0;
    if (tl) { memcpy(dst, title, tl); w = tl; }
    if (dl && w + 3 < cap) {
        memcpy(dst + w, " (", 2); w += 2;
        memcpy(dst + w, dt, dl); w += dl; dst[w++] = ')';
    }
    dst[w] = '\0';
}
```
Call `nib_summary(node.summary, sizeof node.summary, info.appointment_title, info.datetime_str);` and the same with `info.action_verb`.

### (b) `adapters/nibras_arabic_fademem_adapter.c:464,467`

`strncpy(dst, src, sizeof-1)` never terminates a longer source. Add:
```c
static void nib_strlcpy(char *dst, const char *src, size_t cap) {
    if (!dst || cap == 0) return;
    size_t n = src ? strlen(src) : 0;
    if (n >= cap) n = cap - 1;
    if (n) memcpy(dst, src, n);
    dst[n] = '\0';
}
```
and replace the calls. Sweep the other `strncpy` sites (`:246,252,256`) — same latent missing-NUL.

**Accept:** `make clean && make all` exit 0. Plus a regression test: write an appointment with a 40-character mixed Arabic/English title, reload the store, assert the JSONL decodes as UTF-8.

## N-2 · gomaa guard (30 min)

`adapters/nibras_gomaa_bridge.py` searches `~/.gemini/antigravity/scratch/mnemosyne`, `~/mnemosyne`, `~/gomaa`. Replace with `GOMAA_HOME` (default `~/gomaa`). Decorate `tests/test_nibras_gomaa.py::TestNibrasGomaaIntegration` with `@unittest.skipUnless(GOMAA_AVAILABLE, "optional: gomaa Memory OS not installed (set GOMAA_HOME)")`. Add an "Optional integration: gomaa" README section.

**Accept:** `GOMAA_HOME=/nonexistent make test` exits 0 → `skipped=6`; the five C suites still `[PASS]`.

## N-3 · Make the privacy claim precise (30 min)

`README.md:14` says "100 % On-Device Sovereign Privacy: Zero external network telemetry" while the repo also ships an optional gomaa bridge. Keep the claim for the C engine and state the boundary in the same sentence: "The C99 engine performs no network calls. The optional gomaa integration is disabled unless installed and enabled."

## N-4 · Licence + attribution (30 min)

`LICENSE` (Apache-2.0), `NOTICE`:

```
This product includes software derived from:
  - Almaz (C99 cognitive organism)    — © Mohamed Fathy — <repo URL>
  - gomaa (agent memory OS, optional) — © Mohamed Fathy — <repo URL>
Components: adapters/nibras_arabic_fademem_adapter.c, adapters/nibras_memory_store.c,
src/nibras_agent.c, adapters/nibras_gomaa_bridge.py
```
README "Built on": new = memory store, entity extractor, decay policy, CLI; reused = Almaz engine; optional = gomaa.

## N-5 · CI + `make verify` (30 min)

`.github/workflows/ci.yml` on `ubuntu-latest` running `make clean && make test`; `make verify` target. Nibras needs no libcurl.

## N-6 · Mirror script and personal data (30 min)

Same three changes as every repo: `GDRIVE_ROOT` env with the personal path as default only; explicit `git add` paths; abort on `build/|*.o|*.so|*.jsonl|__pycache__`; no `rsync --delete` by default. Remove the Gmail address from `AGENT_INSTRUCTIONS.md`.

---

# Round 2 — memory is your product (3–5 days, after Round 1)

`nibras_store_search` is currently a **linear `strstr` scan**: no index, no ranking. That is the gap between the README and the code.

- [x] Add ranking to the C store: tokenise the summary, score with BM25 (or a small TF-IDF), return top-k with scores. Keep it allocation-light and deterministic — this is the demo.
- [x] Keep FadeMem as the pruning policy; rank *after* decay, not instead of it.
- [x] Index on load, not per query; a 1,000-note store must answer in < 1 ms.
- [x] Expose `search <query>` in the CLI; show the score and why a note ranked.
- [x] Test: 5 notes, query a 2-word Arabic/English mix, assert the right note ranks first and the pruned one is absent.

**Optional, same round:** a `--semantic` flag that uses gomaa (when installed) for embedding search, falling back to BM25 when absent. Never make it the default path — a judge must not need it.

---

# Round 3 — personal RAG (5–8 days, after Round 2)

- [x] Corpus = the user's own notes and prior decisions (this is the honest "personal knowledge base").
- [x] Chunk, index in SQLite FTS5, retrieve top-k, and **return citations**: note title + line range + `passage_sha256`.
- [x] Upgrade `grounding_shield` usage: a factual claim about the user's own data must cite the note it came from, or it is reported as unverified.
- [x] Demo: "when did I last meet the DHA chair and what did we decide?" → retrieved note + citation + confidence, no invented date.

**Do not** claim RAG over a model that is not in the repo.

---

## Report back (each round)

1. Exit codes: `make clean && make test` ×2, container run.
2. Round 1: `GOMAA_HOME=/nonexistent make test`; the UTF-8 round-trip test output (bytes → decoded string).
3. `git diff --stat`; claims-table commands with output.
4. Confirmation that no file outside `nibras-agent/` was touched.

**Do not start Round 2/3 work in Round 1's PR.**
