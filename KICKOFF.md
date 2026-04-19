# NeuroGenesis: The Cognitive Bootstrapper

**NeuroGenesis** is the "Day 0" intelligence that architects a team of AI agents before a single line of code is written. 

While **NeuroFabric** is the decentralized engine (the Rust microkernel, process manager, and security sandbox), **NeuroGenesis** acts as the Cognitive Bootstrapper. It is not part of the microkernel itself; rather, it is a default tool (or skill) of the broader Neuro Agentic AI OS.

## The 4 Pillars of NeuroGenesis

### 1. The Socratic Interrogation (Context Acquisition)
NeuroGenesis acts as a Socratic interviewer during project initialization. It evaluates the industry (MedTech, FinTech, Indie Game), risk profile (Compliance-heavy vs. Startup speed), target audience scale, and preferred tech stack. It compiles this into a strict `genesis-context.json` that acts as the unalterable "Constitution" for the project.

### 2. Dynamic Swarm Blueprinting (The Actor Casting)
Based on the `genesis-context.json`, NeuroGenesis dynamically tailors the adversarial panel (based on the ARC-7 methodology) to the project's exact risk profile. It creates specialized agents (e.g., a `Compliance Auditor` for FinTech) and a dedicated **Lead Agent** that holds the ultimate context of the project.

### 3. Provider-Agnostic Routing (The "Best Brain" Selection)
NeuroGenesis reads the user's global model mappings and dynamically assigns the highest-IQ foundation models to the most critical roles (e.g., mapping `gemini-1.5-pro` to the Context Master, and `o1-preview` to the Security Sentinel).

### 4. The Routing Enforcement (The Microkernel Handshake)
NeuroGenesis outputs strict routing rules into the NeuroFabric Capability Matrix (via a `neurofabric.json` manifest). This dictates the rules of engagement, hardcoding triggers that force the Lead Agent to consult the adversarial panel before executing dangerous commands.

---

## Solving the CLI Tool Bottleneck

Current AI CLI tools (OpenCode, Claude Code, Aider, Cursor) face two massive hurdles when trying to run autonomous swarms:
1. **Directory Binding:** They default to generic coding assistants rather than waking up as a dedicated `ProjectX-Agent`.
2. **Finicky Permissions (The `[Y/n]` Hang):** Native CLI filesystem/bash tools often halt execution with interactive security prompts outside their scope, deadlocking the swarm.

NeuroGenesis solves this with two aggressive strategies during the bootstrap phase:

### Strategy A: The "Omni-Bind" (Directory-to-Agent Hijacking)
To guarantee the tool wakes up and says, *"I am ProjectX-Agent, the Lead Architect of this specific codebase,"* NeuroGenesis drops symlinked configuration files into the root of the new project directory:
*   `AGENTS.md` (Overrides generic context for OpenCode/deepwiki).
*   `.github/copilot-instructions.md` (Forces Copilot CLI/IDE).
*   `.cursorrules` / `.windsurfrules` (Hijacks IDE agentic loops).
*   `.clauderc` (Configures Anthropic's Claude Code CLI).

All these files point back to a central JSON/Markdown frontmatter definition (e.g., `.neurogenesis/lead_agent.json`), ensuring the directory itself is booby-trapped to force the correct persona.

### Strategy B: The "Lobotomy" (Headless MCP Delegation)
To prevent interactive security prompts from halting the swarm:
1. **Lobotomize the CLI:** NeuroGenesis explicitly disables the native filesystem and bash execution tools of the underlying CLI (e.g., stripping `default_api:read` from OpenCode).
2. **Inject the Kernel Proxy:** It configures the CLI to connect to the **NeuroFabric Kernel** via Unix Domain Sockets (UDS), providing the agent with custom MCP tools (`nf_read_file`, `nf_execute_bash`).
3. **Headless Execution:** When the `ProjectX-Agent` wants to read a file or run bash, the request routes to the Rust Kernel. The Kernel's Token Bouncer checks the Capability Matrix headlessly. If permitted, it executes. If dangerous, it triggers a non-blocking Nonce-based Human-in-the-Loop (HITL) push notification to the developer.

This guarantees the CLIs act solely as "dumb" reasoning engines while the NeuroFabric Kernel serves as the true, headless Operating System.
