# NeuroGenesis: Remaining Architectural Blueprints

This document outlines the three remaining foundational pieces of the NeuroGenesis bootstrapper before implementation begins. It pays special attention to the Tracker-Agnostic Task API to prevent the Neuro OS from becoming rigidly coupled to any specific issue tracker (like BeadBoard).

---

## Proposal 1: The Prompt Compilation Engine (The "Alchemist")
*Problem:* How do we reliably assemble JSON schemas, constraints, memory rules, and risk profiles into a robust system prompt for the Lead Agent without losing context or truncating critical invariants?

**Design:**
The engine uses a deterministic, bottom-heavy compilation strategy based on **LLM Recency Bias** (LLMs pay the most attention to the absolute beginning and the absolute end of their context window), combined with a strict **Token Budget Allocator**.

*   **Token Budget Allocator:** Before compilation, the engine calculates the total available context window. It reserves fixed percentages for the Header (10%), Footer (20% for invariants), and truncates or summarizes the Body (Project Context/Roster) if it exceeds the remaining 70% budget. This prevents "lost in the middle" truncation of critical rules.
*   **Header (The Persona):** The agent's base template (e.g., "You are the Context Master...").
*   **Body A (The Constitution):** The injected `genesis-context.json` (Tech Stack, Risk Profile, Target Audience).
*   **Body B (The Roster):** Who else exists in this project's swarm (e.g., Security Sentinel, SRE).
*   **Footer A (Memory Instructions):** The "Soul" rules (when to save learnings, how to categorize them).
*   **Footer B (Rules of Engagement):** The Soft Lock triggers compiled from the selected panels (e.g., "RULE: If touching auth, you MUST invoke Security Sentinel").
*   **Absolute Bottom (Global Invariants):** Strict Anti-Sycophancy, Epistemic Humility, and Anti-Simulation rules. (Because these are at the very end, the model cannot ignore them).

---

## Proposal 2: The Tool Distribution Matrix (Principle of Least Privilege)
*Problem:* If every agent gets `bash` and `write_file`, the security model collapses. We must distribute MCP tools securely and enforce strict proxy-level restrictions.

**Design:**
Every tool invocation routes through the NeuroFabric Proxy, which enforces **Strict Command Allowlists** mathematically defined per role. Agents cannot bypass these allowlists.

*   **The Lead Agent (Context Master):** 
    *   *Gets:* `nf_write_file` (gated by proxy), `nf_execute_bash` (gated by proxy), `Task` (to spawn subagents), `neurostrata_add_memory`, `nf_create_issue`.
    *   *Proxy Allowlist (Bash):* `git`, `npm`, `tsc`, `pytest`, `ls`, `grep` (only safe project commands).
    *   *Restricted:* Cannot run arbitrary system commands (`rm -rf`, `curl`, `wget`). Cannot read CVE databases or perform isolated math verification natively.
*   **Security Sentinel:** 
    *   *Gets:* `nf_read_file`, `nf_search_code`, `webfetch` (to check live CVE databases/OWASP), `neurostrata_add_memory`.
    *   *Restricted:* NO `nf_write_file`. NO `nf_execute_bash`. (It is an auditor, not an executor).
*   **Financial Auditor:** 
    *   *Gets:* `nf_read_file`, `nf_execute_python` (isolated sandbox for verifying double-entry math, avoiding LLM floating-point errors).
    *   *Restricted:* NO `nf_write_file`. NO `nf_execute_bash`.
*   **The Naysayer / QA:**
    *   *Gets:* `nf_read_file`, `nf_execute_bash` (strictly limited to read-only profiling tools, e.g., `curl`, `ab`, `pytest`).
    *   *Proxy Allowlist (Bash):* `curl`, `ab`, `pytest`, `wrk`. No write access to filesystem via bash.

---

## Proposal 3: The Tracker-Agnostic Task API (Day 1 Handoff)
*Problem:* We want NeuroGenesis to drop the initial 3-5 bootstrap tasks (e.g., "Setup DB Schema", "Initialize Auth Gateway") into the user's issue tracker. Currently, the environment relies heavily on BeadBoard (`bd`), but we *cannot* hardcode `bd` into the OS. If a user wants to use Jira, GitHub Issues, or Linear (or "Paperclip"), swapping the tracker must not break the OS.

**Design:**
We implement a **Standard MCP Task Interface** backed by **Adapter Shell Scripts**, completely blinding the LLM to the underlying tracker.

1.  **The LLM Interface (MCP Tools):** 
    The Lead Agent is only given generic OS tools: `nf_issue_create`, `nf_issue_update`, `nf_issue_list`. It never knows the word "BeadBoard" or "Jira".
    *Payload:* `{"title": "Setup DB", "description": "...", "priority": "high"}`
4.  **The Adapter Layer (NeuroFabric Proxy):**
    When the Kernel intercepts `nf_issue_create`, it delegates the execution to a configurable shell script located in `.neurofabric/adapters/issue_create.sh`.
    **CRITICAL SECURITY ENFORCEMENT:** Passing data via CLI arguments or environment variables creates a severe command injection vulnerability. The proxy MUST pipe the raw JSON payload strictly via `stdin`, forcing the shell script to parse it safely.
5.  **The Default Implementation:**
    Out of the box, `issue_create.sh` is a wrapper around `bd` that reads JSON from `stdin` using `jq`:
    ```bash
    #!/bin/bash
    # Translates standard OS JSON from stdin to BeadBoard CLI safely
    PAYLOAD=$(cat -)
    TITLE=$(echo "$PAYLOAD" | jq -r '.title // empty')
    DESC=$(echo "$PAYLOAD" | jq -r '.description // empty')
    PRIORITY=$(echo "$PAYLOAD" | jq -r '.priority // "medium"')
    
    # Execute BeadBoard using the parsed variables
    bd create "$TITLE" -d "$DESC" -p "$PRIORITY"
    ```
6.  **The Swap (Pluggability):**
    If a user uninstalls BeadBoard and wants to use GitHub Issues, they simply swap the adapter script to parse the identical `stdin` JSON into `gh` format:
    ```bash
    #!/bin/bash
    PAYLOAD=$(cat -)
    TITLE=$(echo "$PAYLOAD" | jq -r '.title // empty')
    DESC=$(echo "$PAYLOAD" | jq -r '.description // empty')
    
    gh issue create --title "$TITLE" --body "$DESC"
    ```

**Result:** The entire OS multi-agent swarm continues functioning perfectly. No prompts need to change. No agents need to be retrained on new CLI syntax. The OS interacts with a standardized "Tracker Interface," and the cheap, dumb bash scripts safely handle the translation via `stdin`.