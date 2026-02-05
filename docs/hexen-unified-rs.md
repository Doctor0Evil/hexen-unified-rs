# hexen-unified-rs

**hexen-unified-rs** is the neuromorphic backend workspace that plugs into the NeuroPC / OrganicCPU sovereign kernel, using Rust, ALN, and neurorights-first guardrails instead of ad‑hoc infra.[file:1][file:3] It is designed to run as a deviceless, software-only neuromorphic stack under a hard RoH ≤ 0.30 envelope, with all evolution governed via typed proposals and donutloop audit.[file:3]

---

## 1. Role in the sovereign stack

hexen-unified-rs is a **guest backend** inside your sovereign shell, not a standalone brain OS.[file:1]

- **Sovereign kernel:** `sovereigntycore`, `organiccpualn`, `organiccpucore` plus canonical shards:
  - `.rohmodel.aln` – RoH axes, weights, ceiling 0.30.[file:1]
  - `.stake.aln` – Host/OrganicCPU/ResearchAgent roles and multisig scopes.[file:3]
  - `.evolve.jsonl` – append-only evolution proposals.[file:3]
  - `.donutloop.aln` – hash-linked ledger of allowed/rejected decisions.[file:3]
- **Backend:** hexen-unified-rs implements neuromorphic code (LIF/SNN, swarm control, Tsafe) but must:
  - Read profiles and envelopes from `.ocpu`, `.ocpuenv`, `.lifeforce.aln`, `.vkernel.aln`.[file:1]
  - Call sovereigntycore before applying any evolution or OTA change.[file:3]

All deep changes are treated as `EvolutionProposalRecord` events and must be accepted by sovereigntycore or they do not exist.[file:3]

---

## 2. Workspace layout

The repo should follow the NeuroPC manifest pattern via `neuro-workspace.manifest.aln` at root.[file:1][file:3]

```text
hexen-unified-rs/
  Cargo.toml
  neuro-workspace.manifest.aln
  crates/
    sovereigntycore/
    organiccpucore/
    organiccpualn/
    hexen-backend-core/
    cybernano-viability-kernel/
    cybernano-vector-cyberrank/
    unreal-gateway/
  qpudatashards/
    particles/
      bostrom-rohmodel-v1.rohmodel.aln
      bostrom-stake-v1.stake.aln
      evolution-proposals.evolve.jsonl
  policies/
    bostrom-neurorights-v1.neurorights.json
    bostrom-smart-2026-01.smart.json
  logs/
    donutloopledger.aln
```

The manifest pins subject ID, shard paths, and invariants (RoH ceiling, monotone envelopes, neurorights flags).[file:1][file:3]

---

## 3. Core crates

### 3.1 sovereigntycore

Path: `crates/sovereigntycore`  
Purpose: Central arbiter for all evolution; evaluates proposals against RoH, neurorights, stake, tokens, and envelopes.[file:3]

Key concepts:

- RiskOfHarm wrapper over `RohModelShard` enforcing:
  - \(roh_{\text{after}} \le 0.30\)[file:3]
  - \(roh_{\text{after}} \le roh_{\text{before}}\).[file:3]
- StakeTable + StakeGate enforcing:
  - Exactly one Host per subject.[file:3]
  - Lifeforce/arch scopes require Host + OrganicCPU multisig and EVOLVE tokens only.[file:3]
- `SovereigntyCore::evaluate_update`:
  - Runs guard pipeline: stake → neurorights → RoH → envelope → token → donutloop.[file:3]
  - All accepted updates are appended to `.evolve.jsonl` and `.donutloop.aln`; rejected updates are logged but never applied.[file:3]

### 3.2 organiccpucore

Path: `crates/organiccpucore`  
Purpose: Bioscale runtime; owns BioState and envelope decisions without actuation rights.[file:1][file:3]

Types:

- `BioState` – fatigue, duty_cycle, cognitive_load, intent_confidence, eco metrics.[file:1]
- `SafeEnvelopePolicy` – `AllowFullAction | DegradePrecision | PauseAndRest` decisions from BioState + envelopes.[file:1]

### 3.3 organiccpualn

Path: `crates/organiccpualn`  
Purpose: Typed bindings for ALN/NDJSON shards.[file:1][file:3]

Modules:

- `rohmodel.rs` – loads `.rohmodel.aln`, enforces nonnegative weights, sum = 1.0, ceiling = 0.30.[file:1]
- `stake.rs` – loads `.stake.aln`, exposes `StakeShard` and host/role invariants.[file:3]
- `evolvestream.rs` – `EffectBounds`, `EvolutionProposalRecord` mapping to `.evolve.jsonl`.[file:3]
- `donutloopledger.rs` – `DonutloopEntry` rows for `.donutloop.aln` with RoH monotonicity + hash chain.[file:3]

