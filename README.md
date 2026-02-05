# hexen-unified-rs
hexen-unified-rs is a neuromorphic-computing engine for delivering content, and data-streaming to BCI, EEG, or organically-integrated neuromorphic-components such-as NeuroPC, or Reality.os. hexen-unified-rs is designed-to develop next-generation gaming-content, &amp; maintain sovereign, &amp; neural-safe pipelines for content-freedom, neural-sovereignty, and sets new-heights for gaming with age-gating; Where some neuromorphic-content is intended-for ages 25+

`hexen-unified-rs` is a Rust-first neuromorphic and BCI-safe backend that wraps all bio-adjacent behavior in explicit bioscale envelopes, ALN neurorights grammars, and auditable evolution windows.[web:10]

## Goals

- Centralize neuromorphic and BCI control through typed Rust crates that expose explicit **safety** and duty-cycle limits at compile-time.[web:12]
- Require evidence-tagged upgrades, routed through ALN grammars and neurorights clauses before any actuation path can be built or deployed.[web:10]
- Keep all workers, game/XR integrations, and Xbox/BLE clients behind shared envelopes so remote actuation remains thermodynamically and neurorights constrained.[web:13]

## Workspace structure

- `crates/hexen-backend-core`: HTTP/gRPC entrypoint, ALN gates, circuit breakers.
- `crates/cyberswarm-neurostack`: BCI routing, safety thresholds, evolution windows.
- `crates/bioscale-upgrade-store`: UpgradeDescriptor, HostBudget, EvidenceBundle types.
- `crates/hexen-evidence-registry`: EvidenceTag registries wired to ALN specs.
- `clients/`: Android, Unity/XR, and web frontends talking to Rust guard endpoints.[web:12]

## Getting started

- `cargo build` to compile all workspace crates.
- `cargo run -p hexen-backend-core` to start the primary HTTP/gRPC service.
- `cargo run -p bioscale-evolution-cli -- --help` to generate OTA manifests and research manifests.[web:12]
