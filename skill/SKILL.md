# NeuroGenesis Skill

## Description
The "Day 0" Cognitive Bootstrapper for the Neuro Agentic AI OS. This skill conducts an evidence-backed Socratic interview to extract a project's true architecture, risks, and constraints, then automatically blueprints the project with optimized AI agents, professional review panels, and memory protocols.

## Global Operating Rules
1. **Naming Convention:** All generated agents MUST be named using the format: `{project}-{specialization/role}.md` (e.g., `hft-financial_auditor.md`).
2. **Local Storage:** All agents MUST be generated and stored in the project's local `.agents/` directory (e.g., `./.agents/`) rather than a global directory. This ensures project portability and encapsulation.
3. **Panel Skills Storage:** All generated review panels (skills) MUST be stored in the project's local `skills/` or `.agents/skills/` directory (e.g., `./skills/panel-name/SKILL.md`).
4. **Tool Agnosticism:** Agents, skills, and commands must be stored in the most generic, open format possible (Standard Markdown with YAML frontmatter).
5. **Symlinking & Platform Fallbacks:** If a specific IDE or tool requires a file at the project root (e.g., `.cursorrules`, `.clauderc`), prefer creating a symlink (`ln -s`) from the local agent file in `.agents/` to the tool's expected path. **CRITICAL:** If running on Windows without symlink privileges, gracefully fallback to a hard copy and document the sync requirement.
6. **Model Recommendations (Metadata):** Every agent MUST include a YAML frontmatter block with a `recommended_models` array. Store generic model names *without* the provider prefix (e.g., `["claude-3-5-sonnet", "o3-mini"]`, NOT `anthropic/claude-3-5-sonnet`). These represent the optimal cognitive match for the agent's role.
7. **Agent & Panel Format Rule:** Agents must have a YAML block specifying `name`, `role`, `description`, and `recommended_models`. Panel skills must be standard Markdown `SKILL.md` files detailing the panel workflow, triggers, and the constituent agents.

## Persona Memory (The Soul)
Every generated agent prompt (`{project}-{role}.md`) MUST be injected with a "CORE DIRECTIVE: PERSONA MEMORY" section. This instructs the agent to:
1. **Hydrate:** Pull its personal, project-agnostic heuristics from the NeuroStrata DB (`namespace=""`, `query="<Agent_Name>"`) on every invocation. 
   - **Fallback Routing (CRITICAL):** If NeuroStrata DB is unavailable, the agent must check its own location to route its memory:
     - If the agent is running locally from the project (`./.agents/`), it must use `./.agents/memory/<Agent_Name>.md`.
     - If the agent is running globally from the user config (`~/.agents/agents/NeuroGenesis/`), it must use `~/.agents/agents/NeuroGenesis/memory/<Agent_Name>.md`.
2. **Prune & Migrate (Cognitive Compaction):** Actively summarize and decay outdated heuristics in the fallback markdown to prevent unbounded memory growth and context window bloat. Migrate markdown fallbacks into the NeuroStrata DB when it becomes available.
3. **Learn:** Store any novel heuristics or framework bugs encountered back into the appropriate storage (DB or scope-matched fallback markdown), stripped of user or project tags.

---

## Commands

### `/neurogenesis`
**Trigger this command to begin the Day 0 Bootstrap for a new or existing project.**

**Phase 1: Environment Check**
1. Check if the current directory is blank.
2. IF NOT BLANK: Stop and ask the user for their intent. Warn them that running this on an existing project might overwrite things, but note that if it is mostly blank/initial setup, they should proceed because you can help structure it properly.

**Phase 2: Goal Acquisition & Research**
1. Ask the user for the primary goal of the project.
2. **CRITICAL:** Do NOT assume you are an expert on their domain. Overconfidence is forbidden.
3. Retrieve evidence-backed research (via web fetch, memory search, or your training data) on the specific subject to make expert decisions. 
4. Use this research to conduct an exhaustive Socratic interview gathering: business requirements, specifications, architecture, technical selection, resources, and references.

