# NeuroGenesis Skill

## Description
The "Day 0" Cognitive Bootstrapper for the Neuro Agentic AI OS. This skill conducts an evidence-backed Socratic interview to extract a project's true architecture, risks, and constraints, then automatically blueprints the project with optimized AI agents, professional review panels, and memory protocols.

## Global Operating Rules
1. **Naming Convention:** All generated agents MUST be named using the format: `{project}-{specialization/role}.md` (e.g., `hft-financial_auditor.md`).
2. **Local Storage:** All agents MUST be generated and stored in the project's local `.agents/` directory.
3. **Panel Skills Storage:** All generated review panels MUST be stored in `.agents/skills/<panel-name>/SKILL.md`.
4. **Tool Agnosticism:** Agents and skills must be Standard Markdown with YAML frontmatter.
5. **Model Recommendations & Binding:** Every agent MUST include two model-related fields in its YAML frontmatter:
   - `recommended_model`: A comma-separated string representing a prioritized list of the theoretical best provider-agnostic models for this role's cognitive profile (e.g., `"claude-3-5-sonnet, gpt-4o, gemini-1.5-pro"`). The first is the ideal choice, followed by fallbacks.
   - `model`: A string representing the exact executable model from the user's available pool, formatted exactly as it appears in the target provider's registry (e.g., `"github copilot/gpt-4o"` or `"openrouter/anthropic/claude-3.5-sonnet"`). 
6. **Agent Format Rule:** Agents must have a YAML block specifying `name`, `role`, `description`, `recommended_model`, `model`, and a `tools` boolean dictionary (e.g., `read: true`, `bash: true`). **CRITICAL: Using arrays for tools or models will crash the OpenCode host application. ALWAYS use comma-separated strings instead of YAML arrays.**

## Persona Memory (The Soul)
Every generated agent prompt MUST be injected with this EXACT "CORE DIRECTIVE: PERSONA MEMORY" section:
1. **Hydrate (Two-Pass):** 
   - Pass 1 (Persona): Pull project-agnostic heuristics from NeuroStrata DB (`namespace="global"`, `query="<Agent_Name>"`).
   - Pass 2 (Context): Pull project-specific context from NeuroStrata DB (`namespace="<Project_Name>"`, `query="<Agent_Name>"`).
2. **Fallback Routing (CRITICAL):** If DB is unavailable, route memory to `./.agents/memory/<Agent_Name>.md`. Do not execute state-mutating actions blindly based on fallback memory without Guard validation.
3. **Prune & Migrate:** Summarize and decay outdated heuristics. Migrate fallback to DB when available.
4. **Learn:** Store novel heuristics back into the DB stripped of PII.

---

## Commands

### `/neurogenesis`
**Trigger this command to begin the Day 0 Bootstrap for a new or existing project.**

**Phase 1: Environment & Model Pool Acquisition**
1. Check if the current directory is blank.
2. **Provider Selection:** Present the user with the following curated list of major supported model providers and ask them to select their preferred provider(s) for this project: 
   *OpenAI, Anthropic, Google, DeepSeek, Mistral, xAI, Cohere, Groq, Together AI, Hugging Face, Amazon Bedrock, Azure, OpenCode Zen, OpenRouter, GitHub Copilot*.
   **CRITICAL:** You must also include this exact message: *"Don't see your provider? Please visit https://models.dev/, find your provider ID, and type it below."*
3. **Model Pool Selection:** Once the user selects their provider(s), use the `bash` tool (via a python script) to query `https://models.dev/` to find the exact, current model names available for the chosen provider(s). Present this list to the user as a multi-select checklist (using the `question` tool) and ask them to select all the models they currently have access to. This creates the "model pool".
4. **IF BLANK (Greenfield):** Ask the user for the primary goal. Offer a "Fast Lane" (max 3 questions) vs exhaustive interview.
5. **IF NOT BLANK (Brownfield):** 
   - **Index First:** Read a MAXIMUM of 10 `.md` files or 50KB total of root config files (`package.json`, etc.). STRICTLY IGNORE `node_modules`, `vendor`, `.git`. Do NOT read raw source code.
   - Offer the "Fast Lane" vs exhaustive interview.

**Phase 2: Goal Acquisition & Socratic Interview**
Conduct an exhaustive interview gathering business requirements, architecture, technical selection, and security constraints. Do NOT assume you are an expert yet.

**Phase 2.5: Mandatory Domain Distillation (ANTI-LAZINESS PROTOCOL)**
You are FORBIDDEN from generating generic, 1-sentence context blocks. Before generating agents, you MUST:
1. Use `webfetch` or memory to pull 2-3 authoritative sources specific to the user's domain (e.g., OWASP for web, SEC rules for trading, HIPAA for medical, framework-specific best practices).
2. Distill this into a dense, 200-400 token `## DOMAIN HEURISTICS` block. This block MUST contain:
   - 3 specific anti-patterns to avoid.
   - 3 critical edge-cases or failure modes.
   - Concrete security/compliance constraints cited from your research.
*This block will be injected into every generated agent.*

**Phase 3: Execution & Generation**
Execute the following:
1. **Project Scaffolding:** Initialize `git init` if missing.
2. **Roster Building:** Define required agents and review panels.
3. **Agent Generation (The Anti-Laziness Template):** Generate agents in `.agents/`. You MUST inject the `## DOMAIN HEURISTICS` block from Phase 2.5 into EVERY agent. For the YAML frontmatter:
   - Set `recommended_model` to a prioritized comma-separated string of the theoretical ideal provider-agnostic models for this specific role's cognitive profile (e.g. `"claude-3-5-sonnet, gpt-4o"`).
   - Set `model: "provider/model_id"` by selecting the absolute best match strictly from the user's selected *model pool* (from Phase 1). Iterate through the `recommended_model` priority list and pick the highest available. If no match exists, select the closest capable fallback from their pool based on cognitive profile.
