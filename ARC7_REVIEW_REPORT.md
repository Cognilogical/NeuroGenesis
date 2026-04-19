# ARC-7 Panel Review Report — NeuroGenesis Architecture

**Subject:** NeuroGenesis Architecture & Bootstrapper Plan
**Date:** 2026-04-19
**Mode:** Document Review (Design Files)
**Panel:** Context Master, The Architect, Product Visionary, Creative Strategist, The Optimizer, The Naysayer (Security Sentinel execution paused/deferred)

---

## Final Recommendation: Request Changes (Prototype Spike Required)

The NeuroGenesis architecture is exceptionally well-conceived, scientifically grounded, and introduces a genuinely novel dual-enforcement model (Soft Lock + Hard Lock). However, the panel fundamentally agrees that several critical mechanisms are currently "hand-waved" and pose existential risks to the system's viability if implemented blindly. 

Specifically: The cryptographic receipt mechanism is unspecified, Semantic Trigger detection is an unsolved problem, Omni-Bind symlinks are fragile, and token costs for continuous multi-agent panels will explode without strict budget controls.

**The panel formally recommends a Prototype Spike** (a vertical slice) to prove the Lobotomy and Hard Lock mechanisms work against a real CLI tool before expanding the design further.

---

## Findings Summary

| Severity | Count |
|----------|-------|
| Critical | 4     |
| Major    | 15    |
| Minor    | 3     |
| Info     | 3     |

---

## Critical Issues (Must Address)

### RISK-001: Semantic Trigger Detection is an Unsolved Problem
- **Severity:** Critical
- **Source:** The Naysayer, The Architect
- **Description:** The entire autonomous invocation system hinges on reliably detecting when the Lead Agent is "modifying authentication logic." The Soft Lock relies on the LLM self-policing (prone to instruction drift). The Hard Lock relies on the proxy pattern-matching file paths (e.g., blocking writes to `src/auth.ts`). If auth logic lives in `src/middleware/session.ts`, the proxy misses it. If detection fails, the entire enforcement layer is bypassed silently.
- **Recommendation:** Do not rely solely on LLM semantic understanding or brittle regex paths. Define a two-tier system: (1) Fast path glob/regex, and (2) a lightweight classifier that reviews the diff *intent* before write operations.

### RISK-002: Cryptographic Receipt Mechanism is Completely Unspecified
- **Severity:** Critical
- **Source:** The Naysayer, The Architect, Creative Strategist
- **Description:** The claim that the proxy demands a "cryptographically verified approval receipt" to prevent hallucinated approvals is the core security mechanism, but has zero specification. Who signs it? With what key? How does the proxy verify it came from an actual agent execution and not the Lead Agent crafting a fake JSON blob?
- **Recommendation:** Design a concrete receipt protocol. Minimum viable: The NeuroFabric kernel generates a nonce before dispatching to the specialist; the specialist response includes the nonce; the kernel validates it. Define the `agent_message_envelope.json` schema.

### PERF-001: Missing end-to-end latency/cost budget per workflow
- **Severity:** Critical
- **Source:** The Optimizer
- **Description:** The architecture describes many sequential phases but does not define an explicit token/cost/latency budget. A 4-agent panel running on frontier models can burn hundreds of thousands of tokens per invocation. Without bounds, Day 0 bootstrap and continuous triggers will become economically unviable for individual developers.
- **Recommendation:** Define a hard execution budget contract: max tokens per phase, max retries, max wall-clock per stage, and total session budget. Implement a `/metrics` endpoint to track cumulative spend.

### CREAT-001: Omni-Bind may create brittle single-entry UX
- **Severity:** Critical
- **Source:** Creative Strategist, The Architect
- **Description:** The design assumes all assistants will reliably obey injected files like `.cursorrules`. In practice, different tools parse these differently (Markdown vs JSON vs TOML). Symlinking `.cursorrules` to `.neurogenesis/lead_agent.json` will break if the tool doesn't support JSON. 
- **Recommendation:** Use a template-and-render approach instead of raw symlinks. Have `neurogenesis bind` generate format-specific files with a header comment indicating they are auto-generated from the master Constitution.

---

## Major Issues (Should Address)

### ARCH-001: The "Immutable Constitution" is Not Actually Immutable
- **Severity:** Major
- **Source:** The Architect, The Naysayer
- **Description:** `genesis-context.json` is described as immutable, but projects pivot. If it's truly frozen, the swarm roster becomes stale. If a user just edits the file manually, the downstream agents might break.
- **Recommendation:** Add a `schema_version` field and define a formal `genesis amend` ceremony: re-invoke the Socratic Inquisitor scoped to the changed dimension, and force a Pre-Flight Architecture Panel review of the delta.

### PROD-004: Proxy enforcement is high-effort and may reduce activation
- **Severity:** Major
- **Source:** Product Visionary, Creative Strategist
- **Description:** Immediate hard locks can feel punitive during onboarding and cause users to abandon the tool.
- **Recommendation:** Stage enforcement: start with "Observe -> Warn -> Enforce" modes. Only escalate to "Hard Locks" for high-risk actions once the user trusts the system.

### ARCH-003: No degraded-mode or fallback when NeuroFabric proxy is unavailable
- **Severity:** Major
- **Source:** The Architect
- **Description:** If the NeuroFabric proxy crashes, the agent is completely lobotomized with zero capability (no native tools). This is a single point of failure that bricks the developer's environment.
- **Recommendation:** Define operating modes: Full Enforcement, Soft-Only (proxy down, native tools restored, audit logged), and Emergency Bypass.

### RISK-004: Lobotomy Strategy Assumes Controllable CLI Tool Internals
- **Severity:** Major
- **Source:** The Naysayer
- **Description:** Disabling native CLI tools assumes tools like Cursor, Claude Code, and OpenCode expose stable APIs for capability stripping. Many do not.
- **Recommendation:** Audit target CLI tools' capability-stripping APIs *today*. Build a vertical slice prototype to prove the Lobotomy works against at least one real tool before expanding the design.

---

## What Was Done Well

*   **Dual-Enforcement Model:** The Soft Lock (prompt) + Hard Lock (kernel proxy) is genuinely novel and addresses the critical LLM instruction-drift problem that plagues every other framework.
*   **Separation of Concerns in Memory:** Separating "what to remember" from "how to remember" (tool syntax) is excellent decoupling that prevents conflicting tool instructions.
*   **Scientific Grounding:** The explicit citations of Du et al., Sharma et al., and Li et al. prove this is built on actual cognitive science research, not cargo-culting.
*   **Socratic Inquisitor "Bucket Filling":** The approach of opening new conceptual dimensions rather than rejecting inputs is a sophisticated interview technique.
*   **Anti-Simulation Rule:** Forbidding the Lead Agent from hallucinating specialist responses addresses a real and dangerous LLM failure mode.

---

## Action Items

- [ ] **Prototype Spike:** Build a vertical slice (1 CLI tool, 1 panel, 1 trigger) to prove the Lobotomy and Hard Lock receipt mechanism work in practice.
- [ ] **Define the Envelope:** Draft the JSON schema for the cryptographic receipt / agent communication protocol.
- [ ] **Revise Omni-Bind:** Update DESIGN.md to use a template-and-render generation approach rather than fragile symlinks.
- [ ] **Add Cost Controls:** Add token/cost budget configurations to the Panel schemas.
- [ ] **Design `genesis amend`:** Define the workflow for safely updating the `genesis-context.json` Constitution.

---
*Generated by ARC-7 Panel · 2026-04-19*