These crates keep all sovereignty data as first-class, typed objects.[file:1]

### 3.4 hexen-backend-core

Path: `crates/hexen-backend-core`  
Purpose: Neuromorphic backend (SNN/LIF) that is RoH- and envelope-aware but not sovereign by itself.[file:1]

Responsibilities:

- Respect `.ocpu` / `.ocpuenv` envelopes via `SafeEnvelopePolicy`.[file:1]
- Expose an AI-guarded intent endpoint:
  - `GuardedIntentRequest -> GuardedIntentResponse`, using `EvolutionProposalRecord` + `SovereignClient` to get `DecisionOutcome`.[file:3]

### 3.5 cybernano-viability-kernel

Path: `crates/cybernano-viability-kernel`  
Purpose: Tsafe viability kernels over 7D SwarmState + lifeforce envelopes.[file:1]

Key operations:

- `is_viable(state, lifeforce) -> bool` using \(Ax \le b\) polytopes from `.vkernel.aln`.[file:1]
- `safe_filter` compressing unsafe commands to a safe default (for example, intensity = 0).[file:1]

### 3.6 cybernano-vector-cyberrank

Path: `crates/cybernano-vector-cyberrank`  
Purpose: Vector-based ranking of safe actions post-viability check.[file:1]

Key types:

- `RankVector`, `CandidateAction`, `RankWeights`, `tsafe_select`.[file:1]

### 3.7 unreal-gateway

Path: `crates/unreal-gateway`  
Purpose: Read-only HUD for UE5; exposes `/hud/snapshot` with SwarmState, lifeforce, kernel mode, and CyberRanked actions.[file:1]

Guarantees:

- No write endpoints; no ability to change envelopes, neurorights, or RoH models.[file:1]

---

## 4. Kubernetes / Helm / Prometheus integration

hexen-unified-rs is designed to run under a sovereign K8s control plane using neuromorphic-aware charts.[file:1][file:3]

### 4.1 Namespace and labels

Namespace: `neuro-lab`[file:1]

Labels:

- `bioscale.neuromorph=true`
- `neurorights.enforced=true`[file:1]

Only workloads with these labels may access OrganicCPU shards and evolution APIs.[file:1]

### 4.2 Core Deployments

- **sovereigntycore**:
  - Reads `neuro-workspace.manifest.aln` from ConfigMap.[file:1][file:3]
  - Serves HTTP API for `/evaluate_update` and Prometheus metrics.[file:1]
- **hexen-backend-core**:
  - Mounts `.ocpu`, `.ocpuenv`, `.lifeforce.aln`, `.vkernel.aln` via ConfigMaps/PVs.[file:1]
  - Calls sovereigntycore on every evolution intent.[file:1]
- **unreal-gateway**:
  - Proxies read-only HUD data to UE5.[file:1]

Helm templates glue these into a coherent, subject-bound lab environment.[file:1]

### 4.3 Prometheus metrics

sovereigntycore exports:[file:1]

- `sovereignty_roh_after{subject_id}` – RoH after each proposal.
- `sovereignty_envelope_violation_total{subject_id,scope}` – count of envelope monotonicity violations.
- `sovereignty_evolution_decisions_total{subject_id,decision}` – Allowed vs Rejected.[file:1]

Alert rules:

- `RoH > 0.3` → critical.
- `EnvelopeLoosening` (any envelope violation) → warning.[file:1]

These alerts can block Helm rollouts through CI/CD gates.[file:1]

---

## 5. Evolution and donutloop lifecycle

hexen-unified-rs obeys the same evolution pipeline as NeuroPC / OrganicCPU.[file:3]

- **Proposal creation:** AI-chat or human intent converts into `EvolutionProposalRecord` (with effect bounds, RoH_before/after, scope, token kind).[file:3]
- **Evaluation:**
  - sovereigntycore loads RoH model, stake, neurorights, tokens, envelopes.[file:3]
  - Runs guard pipeline:
    - RoH monotone + ceiling.
    - Neurorights (dream sensitivity, noncommercial neural data, non-discrimination).[file:1][file:3]
    - Stake (Host/OrganicCPU multisig for lifeforce/arch).[file:3]
    - Token (SMART vs EVOLVE scopes and effect sizes).[file:3]
