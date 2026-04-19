# NeuroGenesis Architecture & Design Document

## 1. Overview & Elevator Pitch

**"The `create-react-app` for the Neuro Agentic AI OS."**

**NeuroGenesis** is the "Day 0" Cognitive Bootstrapper. Just as `create-react-app` or `npm create vite` scaffolds a ready-to-code frontend project, NeuroGenesis scaffolds a **ready-to-think AI swarm** embedded directly into your new codebase.

Unlike standard CLI AI assistants that act as generic coding helpers, NeuroGenesis initializes a highly specialized, context-aware AI team specifically tailored to the project's risk profile, domain, and technology stack. 

Its primary goal is to shift the AI from a "generic helper" to a "Project-Specific Lead Agent", scaffold the initial directory structure, and configure the execution environment to interface securely with the NeuroFabric microkernel from the very first terminal command.

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

### Phase 7: The "Soul" Injection (NeuroStrata Personal Memory Integration)
Unlike generic static prompts or projects that rely on crude, bloated `Soul.md` files that clobber the context window, Neuro OS treats its agents as continuously evolving entities backed by a shared knowledge graph.
When an agent is initialized, it is connected to the OS's memory architecture. NeuroGenesis dictates **what** the agent should remember (e.g., domain-specific corrections, novel framework bugs) and **how to categorize it** (e.g., as a global heuristic vs. a project-specific quirk). 

Crucially, NeuroGenesis strictly avoids telling the agent **how** to save the memory (e.g., it does not hardcode specific tool names like `neurostrata_add_memory` or paths to specific `.md` files). The memory architecture itself injects the "how" (the specific tool syntax). This separation of concerns prevents conflicting instructions, avoids confusing the LLM with duplicate tool definitions, and ensures the agent always follows the latest memory system protocol natively.

As the agent encounters novel solutions or is corrected on domain-specific facts across different projects, it writes those revelations into its personal namespace, actively increasing its competency and evolving its heuristic frameworks over time without needing manual prompt updates.

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

## 4. The NeuroGenesis Bootstrapping Pipeline (Execution Sequence)

The bootstrapping process strictly follows this deterministic pipeline when initializing a new repository:

1. **Invocation:** The user invokes the `/genesis` command (or `neurogenesis init`) in an empty or existing directory.
2. **Project Interview (Socratic Interrogation):** NeuroGenesis acts as an interviewer, extracting the domain, risk profile, target audience, and tech stack preferences.
3. **Initial Knowledge Base Creation:** The extracted context is compiled and written to the immutable `genesis-context.json` (The Constitution).
4. **Project Agent Generation:** The core "Lead Agent" (Context Master) is generated from a base template and deeply contextualized with the project's knowledge base.
5. **Panel Identification:** Based on the risk profile in the knowledge base, NeuroGenesis identifies which Standard Panels are required (e.g., a FinTech app triggers the *Deep Audit & Compliance Board*; a Web3 app triggers heavy *Pre-Flight Architecture*).
6. **Role Resolution:** The selected Panels declare exactly which specialized agent roles they require to function (e.g., Security Sentinel, Financial Auditor).
7. **Agent Generation & Customization:** NeuroGenesis pulls the required roles from the Standard Agent Roster. If a highly specific niche role is required that doesn't exist in the standard roster, it generates a custom agent strictly adhering to the Agent Generation Ruleset.
8. **Rules of Engagement (RoE) Binding:** NeuroGenesis compiles the "Autonomous Triggers" from all generated panels and agents. It hardcodes these RoE into the Lead Agent's prompt (Soft Lock) and the `neurofabric.json` proxy manifest (Hard Lock), dictating exactly when the Lead Agent *must* invoke the specialists and panels.
9. **Omni-Bind & Handoff:** NeuroGenesis drops the environment-hijacking files (`AGENTS.md`, `.cursorrules`), forces a CLI reload, and wakes up the fully configured Lead Agent.

## 6. Agent Generation Rules & The Standard Roster

While NeuroGenesis can dynamically generate custom agents for niche edge cases, it relies on a highly curated, scientifically grounded **Standard Roster** of industry experts available to all projects. 

### Agent Generation Ruleset
To prevent hallucinated or useless personas, every agent generated or included in the standard roster MUST adhere to the following strict rules:

1. **Academic & Empirical Grounding:** The persona's core instructions and heuristic frameworks must be backed by recent peer-reviewed scientific research, established industry standards, or proven cognitive science (e.g., OWASP for Security, Nielsen Norman Group for UX/UI, IEEE papers for architecture, or Multi-Agent Debate frameworks).
2. **Capability Mapping (Not Model Hardcoding):** Agents must define the *type* of intelligence they need (e.g., `capability: divergent-thinking`, `capability: strict-adversarial`, `capability: massive-context-synthesis`), allowing the OS to route to the best available model.
3. **Cognitive Profiling:** The system prompt must explicitly define the agent's cognitive bias: Are they generative (brainstorming), reductive (optimizing/cutting scope), or adversarial (finding flaws)?
4. **Strict Output Schemas:** Agents must communicate in parseable formats (JSON or strict Markdown tables) to allow the Context Master (Lead Agent) to synthesize their outputs without human intervention.
5. **Zero-Tolerance for Factual Inaccuracy (Strict Anti-Sycophancy):** Neuro OS is an execution engine, not a chatbot or an entertainment device. Every agent MUST explicitly and bluntly correct the user if they propose architectures, constraints, or concepts that violate mathematics, physics, or established computer science facts. Agents are instructed to prioritize engineering truth over politeness.
6. **Mandatory Epistemic Humility (Zero Unverified Assertions):** Agents must NEVER state a claim about the environment, system state, dependencies, or data as fact unless they have explicitly verified it using their available tools (e.g., reading files, querying APIs, executing bash checks). Regardless of the model's internal confidence—which is dangerously inflated in frontier models—if a fact is not directly observed in the current session, the agent must explicitly state, "I need to verify this," or "I cannot confirm this without checking." Assumptions cause catastrophic cascading failures in complex workflows. We are executing enterprise-grade operations—whether in software engineering, financial accounting, data mining, or business strategy—not playing with toys.
7. **Strict Anti-Simulation (No Hallucinated Personas):** The orchestrating agent (e.g., the Lead Agent) is strictly forbidden from simulating, roleplaying, or hallucinating the responses of other agents or panels. If a rule requires the input of the Security Sentinel or a full panel, the Lead Agent MUST physically invoke those distinct agents via the system's designated multi-agent tooling (e.g., the `Task` tool). The Lead Agent cannot simply generate text claiming, "The Security Sentinel has reviewed this and approves." True, independent execution of the specialist agent is a non-negotiable mandatory requirement.

### The Core Standard Roster (The Baseline Swarm)
We will define a focused, high-impact set of standard experts. We do not want to go wild; we want a tight, elite team:

*   **The Architect (Systems & Domain Boundaries):** Grounded in Domain-Driven Design (DDD) and microservice/monolith trade-off research.
*   **The Security Sentinel (Adversarial Threat Modeling):** Grounded strictly in OWASP Top 10, STRIDE threat modeling, and formal verification concepts.
*   **The UX/UI Designer (Human-Computer Interaction):** Grounded in cognitive load theory, Fitts's Law, accessibility standards (WCAG), and modern interaction design research.
*   **The Product/Marketing Visionary (Value & ROI):** Grounded in Lean Startup methodology, cohort analysis, and product-market fit metrics.
*   **The Naysayer / QA (Risk & Failure Modes):** Grounded in Chaos Engineering, edge-case discovery, and system resilience research.
