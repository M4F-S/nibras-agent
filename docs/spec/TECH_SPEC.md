# NIBRAS (نبراس) TECHNICAL ARCHITECTURE & IMPLEMENTATION SPECIFICATION
## Dubai Create AI Agents Championship 2026–2027 | Best Personal AI Agent
**System Role:** Sovereign Dubai Life OS, Civic Navigator & Executive Twin  
**Privacy Standard:** 100% On-Device Air-Gap (UAE Federal Decree-Law No. 45 of 2021 PDPL)  
**Version:** 2.0 (Canonical Implementation Contract)  
**Author:** Mohamed Fathy, BPharm (Lead Systems Architect & Founder)

---

### 1. Architectural Philosophy: The Sovereign Executive Twin
Nibras is an air-gapped, sovereign personal AI assistant designed for UAE executives, investors, and residents. It solves the two critical vulnerabilities of current consumer AI: **cloud privacy profiling** and **context window drift/bloat**.

```
                         ┌────────────────────────────────────────┐
                         │       NIBRAS EXECUTIVE TWIN UI         │
                         │   - Natural Arabic / English dialogue  │
                         │   - Cognitive Friction Index (HUD)     │
                         └───────────────────┬────────────────────┘
                                             │
                                             ▼
                         ┌────────────────────────────────────────┐
                         │   DUBAI CIVIC NAVIGATOR & WORKFLOWS    │
                         ├───────────────────┬────────────────────┤
                         │ • GDRFA Visa      │ • DLD Ejari Rents  │
                         │ • DEWA Utilities  │ • RTA Salik/Fines  │
                         │ • KHDA Schools    │ • UAE Pass PKI     │
                         └───────────────────┬────────────────────┘
                                             │
                                             ▼
                         ┌────────────────────────────────────────┐
                         │      FADEMEM MEMORY SYSTEM (C99)       │
                         ├───────────────────┬────────────────────┤
                         │ • In-place Arabic │ • Half-life decay  │
                         │   UTF-8 Normalize │ • Decision Pinning │
                         └───────────────────┬────────────────────┘
                                             │
                                             ▼
                         ┌────────────────────────────────────────┐
                         │    SOVEREIGN ON-DEVICE VAULT (DB)      │
                         │   - Encrypted local Family Office DB   │
                         │   - 100% Air-Gapped (UAE PDPL Law 45)  │
                         └────────────────────────────────────────┘
```

---

### 2. Dubai Civic Navigator: The 6 Statutory Engines

Nibras codifies official Dubai statutory rules, eliminating administrative friction for residents and business leaders:

#### 2.1 GDRFA Golden Visa Eligibility Engine
Determines 10-year Golden Visa eligibility without external API leakage:
1. **Real Estate Investor Category:**
   - Single or aggregate property value $\ge \text{AED 2,000,000}$.
   - Mortgaged properties eligible if letter from bank confirms $\ge \text{AED 2,000,000}$ already paid.
2. **Skilled Professional Category:**
   - Valid UAE employment contract.
   - MoHRE Occupational Classification: Level 1 (Executive/Manager) or Level 2 (Professional).
   - Monthly basic salary $\ge \text{AED 30,000}$.
   - Attested Bachelor's degree (or higher).
3. **Dependent Sponsorship:**
   - Uncapped sponsorship of spouse and children (sons up to 25, unmarried daughters without age limit, parents with 1-year renewable visas).

#### 2.2 DLD Ejari & Rental Increase Engine (Decree No. 43 of 2013)
Calculates statutory legal limits on residential/commercial rent increases:
- **Bracket 1:** Current rent is $\le 10\%$ below average market rent $\rightarrow$ **0% increase permitted**.
- **Bracket 2:** Current rent is $11\% - 20\%$ below average $\rightarrow$ **maximum 5% increase**.
- **Bracket 3:** Current rent is $21\% - 30\%$ below average $\rightarrow$ **maximum 10% increase**.
- **Bracket 4:** Current rent is $31\% - 40\%$ below average $\rightarrow$ **maximum 15% increase**.
- **Bracket 5:** Current rent is $> 40\%$ below average $\rightarrow$ **maximum 20% increase**.
- **Statutory Notice Period:** Landlord MUST serve formal notice via Notary Public or registered mail at least **90 calendar days** prior to lease expiry (Law No. 33 of 2008 Article 14). Otherwise, the lease renews under identical terms.

#### 2.3 DEWA Smart Utilities Engine
1. **Move-in Security Deposit Calculator:**
   - Residential Apartment: **AED 2,000.00**.
   - Residential Villa: **AED 4,000.00**.
   - Commercial / Free Zone: Custom calculated based on connected electrical load.
2. **Slab Tariff Engine (Expatriate Residential):**
   - *Electricity:*
     * Slab 1 ($0 - 2,000\text{ kWh}$): 23 fils / kWh.
     * Slab 2 ($2,001 - 4,000\text{ kWh}$): 28 fils / kWh.
     * Slab 3 ($4,001 - 6,000\text{ kWh}$): 32 fils / kWh.
     * Slab 4 ($> 6,000\text{ kWh}$): 38 fils / kWh.
   - *Fuel Surcharge:* Fixed statutory addition of 5 fils / kWh.
   - *Water:* 3.5 fils / IG ($0-6000$), 4.0 fils ($6001-12000$), 4.6 fils ($>12000$).