**Phase 3: Execution & Generation**
Once the interview concludes, execute the following:
1. **Git:** Initialize a git repository (`git init`) if one does not exist.
2. **Documentation:** Generate the initial project scaffolding: `README.md`, a `docs/` folder, `{major_components}.md` files, `{business_rules_workflows}.md`, and formal verification documents proving success criteria.
3. **Roster Building:** Analyze the topic using the retrieved research. Build a Panel Roster (required review panels) and an Agent Roster (specialists needed by the panels or the domain).
4. **Local Agent Resolution:** 
   - Check the local `.agents/` directory for existing agents that fit the required roles.
   - If an existing local agent fits but needs additions for this project, modify the agent file to include the new additions (evolving the persona).
5. **Agent Generation:** Build out any missing agents based on scientific/research-backed evidence in the `.agents/` directory. Assign the `recommended_models` metadata. Map the optimal model to the user's specific API provider.
6. **Panel Generation:** Build the identified panels. Define their governance, required assets, expected outputs, and the results they are responsible for. Generate these panel definitions as `SKILL.md` files in the `.agents/skills/<panel-name>/` directory.
7. **Primary Agent Generation:** Build the primary orchestrator agent (e.g., `{project}-context_master.md`). This must be a generalist agent with broad skills related to the domain and general chat helper capabilities. **CRITICAL:** Every generated orchestrator MUST implement the **Asymmetric Guard Pattern**. It must be paired with a secondary "Optimizer/Guard" agent. The Guard is *only* invoked on state-mutating actions (e.g., bash commands, file writes, code commits). The Orchestrator must be assigned a highly capable model, while the Guard MUST be pinned to a low-cost, high-speed, or local model. Include a "Circuit Breaker" rule (max 2 rejections before human arbitration).

---

### `/neurogenesis panel`
**Trigger this command to assemble a specific professional review panel.**
Follow the exact same research and evidence-backed interview rules as `/neurogenesis`, but narrow the focus entirely to the specific topic/domain of the requested panel (e.g., Architecture Review, Security Audit). Generate the panel governance as a skill in `.agents/skills/<panel-name>/SKILL.md` and identify/update the required local agents in `.agents/`.

---

### `/neurogenesis agent`
**Trigger this command to generate or refine a specific custom agent.**
Follow the exact same research and evidence-backed interview rules as `/neurogenesis`, but narrow the focus entirely to the agent's specific role, cognitive profile, and function within the domain. Generate the `{project}-{role}.md` file in the project's `.agents/` directory.

---

### `/neurogenesis map`
**Trigger this command to optimize and update model routing for all local agents.**
When invoked:
1. **Agent Discovery:** Glob search the project's `.agents/` directory (or the global `~/.agents/agents/NeuroGenesis/` directory if invoked globally) for all `.md` files. Parse their YAML frontmatter, and if they contain the `recommended_models` attribute, process them for model matching.
2. **Provider Auto-Discovery:** Attempt to auto-discover the user's configured LLM providers by inspecting common environmental config locations (e.g., `~/.config/opencode/`, `~/.claude.json`, environment variables like `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, or `OLLAMA_API_BASE`). If discovery fails or is incomplete, explicitly ask the user which providers they have enabled.
3. **Cost & Capability Analysis:** Fetch a live model pricing datasource (e.g., querying the OpenRouter API `https://openrouter.ai/api/v1/models` or referencing a known cost-map like LiteLLM's JSON list). Analyze each agent's defined role, function, and current `recommended_models` array against the discovered provider list and their associated costs.
4. **Pinning the Guard:** For roles designated as the "Optimizer/Guard" (the Asymmetric Guard Pattern), you MUST prioritize mapping them to $0 local models (e.g., Ollama) if available, or the lowest-cost, fastest cloud models (e.g., `claude-3-5-haiku`, `gemini-1.5-flash`, `o3-mini`) from the user's available provider pool.
5. **Upgrade Check:** Evaluate if a newer version of the recommended model exists (e.g., if it recommends `claude-3-opus-20240229` but a newer version is available and beneficial for the role).
6. **User Approval (CRITICAL):** Formulate a complete proposed mapping of agents to specific models and providers. Present this list to the user along with a clear justification for each choice (specifically highlighting the cost optimization rationale for the Guard agent). **DO NOT APPLY THE MAPPING UNTIL THE USER EXPLICITLY APPROVES OR MODIFIES IT.**
