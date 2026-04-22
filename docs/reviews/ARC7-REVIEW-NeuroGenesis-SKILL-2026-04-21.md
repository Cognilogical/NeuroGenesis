# ARC-7 Architecture Review: NeuroGenesis SKILL.md Architecture

**Date:** 2026-04-21
**Mode:** Document Review
**Topic:** NeuroGenesis SKILL.md Architecture
**Context Master:** Gemini 3.1 Pro (Preview)

## 1. Executive Summary
The NeuroGenesis SKILL.md architecture provides a flexible, text-based mechanism for extending agent capabilities. The panel synthesis indicates that while the product vision and extensibility are highly promising, there are critical optimization and execution boundaries that must be addressed to ensure safe scaling across multi-agent environments.

## 2. Panel Findings

### Architect
The structural design of the SKILL.md format effectively encapsulates domain-specific instructions and localized tool references. However, this appears to require stricter bounding constraints within the preamble to ensure predictable parsing and context loading across varying agent runtimes.

### Security Sentinel
* **Concern 1 (OWASP A01):** Access control gap regarding script execution paths. Skills might implicitly request execution context outside the intended workspace boundaries if path resolution is not strictly bounded.
* **Concern 2 (OWASP A03):** Input validation gap. Arguments passed via command invocation (e.g., `/skill <args>`) must be strictly sanitized before being interpolated into underlying system operations or sub-agent tasks.

### Optimizer
Dynamic loading of SKILL.md files on every agent invocation introduces measurable I/O latency and context-window bloat. I recommend implementing an aggressive caching layer to store pre-parsed skill schemas and instructions.

### Naysayer
Relying heavily on Markdown for executable contracts is inherently brittle. As skills grow in complexity, embedding tool schemas and execution protocols inside Markdown blocks will likely become unmaintainable and prone to silent parsing errors.

### Product Visionary
This architecture dramatically lowers the barrier to entry for creating new agent capabilities. If the underlying schema is standardized correctly, it unlocks a massive community ecosystem of plug-and-play skills.

### Creative Strategist
We should consider allowing SKILL.md files to declare semantic dependencies on other skills or project memories, creating a rich, composable knowledge graph rather than isolated command utilities.

## 3. Conflict Resolution
**Conflict:** Markdown brittleness (Naysayer) vs. Low barrier to authoring (Product Visionary).
**Resolution:** Maintain Markdown as the primary human-authoring format to preserve accessibility, but introduce a strict validation step that compiles SKILL.md into a rigid JSON schema internally before it can be registered or executed by the agent runtime.

## 4. Final Decision & Action Plan

The architecture is approved with mandatory conditions for compilation and boundary enforcement. 

**Model Assignments:**
* **Task:** Implement the strict SKILL.md parser, compiler, and caching layer.
  * **Assigned Model:** GPT-5.3-Codex
  * **Rationale:** Best suited for pure coding tasks involving parsers and I/O optimization.
* **Task:** Design and implement the path resolution constraints and input sanitization layer.
  * **Assigned Model:** OpenAI o4 (High)
  * **Rationale:** Required for adversarial testing and mitigating OWASP A01 / A03 concerns.
* **Task:** Define the formal schema mapping and inter-skill dependency contract.
  * **Assigned Model:** Claude Sonnet 4.6
  * **Rationale:** Excels at complex API contracts and integration design for safety-critical systems.