#### 2.4 RTA Smart Mobility Engine
1. **Salik Toll Accounting:**
   - Standard toll rate: **AED 4.00 per gate passage**.
   - Automatic low-balance alert trigger: $\le \text{AED 20.00}$.
2. **Vehicle Registration Renewal Checklist:**
   - Technical inspection certificate (valid for 30 days).
   - Valid 13-month UAE motor vehicle insurance policy.
   - Clearance of outstanding traffic fines (Dubai Police / RTA / Abu Dhabi Police).
3. **Parking Zone Guidance:**
   - Zone A (Commercial road side): AED 4.00/hr.
   - Zone B (Commercial parking lot): AED 3.00/hr.
   - Zone C (Residential road side): AED 2.00/hr.
   - Zone D (Residential parking lot): AED 2.00/hr.

#### 2.5 KHDA Education Advisory
- **DSIB Rating Tiers:** Outstanding, Very Good, Good, Acceptable, Weak.
- **Tuition Fee Increase Framework:** Connects school DSIB inspection rating with the official Dubai Educational Cost Index (ECI) to compute maximum allowable tuition adjustments.

#### 2.6 UAE Pass PKI & Digital Signature Engine
- Verifies local cryptographic digital identity credentials without transmitting plaintext personal identifiers to third parties.

---

### 3. Sovereign Family Office Vault Schema (`data/nibras_vault.db`)

All personal contracts, investment holdings, and estate documents reside in a local SQLite database:

```sql
CREATE TABLE vault_assets (
    asset_id TEXT PRIMARY KEY,
    asset_name TEXT NOT NULL,
    asset_type TEXT NOT NULL,                -- 'REAL_ESTATE', 'EQUITY', 'BANK_ACCOUNT', 'VEHICLE'
    valuation_aed REAL NOT NULL,
    jurisdiction TEXT NOT NULL,              -- 'DUBAI_MAINLAND', 'DIFC', 'ADGM', 'JAFZA'
    ownership_entity TEXT NOT NULL,          -- Individual name or SPV/Holding company
    registration_number TEXT NOT NULL,       -- Title deed number or trade license
    acquisition_date TEXT NOT NULL
);

CREATE TABLE vault_contracts (
    contract_id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    counterparty TEXT NOT NULL,
    effective_date TEXT NOT NULL,
    expiry_date TEXT NOT NULL,
    renewal_notice_deadline TEXT NOT NULL,   -- e.g. 90 days prior for Ejari
    file_sha256 TEXT NOT NULL,
    governing_law TEXT NOT NULL              -- 'DIFC', 'UAE_FEDERAL', 'DUBAI_COURTS'
);

CREATE TABLE executive_commitments (
    commitment_id TEXT PRIMARY KEY,
    description TEXT NOT NULL,
    stakeholder TEXT NOT NULL,
    due_date TEXT NOT NULL,
    is_statutory BOOLEAN DEFAULT 0,
    status TEXT NOT NULL                     -- 'PENDING', 'COMPLETED', 'ESCALATED'
);
```

---

### 4. FadeMem Biological Episodic Memory Engine

#### 4.1 Decay Mathematics
Memory nodes decay exponentially according to the Ebbinghaus retention function:
$$I(t') = I_0 \cdot 2^{-(t' - t) / \tau_{1/2}}$$
Where:
- $I_0 = 1.0$ (initial salience upon memory creation).
- $\tau_{1/2} = 86,400\text{ seconds}$ (24-hour half-life).
- $t' - t$: Elapsed operational time.

#### 4.2 Dynamic Pruning & Pinning Policy
1. **Transient Chatter:** Casual remarks, confirmations, temporary links decay below the retention threshold ($I < 0.25$) within 48 hours and are excluded from LLM inference context.
2. **Permanent Decisions & Rules:** Executive directives, statutory deadlines, and personal preferences are pinned with $I = 1.0$ (immune to decay).
3. **Efficiency:** Achieves **$> 85\%$ token reduction** on multi-turn conversations without loss of critical context.

#### 4.3 Cognitive Friction Index (CFI) HUD
Monitors executive mental friction based on:
- Unresolved statutory deadlines (Ejari renewal $< 90$ days, Emirates ID expiry).
- Outstanding financial reconciliation tasks.
- Ambiguous or conflicting commitments.

---

### 5. Conformance & Verification Suite
Implementers must ensure all tests pass:
1. `make test_nibras` (C99 standalone Arabic normalizer and FadeMem decay tests).
2. `python3 tests/test_nibras_gomaa.py` (Longitudinal 4-quadrant memory tests).
3. `tests/test_civic_navigator.py` (Verification of Golden Visa, Ejari 5-tier bracket, DEWA, RTA, and KHDA calculations).
