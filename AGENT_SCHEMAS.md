# Neuro OS: Agent & Panel Schemas (Autonomous Invocation)

This document defines the underlying JSON/YAML templates used to construct individual agents and entire review panels.

A critical evolution from ARC-7 is **Autonomous Invocation**. Instead of relying on a human to manually type `/ARC-7` to summon a panel, the Neuro OS embeds **Triggers** directly into the Lead Agent's environment and the NeuroFabric proxy. When specific conditions are met (e.g., modifying an authentication module, finalizing a database schema), the Lead Agent is *forced* by the system rules to invoke a specialist or convene a full panel before proceeding.

---

## 1. The Agent Template (`agent_schema.json`)

Every specialized persona is defined by a strict JSON template. This ensures they are machine-readable and map perfectly to the NeuroFabric Capability Matrix.

```json
{
  "name": "Security Sentinel",
  "domain": "Cybersecurity & Threat Modeling",
  "capability_required": "security-analysis",
  "cognitive_profile": "strict-adversarial",
  "academic_grounding": ["OWASP Top 10", "STRIDE", "Zero-Trust Architecture"],
  "system_prompt": "You are the Security Sentinel. Your mandate is...",
  "output_schema": {
    "format": "json",
    "strict": true,
    "fields": ["severity", "finding", "owasp_category", "remediation"]
  },
  "autonomous_triggers": {
    "specialist_invocation": [
      {
        "condition": "When proposing, reviewing, or modifying authentication, authorization, or cryptographic logic.",
        "action": "require_specialist_review",
        "blocking": true,
        "rationale": "Security must vet trust boundaries before code generation."
      },
      {
        "condition": "When configuring network gateways, firewalls, or CORS policies.",
        "action": "require_specialist_review",
        "blocking": true,
        "rationale": "Infrastructure exposure requires adversarial threat modeling."
      }
    ],
    "memory_retention": [
      {
        "condition": "Upon identifying a novel framework bug, undocumented vulnerability pattern, or novel remediation technique.",
        "action": "commit_to_personal_memory",
        "categorization": {
          "scope": "global_persona",
          "memory_type": "heuristic_rule"
        },
        "rationale": "Allows the agent to evolve a persistent, project-independent knowledge base (The 'Soul') rather than relying on static prompt files."
      },
      {
        "condition": "When explicitly corrected by a human expert on a domain-specific engineering fact or methodology.",
        "action": "commit_to_personal_memory",
        "categorization": {
          "scope": "global_persona",
          "memory_type": "preference_or_correction"
        },
        "rationale": "Prevents the agent from repeating the same heuristic failure across future projects."
      }
    ]
  }
}
```

### 1.1 Trigger Anatomy
*   **`condition`**: The specific semantic trigger injected into the Lead Agent's prompt or monitored by the proxy.
*   **`action`**: What the Lead Agent must do (e.g., `require_specialist_review`).
*   **`blocking`**: If `true`, the Lead Agent cannot execute the change (via bash or file write) until it receives a positive JSON response from the specialist.
*   **`rationale`**: Explains *why* the trigger exists, ensuring the LLM understands the context of the rule.

---

## 2. The Panel Template (`panel_schema.json`)

A **Panel** is a pre-defined, domain-specific ensemble of agents convened for broad, high-stakes decisions. The orchestrator (Lead Agent) uses the `Task` tool (or equivalent MAS protocol) to spin up the panel members simultaneously.

```json
{
  "name": "Pre-Flight Architecture Panel",
  "description": "A comprehensive review panel convened before major structural codebase changes or new feature deployments.",
  "members": [
    "Systems Architect",
    "Security Sentinel",
    "The Naysayer",
    "SRE"
  ],
  "voting_rules": {
    "consensus_required": false,
    "veto_power": ["Security Sentinel"],
    "tie_breaker": "Lead Agent"
  },
  "autonomous_triggers": {
    "panel_invocation": [
      {
        "condition": "When finalizing a new database schema or state management architecture.",
        "action": "convene_panel",
        "blocking": true,
        "rationale": "Data layer changes are notoriously difficult to reverse and require multi-disciplinary consensus."
      },
      {
        "condition": "Prior to executing a deployment script to a staging or production environment.",
        "action": "convene_panel",
        "blocking": true,
        "rationale": "Final sanity check for security, performance regressions, and failure modes."
      }
    ]
  }
}
```

### 2.1 Panel Anatomy
*   **`members`**: The specific agents pulled from the Standard Roster.
*   **`voting_rules`**: Formalizes conflict resolution. If the `Security Sentinel` flags a "Critical" issue, their veto power stops the execution, overriding the rest of the panel.
*   **`autonomous_triggers`**: High-stakes conditions that demand cognitive diversity over speed.

---

## 3. How the Rules are Enforced

To prevent the Lead Agent from "forgetting" or ignoring these triggers (a common problem with LLM instruction drift), the enforcement happens at two layers:

### Layer A: Prompt Injection (The Soft Lock)
NeuroGenesis compiles all the `autonomous_triggers` from the selected agents/panels and injects them directly into the Lead Agent's `.neurogenesis/lead_agent.json` prompt as an immutable rule block:
> *"RULE: You must pause and invoke the Security Sentinel via the Task tool whenever you modify authentication logic."*

**For Memory Rules (The Soul):** The system injects a directive explaining *what* and *when* to log personal revelations, categorized appropriately. It deliberately omits *how* to save it, deferring to the memory module's injected tool instructions to prevent conflicting syntax rules. For example:
> *"RULE: If you encounter an undocumented framework bug or are explicitly corrected on a methodology, you MUST use your provided memory tools to permanently record this learning. Categorize it as a 'global persona rule' so it applies to all future projects. (Refer to your memory system instructions for the specific tool and syntax)."*

### Layer B: NeuroFabric Proxy (The Hard Lock & Cryptographic Receipt)
For maximum safety, the triggers can be registered with the NeuroFabric microkernel proxy (in `neurofabric.json`). If the Lead Agent tries to use the `write_file` tool on `src/auth.ts`, the Kernel intercepts it:
> *"PROXY REJECT: Write operation blocked. Condition 'auth_modification' triggered. You must present a cryptographically verified approval receipt from the Security Sentinel agent process before this write is permitted."*

Because the kernel demands a system-verified receipt (e.g., an authentic Task ID or signed JSON output from the actual execution of the specialist agent), **the Lead Agent is physically incapable of "hallucinating" or simulating the panel's response.** It cannot just write "The Security Sentinel said it is okay." The proxy enforces actual, independent execution of the required agents.

This combination guarantees that expert reviews and panels are automatically summoned when needed, bridging the gap between an AI assistant and a true, self-governing Agentic OS.