# NeuroGenesis: Architecture Review Package

## Introduction
NeuroGenesis has undergone a radical architectural pivot. It is no longer a compiled Rust CLI binary or a complex Microkernel Proxy (NeuroFabric). It is now implemented purely as a declarative **LLM Skill** (`skill/SKILL.md`) that natively utilizes an intelligent agentic environment (e.g., OpenCode, Claude Code) to scaffold the "Day 0" environment for a project.

## 1. Core Operating Principles
- **No Compiled Binaries:** The bootstrapping process leverages the native reasoning of the LLM rather than brittle shell scripts or string-concatenation logic.
- **Strict Evidence-Based Reasoning:** The agent must NOT assume expertise in a domain. It MUST retrieve external research/evidence to guide the Socratic interview and make architectural choices.
- **Global Tool Agnosticism:** Agents are not stored in proprietary formats (e.g., `.cursorrules`). They are stored as standard Markdown files with YAML frontmatter at the global user level (`~/.config/neurogenesis/agents/`).
- **Symlinking for Local Tooling:** If a specific IDE requires a local file, the system creates a symlink to the global user file, preventing duplication and state drift.

## 2. Agent Naming & Metadata Convention
All agents must adhere to the following naming and metadata rules:
- **Naming Format:** `neuro-{project}-{role}.md` (e.g., `neuro-hft-financial_auditor.md`).
- **Metadata (YAML Frontmatter):** Every agent MUST include a `recommended_models` array containing generic model identifiers *without* provider prefixes (e.g., `claude-3-5-sonnet`, `o3-mini`). This abstracts the cognitive requirements from the API vendor.

## 3. The Commands (Workflow)

### `/neurogenesis` (The Bootstrap)
1. **Environment Check:** Stops if the directory is not blank, asking for user intent.
2. **Goal Acquisition:** Asks for the primary goal, retrieves evidence-backed research, and conducts an exhaustive interview covering business requirements, architecture, and tech selection.
3. **Execution:**
    - Initializes a git repository.
    - Generates project documentation (`README.md`, `docs/`, major components, formal verifications).
    - Analyzes the topic to build a required Panel Roster and Agent Roster.
    - Modifies existing global agents if necessary (evolving the persona).
    - Generates any missing agents with optimal model metadata.
    - Defines panel governance, outputs, and results.
    - Generates the primary orchestrator (generalist chat helper agent).

### `/neurogenesis panel`
A focused version of the bootstrap specifically designed to assemble professional review panels (e.g., Architecture Review Board, Security Audit Panel). It identifies required members, establishes voting mechanics, and executes the `/neurogenesis agent` directive for each member.

### `/neurogenesis agent`
Generates or refines a specific custom agent persona based strictly on evidence-backed research and the project's technical context.

### `/neurogenesis map`
Optimizes model routing for all global agents. It searches the `~/.config/neurogenesis/agents/` directory, analyzes each `neuro-*.md` file, checks for model version bumps (e.g., Opus 4.6 to 4.7), and dynamically maps the generic `recommended_models` array to the optimal model provided by the user's specific API vendor configuration.

## 4. Persona Memory (The Soul)
Every generated agent prompt (`neuro-{project}-{role}.md`) is injected with a "CORE DIRECTIVE: PERSONA MEMORY" section. This instructs the agent to:
1. **Hydrate:** Pull its personal, project-agnostic heuristics from the NeuroStrata DB (`namespace="global"`, `query="<Agent_Name>"`) or a fallback markdown file on every invocation.
2. **Prune & Migrate:** Delete outdated heuristics or migrate markdown fallbacks into the NeuroStrata DB if it becomes available.
3. **Learn:** Store any novel heuristics or framework bugs encountered back into the global DB, stripped of user or project tags.

## 5. Security & Invariants
Every generated agent prompt contains the following absolute invariants at the bottom (to exploit LLM recency bias):
1. **STRICT ANTI-SYCOPHANCY:** Never apologize. Correct factual errors bluntly.
2. **EPISTEMIC HUMILITY:** Only state what is known. Never guess dependencies.
3. **ANTI-SIMULATION:** The agent cannot simulate other specialists.
