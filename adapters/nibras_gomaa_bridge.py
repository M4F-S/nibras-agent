"""
nibras_gomaa_bridge.py - Nibras Personal AI Agent & Gomaa Memory OS Bridge
Dubai Create AI Agents Championship 2026-2027 | Best Personal AI Agent
Tracks: Personal Cognitive Memory, Zero-Allocation Arabic Normalization,
        HippoRAG 2 Associative Retrieval, FadeMem Biological Decay,
        and UAE PDPL Law No. 45 of 2021 Privacy Shield.
"""

import os
import sys
import time
import math
import ctypes
import re
from typing import Dict, Any, List, Optional, Tuple

# 1. Resolve Gomaa Memory OS Import Paths
gomaa_home = os.environ.get("GOMAA_HOME", os.path.expanduser("~/gomaa"))
gomaa_home = os.path.expanduser(gomaa_home)
if os.path.isdir(gomaa_home) and gomaa_home not in sys.path:
    sys.path.insert(0, gomaa_home)

try:
    from gomaa.stores.sqlite import SQLiteStore
    from gomaa.hipporag import HippoRAGEngine
    from gomaa.security import AdmissionControl
    from gomaa.core import _neutralize_control_tokens
    GOMAA_AVAILABLE = True
except ImportError:
    GOMAA_AVAILABLE = False


# ==============================================================================
# C99 NIBRAS STRUCTURES & CTYPES BINDINGS
# ==============================================================================

class NibrasEntity(ctypes.Structure):
    _fields_ = [
        ("type", ctypes.c_int),
        ("label", ctypes.c_char * 32),
        ("text", ctypes.c_char * 96),
        ("offset", ctypes.c_size_t),
    ]


class NibrasExtractedInfo(ctypes.Structure):
    _fields_ = [
        ("entity_count", ctypes.c_size_t),
        ("entities", NibrasEntity * 16),
        ("is_action_item", ctypes.c_bool),
        ("is_appointment", ctypes.c_bool),
        ("is_executive_decision", ctypes.c_bool),
        ("action_verb", ctypes.c_char * 64),
        ("appointment_title", ctypes.c_char * 64),
        ("datetime_str", ctypes.c_char * 64),
        ("suggested_salience", ctypes.c_double),
        ("suggest_pin", ctypes.c_bool),
        ("suggested_valence", ctypes.c_int32),
    ]


class NibrasMemoryNode(ctypes.Structure):
    _fields_ = [
        ("id", ctypes.c_uint64),
        ("summary", ctypes.c_char * 128),
        ("valence", ctypes.c_int32),
        ("initial_intensity", ctypes.c_double),
        ("current_intensity", ctypes.c_double),
        ("created_utc", ctypes.c_uint64),
        ("token_cost", ctypes.c_uint32),
        ("valence_inverted", ctypes.c_bool),
        ("is_pinned", ctypes.c_bool),
        ("state", ctypes.c_int),
        ("category", ctypes.c_char * 32),
        ("raw_arabic", ctypes.c_char * 256),
    ]


class NibrasTelemetrySnapshot(ctypes.Structure):
    _fields_ = [
        ("total_raw_tokens", ctypes.c_uint32),
        ("active_tokens_after_prune", ctypes.c_uint32),
        ("tokens_saved_pct", ctypes.c_uint32),
        ("distilled_rules_count", ctypes.c_uint32),
        ("cognitive_friction_index", ctypes.c_double),
    ]