4. **Panel Generation:** Build identified panels in `.agents/skills/<panel-name>/SKILL.md`. You are FORBIDDEN from generating stub panels. Every panel SKILL.md MUST include:
   - Trigger conditions.
   - A sequential step-by-step workflow.
   - A strict JSON output contract (e.g., `{"panelVerdict": "...", "blockers": [...]}`).
   - *You must immediately generate full agent markdown files in `.agents/` for EVERY constituent member of the panel (e.g., Security Sentinel, Database Expert). Do NOT assume these agents exist globally on the user's machine. The panel and its members must be 100% self-contained and portable within the project.*
5. **Primary Agent Generation (Asymmetric Guard Pattern):** 
   - Generate `{project}-context_master.md` with full tools as a boolean dict (`bash: true`, `write: true`, etc).
   - Generate `{project}-optimizer_guard.md` strictly limited to `read: true`, `glob: true`, `grep: true` in the tools dict. (Do NOT give the guard `webfetch` or `bash` to prevent exfiltration).
   - **Inverted Whitelist (CRITICAL):** Inject this exact rule into the Orchestrator: *"You MAY execute WITHOUT Guard validation ONLY the following tools: read, glob, grep. ALL other tool invocations (bash, write, edit, task, webfetch) REQUIRE Guard approval via the `task` tool."*
   - **Guard Protocol:** The Guard MUST return strict JSON: `{"verdict": "APPROVED" | "REJECTED" | "NEEDS_HUMAN", "policy_id": "...", "severity": "block" | "warn", "reason": "...", "remediation": "..."}`.
   - **Concrete Circuit Breaker:** The Orchestrator MUST track Guard rejections. Inject this rule: *"On a REJECTED verdict, you must read the state file at `./.agents/state/guard_strikes.json`. If strikes >= 3, write `PENDING_ARBITRATION.md` to the workspace root, safely halt, and ask the user to arbitrate. Writing the strike file is the ONLY write operation exempt from Guard review."*
   - **Code Verification Protocol:** Inject this exact rule: *"Before proposing any commit to the Guard, you MUST execute the project's test suite or linter (e.g., `npm test`, `cargo test`) using the `bash` tool. You must include the test output in your JSON payload to the Guard as proof of verification."*

---

### `/neurogenesis panel`
Trigger to assemble a specific professional review panel. Follow Phase 2.5 research rules and generate a fully-fleshed `SKILL.md` with workflow and JSON contracts in `.agents/skills/<panel-name>/`.

### `/neurogenesis agent`
**Trigger this command to generate a single, custom one-off agent without bootstrapping an entire project.**
1. **Scope Inquiry:** Explicitly ask the user if they want this agent stored **Globally** (available across all projects, saved to `~/.agents/agents/`) or **Locally** (scoped to the current project, saved to `./.agents/`).
   - If Global: Name the file `{role}.md`. Set Pass 2 Memory namespace to `"global"`.
   - If Local: Name the file `{project}-{role}.md`. Set Pass 2 Memory namespace to `"<Project_Name>"`.
2. **Interrogation (Phase 2):** Conduct a targeted Socratic interview to extract the agent's role, cognitive profile, needed tools, and specific constraints.
3. **Domain Distillation (Phase 2.5):** Perform the mandatory webfetch/research to build the `## DOMAIN HEURISTICS` block (anti-patterns, edge cases, cited constraints) specific to this agent's role.
4. **Generation (Phase 3):** Generate the agent file adhering to all Global Operating Rules (YAML frontmatter, Two-Pass Memory, etc.). If the new agent requires state-mutating tools (`bash`, `write`, `edit`), you MUST also generate its paired read-only Guard according to the Asymmetric Guard Pattern rules.

### `/neurogenesis map`
Trigger to re-optimize model routing for existing local agents based on their required cognitive profiles.
1. **Provider Selection:** Present the user with the following curated list of major supported model providers and ask them to select their preferred provider(s) for this project: 
   *OpenAI, Anthropic, Google, DeepSeek, Mistral, xAI, Cohere, Groq, Together AI, Hugging Face, Amazon Bedrock, Azure, OpenCode Zen, OpenRouter, GitHub Copilot*.
   **CRITICAL:** You must also include this exact message: *"Don't see your provider? Please visit https://models.dev/, find your provider ID, and type it below."*
2. **Model Retrieval & Pool Selection:** Once the user selects their provider(s), use the `bash` tool (e.g., via a python script) to query `https://models.dev/` to find the exact, current model names available for the chosen provider(s). Present this list to the user as a multi-select checklist (using the `question` tool) and ask them to select all the models they currently have access to. This creates the "model pool".
3. Scan `.agents/*.md` for the `recommended_model` and the cognitive profile/role of each agent.
4. **Cognitive Profile Matching:** Re-map the `model` property in each agent's YAML frontmatter to the highest priority model available in the user's new *model pool*. Read the agent's comma-separated `recommended_model` string and pick the first available match. If none are present, fallback to general cognitive profiling (creative Orchestrators vs deterministic Guards).
5. Present the cognitively-optimized mapping to the user for approval. DO NOT APPLY until approved.

### `/neurogenesis evolve`
Trigger to patch existing `.agents/` when the codebase structure changes. Use the Index First (50KB limit) rule, then carefully patch diffs to the agents' `DOMAIN HEURISTICS` without destroying their core memory.