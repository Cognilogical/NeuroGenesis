# NeuroGenesis Architecture & Design Document

## 1. Overview
**NeuroGenesis** is the "Day 0" Cognitive Bootstrapper for the Neuro Agentic AI OS. Unlike standard CLI AI assistants that act as generic coding helpers, NeuroGenesis initializes a highly specialized, context-aware AI swarm specifically tailored to the project's risk profile, domain, and technology stack.

Its primary goal is to shift the AI from a "generic helper" to a "Project-Specific Lead Agent" and configure the execution environment to interface securely with the NeuroFabric microkernel.

## 2. Core Pillars & Workflow

### Phase 1: Socratic Interrogation
The entrypoint to NeuroGenesis. Upon invocation (e.g., `/genesis` or `bd create init`), the system acts as a Socratic interviewer to extract:
*   **Domain / Industry:** (e.g., MedTech, FinTech, Indie Game)
*   **Risk Profile:** (Compliance-heavy vs. Startup speed)
*   **Target Audience / Scale:** (Internal tool vs. global consumer app)
*   **Preferred Tech Stack:** (Languages, frameworks, cloud providers)

### Phase 2: The Constitution (`genesis-context.json`)
The extracted context is compiled into an immutable JSON artifact (`genesis-context.json`). This acts as the project's Constitution. All subsequent agents, including the Lead Agent and the Adversarial Panel, will mount this file to ground their decisions.

### Phase 3: Dynamic Swarm Blueprinting
Using the `genesis-context.json`, NeuroGenesis dynamically casts the actor agents needed for the project. Evolving from the static 7-member ARC-7 panel, this swarm is custom-tailored:
*   **Lead Agent:** The core orchestrator holding the ultimate project context.
*   **Adversarial Panel:** Specialized agents tailored to the risk profile (e.g., a `Compliance Auditor` for FinTech, a `Graphics Optimizer` for games).
*   These personas are written as configuration files (e.g., `.neurogenesis/lead_agent.json`, `.neurogenesis/agents/compliance.json`).

### Phase 4: Provider-Agnostic Routing
NeuroGenesis reads global model mappings (similar to ARC-7's `model-mappings.json`) and dynamically assigns the highest-IQ foundation models to critical roles. For example:
*   Lead Agent -> `gemini-1.5-pro` (for massive context)
*   Security Sentinel -> `o1-preview` (for deep adversarial reasoning)

### Phase 5: The "Omni-Bind" (Directory-to-Agent Hijacking)
To solve the "Directory Binding" problem where CLI tools act generically, NeuroGenesis drops configuration files into the project root that "hijack" the IDE or CLI, forcing it to assume the Lead Agent persona:
*   `AGENTS.md` (for OpenCode / deepwiki-rs)
*   `.github/copilot-instructions.md` (for Copilot CLI/IDE)
*   `.cursorrules` / `.windsurfrules` (for Cursor/Windsurf)
*   `.clauderc` (for Anthropic's Claude Code CLI)
*   These files symlink or reference the `.neurogenesis/lead_agent.json` definition.

### Phase 6: The "Lobotomy" & Routing Enforcement
To solve the "Finicky Permissions" problem (interactive prompts deadlocking swarms), NeuroGenesis reconfigures the environment:
1.  **Capability Matrix:** Generates `neurofabric.json`, which dictates the rules of engagement and hardcodes triggers for the Adversarial Panel before dangerous commands are executed.
2.  **Kernel Proxy Injection:** Disables native filesystem/bash tools in the underlying CLI (e.g., stripping `default_api:read` or `default_api:bash`).
3.  **Headless Execution:** Injects custom MCP tools (`nf_read_file`, `nf_execute_bash`) that communicate via Unix Domain Sockets (UDS) to the NeuroFabric Kernel.
4.  **Token Bouncer / HITL:** The NeuroFabric Kernel handles security checks. If a command is dangerous, it triggers a non-blocking Nonce-based Human-in-the-Loop (HITL) push notification instead of a terminal `[Y/n]` prompt.

## 3. Directory Structure

```text
/ (Project Root)
├── genesis-context.json                 # The immutable project Constitution
├── neurofabric.json                     # Kernel routing & capability matrix
├── .neurogenesis/                       # Swarm definitions
│   ├── lead_agent.json                  # Core agent persona
│   └── agents/                          # Adversarial panel members
│       ├── security_sentinel.json
│       ├── product_visionary.json
│       └── ... (dynamic roles)
├── AGENTS.md                            # Omni-Bind for OpenCode
├── .cursorrules                         # Omni-Bind for Cursor
├── .github/
│   └── copilot-instructions.md          # Omni-Bind for Copilot
└── ...
```

## 4. Execution Sequence Diagram

1.  **User** runs `neurogenesis init`.
2.  **NeuroGenesis** interrogates the user.
3.  **NeuroGenesis** writes `genesis-context.json`.
4.  **NeuroGenesis** generates tailored agent personas in `.neurogenesis/`.
5.  **NeuroGenesis** writes `neurofabric.json` for the microkernel.
6.  **NeuroGenesis** drops Omni-Bind files (`.cursorrules`, `AGENTS.md`) into the project root.
7.  **NeuroGenesis** instructs the CLI to reload with the new tools (`nf_read_file`, `nf_execute_bash`) and the Lead Agent persona.
8.  **Project-Specific Lead Agent** wakes up and is ready to work.

## 5. Evolution from ARC-7

While ARC-7 is a static, post-facto architectural review panel (7 fixed roles reviewing an existing doc/repo), NeuroGenesis is a dynamic, "Day 0" bootstrapper. It *uses* the principles of ARC-7 but evolves them significantly:

### Panel Structuring (Dynamic Cognitive Diversity)
In ARC-7, the panel is static (Context Master, Architect, Security Sentinel, Product Visionary, Creative Strategist, Optimizer, Naysayer). NeuroGenesis evolves this by dynamically generating the panel *based on the risk profile and domain* captured in `genesis-context.json`.
*   A Fintech app will generate a heavy compliance and security panel.
*   A gaming app will generate graphics, performance, and creative agents.
*   The orchestrator pattern remains, but the actors are cast on the fly, injecting specific context into their system prompts dynamically rather than relying on static `.md` files.

### Provider-Agnostic Model Mapping (Abstracted Capabilities)
ARC-7 utilized a `model-mappings.json` to map abstract models (e.g., `recommended_model: claude-3.5-sonnet`) to the underlying execution tool's specific configuration. NeuroGenesis takes this further by treating foundation models as *Capability Pools*.
*   Agents are defined with required "Capabilities" (e.g., `capability: deep-reasoning`, `capability: massive-context`, `capability: fast-execution`).
*   The `neurofabric.json` manifest handles the routing, querying the user's available provider configuration (OpenAI, Anthropic, local Ollama) to assign the best available model that satisfies the capability requirement, decoupling the persona from the specific model string. This prevents the swarm from breaking when a new model is released or an API key expires.