class NibrasC99Engine:
    """Zero-overhead wrapper over compiled libnibras.(dylib|so)."""

    def __init__(self, lib_dir: Optional[str] = None):
        self._lib = None
        if lib_dir is None:
            lib_dir = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "build")

        for name in ["libnibras.dylib", "libnibras.so", "libdubai_adapters.dylib", "libdubai_adapters.so"]:
            cand = os.path.join(lib_dir, name)
            if os.path.exists(cand):
                try:
                    self._lib = ctypes.CDLL(cand)
                    break
                except OSError:
                    continue

        if self._lib:
            self._setup_bindings()

    def _setup_bindings(self):
        if hasattr(self._lib, "arabic_normalize_utf8_hardened"):
            self._lib.arabic_normalize_utf8_hardened.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_size_t]
            self._lib.arabic_normalize_utf8_hardened.restype = ctypes.c_size_t

        if hasattr(self._lib, "nibras_extract_arabic_entities"):
            self._lib.nibras_extract_arabic_entities.argtypes = [ctypes.c_char_p, ctypes.POINTER(NibrasExtractedInfo)]
            self._lib.nibras_extract_arabic_entities.restype = ctypes.c_size_t

        if hasattr(self._lib, "nibras_fademem_step_hardened"):
            self._lib.nibras_fademem_step_hardened.argtypes = [
                ctypes.POINTER(NibrasMemoryNode),
                ctypes.c_size_t,
                ctypes.c_uint64,
                ctypes.c_double,
                ctypes.c_double,
                ctypes.c_double,
                ctypes.POINTER(NibrasTelemetrySnapshot)
            ]
            self._lib.nibras_fademem_step_hardened.restype = None

    def normalize_arabic(self, text: str) -> str:
        if not text:
            return ""
        if self._lib and hasattr(self._lib, "arabic_normalize_utf8_hardened"):
            src = text.encode("utf-8")
            buf_cap = max(len(src) * 2 + 64, 512)
            dst = ctypes.create_string_buffer(buf_cap)
            self._lib.arabic_normalize_utf8_hardened(src, dst, buf_cap)
            return dst.value.decode("utf-8", errors="replace")
        # Python fallback normalizer
        repl = text.replace("أ", "ا").replace("إ", "ا").replace("آ", "ا").replace("ى", "ي").replace("ة", "ه")
        repl = re.sub(r"[\u064B-\u065F\u0640]", "", repl)
        return repl

    def extract_entities(self, normalized_text: str) -> Dict[str, Any]:
        if self._lib and hasattr(self._lib, "nibras_extract_arabic_entities"):
            info = NibrasExtractedInfo()
            cnt = self._lib.nibras_extract_arabic_entities(normalized_text.encode("utf-8"), ctypes.byref(info))
            entities_list = []
            for i in range(info.entity_count):
                e = info.entities[i]
                entities_list.append({
                    "type": e.type,
                    "label": e.label.decode("utf-8", errors="ignore"),
                    "text": e.text.decode("utf-8", errors="ignore"),
                    "offset": e.offset
                })
            return {
                "entity_count": cnt,
                "entities": entities_list,
                "is_action_item": bool(info.is_action_item),
                "is_appointment": bool(info.is_appointment),
                "is_executive_decision": bool(info.is_executive_decision),
                "action_verb": info.action_verb.decode("utf-8", errors="ignore"),
                "appointment_title": info.appointment_title.decode("utf-8", errors="ignore"),
                "datetime_str": info.datetime_str.decode("utf-8", errors="ignore"),
                "suggested_salience": float(info.suggested_salience),
                "suggest_pin": bool(info.suggest_pin),
                "suggested_valence": int(info.suggested_valence),
            }

        # Python fallback entity extraction
        is_app = any(w in normalized_text for w in ["اجتماع", "موعد", "جلسه", "لقاء", "اتصال"])
        is_act = any(w in normalized_text for w in ["ارسال", "متابعه", "مراجعه", "سداد", "اعتماد"])
        is_dec = any(w in normalized_text for w in ["قرار", "قانون", "سياسه", "لايحه", "مرسوم"])
        salience = 0.95 if is_dec else (0.85 if is_app else (0.75 if is_act else 0.40))
        return {
            "entity_count": 1 if (is_app or is_act or is_dec) else 0,
            "entities": [],
            "is_action_item": is_act,
            "is_appointment": is_app,
            "is_executive_decision": is_dec,
            "action_verb": "متابعه" if is_act else "",
            "appointment_title": "اجتماع" if is_app else "",
            "datetime_str": "",
            "suggested_salience": salience,
            "suggest_pin": is_dec,
            "suggested_valence": 1 if is_dec else 0,
        }


# ==============================================================================
# NIBRAS GOMAA MEMORY OS BRIDGE
# ==============================================================================