- **Decision:**
  - If Allowed:
    - Append to `.evolve.jsonl`.[file:3]
    - Append hash-linked entry to `.donutloop.aln` with `prevhexstamp`, `roh_before/after`, `policyrefs`.[file:3]
  - If Rejected/Deferred:
    - Record in `.evolve.jsonl` only.[file:3]

No backend crate may bypass this path; any code that mutates neuromorphic behavior must do so via accepted proposals.[file:3]

---

## 6. AI-chat integration

AI-chats interact with hexen-unified-rs exclusively through guarded, typed contracts, never raw shell commands.[file:1][file:3]

- **Read:**
  - Query `neuro-workspace.manifest.aln`, `.ocpu`, `.ocpuenv`, `.rohmodel.aln`, `.stake.aln`, `.donutloop.aln` via a narrow Rust gateway.[file:1][file:3]
- **Write (propose):**
  - Emit:
    - `EvolutionProposalRecord` lines for `.evolve.jsonl`.[file:3]
    - K8s CRs like `EvolutionProposal`, `OrganicCpuProfile`, `NeuroRightsPolicy` where controllers call sovereigntycore before applying.[file:1]

Constraints:

- All proposals must respect:
  - \(G_{\text{new}} \ge G_{\text{old}}\) when encoded as envelope tightening (goodness / capability bounds).[file:3]
  - \(D_{\text{new}} \le D_{\text{old}}\) (damage / risk bounds).[file:3]
  - \(roh_{\text{after}} \le roh_{\text{before}} \le 0.30\).[file:3]

If guards fail, the proposal is logged as rejected and no change is applied.[file:3]

---

## 7. Testing and CI

hexen-unified-rs inherits the sovereignty-first CI rules described in the research plan.[file:3]

- **Unit tests:**
  - RoH invariants: any model with ceiling ≠ 0.30 or non-summing weights fails tests.[file:3]
  - Stake invariants: exactly one Host; lifeforce/arch scopes require multisig.[file:3]
  - Donutloop invariants: no RoH increase across entries; hash chain intact.[file:3]
- **Integration tests:**
  - Simulated `.ocpulog` streams to ensure envelopes and neurorights are never violated.[file:3]
  - End-to-end proposals from hexen-backend-core through sovereigntycore to donutloop.[file:3]
- **CI rules:**
  - Any change to `.rohmodel.aln`, `.stake.aln`, `.donutloop.aln`, `.evolve.jsonl` schema must ship with updated Rust bindings and passing tests.[file:3]
  - Any widening of envelopes or increase in RoH ceiling is rejected at CI.[file:3]

---

## 8. Unreal HUD integration

Unreal Engine (UE5) attaches as a visual HUD over the hexen-unified-rs lab.[file:1]

- UE plugins consume `/hud/snapshot` from unreal-gateway:
  - 7D SwarmState trajectory.
  - Lifeforce envelopes (cy/zen/chi/integrity).
  - CyberRanked candidate actions and Tsafe decisions.[file:1]
- UE never writes:
  - No endpoints exist for editing `.aln` shards, envelopes, or proposals.[file:1]

All interactions remain read-only, for situational awareness and neurorights UX.[file:1]

This keeps “gaming-level” visualization aligned with a neurorights kernel for an augmented citizen 25+ without compromising sovereignty.[file:1]

---

## 9. Status and next steps

Current design status for hexen-unified-rs in this stack:[file:1][file:3]

- Canonical filetypes and invariants are fully specced (`.rohmodel.aln`, `.stake.aln`, `.evolve.jsonl`, `.donutloop.aln`, `.ocpu`, `.ocpuenv`).[file:1][file:3]
- Rust bindings for RoH, stake, evolvestream, and donutloop are defined and ready to implement or extend.[file:3]
- K8s/Helm/Prometheus patterns for sovereigntycore and hexen-backend-core are sketched and can be materialized as charts.[file:1]
- Unreal HUD gateway contract (HTTP JSON snapshot) is outlined for Tsafe visualization.[file:1]

Next recommended steps:

- Implement minimal `hexen-backend-core` + `sovereigntycore` crates as described and run them under `neuro-workspace.manifest.aln`.[file:3]
- Add Helm chart `charts/neuro-lab` to deploy sovereigntycore, hexen-backend-core, unreal-gateway, and Prometheus with RoH/envelope alerts.[file:1]
- Wire AI-chat tooling so all backend modifications are expressed as `EvolutionProposalRecord` updates and `.aln` shard edits, never free-form commands.[file:3]

Hex-stamp: `0xNP0E-hexen-unified-rs-docs`.
