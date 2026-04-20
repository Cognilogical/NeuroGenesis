# NeuroGenesis Skill

## Description
The "Day 0" Cognitive Bootstrapper for the Neuro Agentic AI OS. This skill conducts an evidence-backed Socratic interview to extract a project's true architecture, risks, and constraints, then automatically blueprints the project with optimized AI agents, professional review panels, and memory protocols.

## Global Operating Rules
1. **Naming Convention:** All generated agents MUST be named using the format: `neuro-{project}-{specialization/role}.md` (e.g., `neuro-hft-financial_auditor.md`).
2. **Global Storage:** All agents MUST be stored in the user-level directory (e.g., `~/.config/neurogenesis/agents/`) so they are available globally across other projects.
3. **Tool Agnosticism:** Agents, skills, and commands must be stored in the most generic, open format possible (Standard Markdown with YAML frontmatter).
4. **Symlinking:** If a specific IDE or tool requires a local file (e.g., `.cursorrules`, `.clauderc`), do NOT duplicate the agent. Create a symlink (`ln -s`) from the global user-level agent file to the tool's expected local path.
5. **Model Recommendations (Metadata):** Every agent MUST include a YAML frontmatter block with a `recommended_models` array. Store generic model names *without* the provider prefix (e.g., `["claude-3-5-sonnet", "o3-mini"]`, NOT `anthropic/claude-3-5-sonnet`). These represent the optimal cognitive match for the agent's role.

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
4. **Global Agent Resolution:** 
   - Check the global `~/.config/neurogenesis/agents/` directory for existing `neuro-` agents that fit the required roles.
   - If an existing global agent fits but needs additions for this project, modify the global agent file to include the new additions (evolving the persona).
5. **Agent Generation:** Build out any missing agents based on scientific/research-backed evidence. Assign the `recommended_models` metadata. Map the optimal model to the user's specific API provider.
6. **Panel Generation:** Build the identified panels. Define their governance, required assets, expected outputs, and the results they are responsible for.
7. **Primary Agent Generation:** Build the primary orchestrator agent (e.g., `neuro-{project}-context_master.md`). This must be a generalist agent with broad skills related to the domain and general chat helper capabilities.

---

### `/neurogenesis panel`
**Trigger this command to assemble a specific professional review panel.**
Follow the exact same research and evidence-backed interview rules as `/neurogenesis`, but narrow the focus entirely to the specific topic/domain of the requested panel (e.g., Architecture Review, Security Audit). Generate the panel governance and identify/update the required global agents.

---

### `/neurogenesis agent`
**Trigger this command to generate or refine a specific custom agent.**
Follow the exact same research and evidence-backed interview rules as `/neurogenesis`, but narrow the focus entirely to the agent's specific role, cognitive profile, and function within the domain. Generate the `neuro-{project}-{role}.md` file globally.

---

### `/neurogenesis map`
**Trigger this command to optimize and update model routing for all global agents.**
When invoked:
1. Glob search the user-level directory for all agents matching `neuro-*.md`.
2. Analyze each agent's defined role, function, and current `recommended_models` array.
3. **Upgrade Check:** Evaluate if a newer version of the recommended model exists (e.g., if it recommends `claude-3-opus-20240229` but a newer version is available and beneficial for the role). Update the YAML metadata to the newer model if it improves performance.
4. **Provider Mapping:** Ask the user what API provider(s) they are currently using. Find the best fit from their provider's catalog that matches the generic `recommended_models` and map/configure their local environment to use it.