class NibrasGomaaMemoryBridge:
    """
    Integrates Nibras C99 deterministic NLP with Gomaa 4-Quadrant Memory OS.
    - Episodic Memory: Multi-turn SQLite WAL conversation ledger
    - Semantic Memory: HippoRAG 2 Personalized PageRank knowledge graph
    - Working Memory: Active executive context buffer (7 ± 2 items)
    - FadeMem Biological Decay: Prunes ephemeral chatter, pins permanent rules
    - UAE PDPL Law No. 45: Zero cloud leakage, local air-gapped storage
    """

    def __init__(self, db_path: str = "data/nibras_gomaa.db", lib_dir: Optional[str] = None):
        self.c99 = NibrasC99Engine(lib_dir=lib_dir)
        self.db_path = os.path.expanduser(db_path)
        os.makedirs(os.path.dirname(os.path.abspath(self.db_path)), exist_ok=True)

        if GOMAA_AVAILABLE:
            self.store = SQLiteStore(self.db_path)
            self.hipporag = HippoRAGEngine(restart_prob=0.15, max_iter=25)
            self.admission = AdmissionControl()
        else:
            raise RuntimeError("Gomaa Memory OS could not be imported. Check GOMAA_SEARCH_PATHS.")

        self.turn_counter = {}

    def sanitize_input(self, text: str) -> Tuple[bool, str, str]:
        """Validates input against prompt injections and scrubs sensitive UAE PII."""
        # 1. Neutralize control tokens
        neutralized = _neutralize_control_tokens(text)

        # 2. Admission control pattern validation
        is_valid, reason = self.admission.validate(title="User Note", content=neutralized)
        if not is_valid:
            return False, neutralized, f"Security check failed: {reason}"

        # 3. UAE PDPL Law No. 45 PII Redaction: Mask Emirates ID and Credit Cards
        # Format: 784-YYYY-XXXXXXX-Z -> 784-YYYY-[REDACTED]-Z
        scrubbed = re.sub(r"\b(784)[- ]?([0-9]{4})[- ]?([0-9]{7})[- ]?([0-9])\b", r"\1-\2-*******-\4", neutralized)
        # Credit Card: 16 digits
        scrubbed = re.sub(r"\b[0-9]{4}[- ]?[0-9]{4}[- ]?[0-9]{4}[- ]?[0-9]{4}\b", "[REDACTED-PAYMENT-CARD]", scrubbed)

        return True, scrubbed, "OK"

    def ingest_executive_note(
        self,
        raw_text: str,
        session_id: str = "nibras_executive_session",
        author: str = "Mohamed Fathy",
        tags: Optional[List[str]] = None
    ) -> Dict[str, Any]:
        """
        Ingests user note into Gomaa memory system:
        1. Sanitizes & validates with Gomaa AdmissionControl & PDPL Privacy Shield
        2. Normalizes Arabic in zero-allocation C99
        3. Extracts entities & salience in C99
        4. Logs Episodic Event in Gomaa SQLite WAL ledger
        5. Updates Working Memory buffer
        6. Pins permanent decisions into Gomaa Semantic Memory with weighted links
        """
        t0 = time.perf_counter_ns()
        if tags is None:
            tags = ["nibras", "executive"]

        # Step 1: Sanitize
        is_safe, sanitized_text, sec_msg = self.sanitize_input(raw_text)
        if not is_safe:
            return {
                "status": "REJECTED_SECURITY",
                "reason": sec_msg,
                "sanitized": sanitized_text,
                "session_id": session_id,
            }

        # Step 2: C99 Arabic Normalization
        norm_text = self.c99.normalize_arabic(sanitized_text)

        # Step 3: C99 Entity Extraction
        ext = self.c99.extract_entities(norm_text)

        # Step 4: Episodic Event Ledger (Turn Logging)
        current_turn = self.turn_counter.get(session_id, 0) + 1
        self.turn_counter[session_id] = current_turn

        if ext["is_appointment"]:
            category = "appointment"
        elif ext["is_executive_decision"]:
            category = "decision"
        elif ext["is_action_item"]:
            category = "action"
        else:
            category = "chatter"

        response_summary = f"[{category.upper()}] {norm_text[:80]}"
        evt = self.store.log_episodic_event(
            session_id=session_id,
            turn_index=current_turn,
            origin_agent="nibras_agent",
            event_type="executive_note",
            stimulus=sanitized_text,
            response=response_summary,
            outcome_status="success",
            surprise_factor=ext["suggested_salience"]
        )

        # Step 5: Working Memory Update (Miller's 7 +/- 2 Items)
        current_wm = self.store.get_working_memory(session_id)
        subtask_stack = current_wm["subtask_stack"] if current_wm else []
        attention_win = current_wm["attention_window"] if current_wm else []

        if ext["is_action_item"] or ext["is_appointment"]:
            summary_item = ext["appointment_title"] or ext["action_verb"] or norm_text[:60]
            if summary_item not in subtask_stack:
                subtask_stack.insert(0, summary_item)
                subtask_stack = subtask_stack[:7]  # Retain top 7 active items

        attention_win.insert(0, {
            "category": category,
            "text": norm_text[:80],
            "salience": ext["suggested_salience"],
            "timestamp": int(time.time())
        })
        attention_win = attention_win[:7]

        self.store.update_working_memory(
            session_id=session_id,
            agent_role="executive_assistant",
            active_goal=f"Coordinate {author} executive schedule & governance",
            subtask_stack=subtask_stack,
            hypotheses=["Air-gapped on-device privacy active", "FadeMem decay active"],
            attention_window=attention_win
        )

        # Step 6: Semantic Memory Consolidation (Permanent Decisions & Rules)
        semantic_note_id = None
        if ext["is_executive_decision"] or ext["suggest_pin"]:
            note_title = f"Executive Rule: {norm_text[:40]}"
            semantic_note_id = self.store.upsert_note(
                title=note_title,
                content=f"Original Text: {raw_text}\nNormalized: {norm_text}\nCategory: {category}",
                tags=tags + [category, "sovereign_rule"],
                memory_type="semantic",
                salience=1.0,
                episodic_parent_hash=evt["content_hash"]
            )
            # Create link to anchor node
            anchor_id = self.store.upsert_note(
                title="Sovereign Enterprise Governance",
                content="Master anchor for UAE corporate decisions and regulatory compliance.",
                tags=["master", "governance"]
            )
            self.store.add_weighted_link(
                source_id=semantic_note_id,
                target_id=anchor_id,
                relation_type="governed_by",
                weight=1.5
            )

        t1 = time.perf_counter_ns()
        return {
            "status": "INGESTED",
            "session_id": session_id,
            "turn_index": current_turn,
            "category": category,
            "raw_arabic": sanitized_text,
            "sanitized_arabic": sanitized_text,
            "normalized_arabic": norm_text,
            "extracted_entities": ext["entities"],
            "salience": ext["suggested_salience"],
            "is_pinned": ext["suggest_pin"],
            "episodic_event_id": evt["id"],
            "semantic_note_id": semantic_note_id,
            "latency_ms": round((t1 - t0) / 1_000_000.0, 3)
        }

    def associative_recall(self, query: str, top_k: int = 5) -> List[Dict[str, Any]]:
        """
        Associative Retrieval via HippoRAG 2 Personalized PageRank.
        Diffuses probability over graph nodes seeded from query keywords.
        """
        norm_q = self.c99.normalize_arabic(query)
        kw_results = self.store.search_keyword(norm_q, top_k=5)
        if not kw_results:
            # Fallback search by raw query tokens
            kw_results = self.store.search_keyword(query, top_k=5)

        seed_scores = {}
        for r in kw_results:
            seed_scores[r["title"]] = 1.0

        if not seed_scores:
            return []

        adj = self.store.get_graph_adjacency()
        ppr_scores = self.hipporag.compute_ppr(adj, seed_scores)

        # Sort nodes by stationary distribution rank
        ranked_titles = sorted(ppr_scores.items(), key=lambda x: x[1], reverse=True)[:top_k]
        output = []
        for title, score in ranked_titles:
            notes = self.store.search_keyword(title, top_k=1)
            if notes:
                item = notes[0]
                item["hipporag_ppr_score"] = round(score, 4)
                output.append(item)
        return output

    def simulate_fademem_decay(self, elapsed_days: float = 7.0, half_life_days: float = 1.0) -> Dict[str, Any]:
        """
        Simulates Ebbinghaus / FadeMem biological memory decay.
        Ephemeral conversational chatter decays (I < 0.25 pruned).
        Consolidated permanent rules stay intact.
        """
        tau_half_sec = half_life_days * 86400.0
        elapsed_sec = elapsed_days * 86400.0

        events = self.store.get_episodic_events()
        total_tokens = len(events) * 45  # average token weight
        retained_tokens = 0
        pruned_count = 0
        retained_count = 0

        for ev in events:
            # Pinned or decision events have high salience (immune to decay)
            surprise = ev.get("surprise_factor", 0.5)
            if surprise >= 0.90:
                retained_tokens += 45
                retained_count += 1
            else:
                # Biological half-life decay formula: I(t) = I_0 * 2^(-dt / tau)
                decayed_intensity = surprise * math.pow(2.0, -elapsed_sec / tau_half_sec)
                if decayed_intensity >= 0.25:
                    retained_tokens += 45
                    retained_count += 1
                else:
                    pruned_count += 1
                    # Pruned chatter nodes are evicted from active prompt context (0 tokens)

        reduction_pct = round((1.0 - (retained_tokens / max(1, total_tokens))) * 100.0)

        return {
            "elapsed_days": elapsed_days,
            "total_events": len(events),
            "retained_count": retained_count,
            "pruned_count": pruned_count,
            "tokens_before": total_tokens,
            "tokens_after": retained_tokens,
            "context_reduction_pct": max(0, reduction_pct),
            "meets_target": reduction_pct >= 85
        }
