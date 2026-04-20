# Neuro OS: Socratic Inquisitor & Context Acquisition

This document defines the "Day 0" interview phase. Unlike a static `create-react-app` CLI prompt that asks 4 rigid questions (e.g., "TypeScript or JavaScript?"), NeuroGenesis uses a dynamic, LLM-driven **Socratic Inquisitor**.

Its goal is to extract the *true* nature of the project and output a structured, immutable `genesis-context.json` file.

---

## 1. The Inquisitor Persona

The Socratic Inquisitor is a temporary, ephemeral agent spun up *only* to conduct the initial interview. It acts as an elite Systems Architect and Product Manager combined.

**Cognitive Profile:** Expansive / Analytical
**Required Capability:** `conversational-synthesis`, `deep-reasoning`

### 1.1 The Inquisitor System Prompt (Core Directives)
> "You are the Neuro OS Socratic Inquisitor. A user is starting a new software project. Your mandate is to interview them, uncover the abstract buckets of their system, and continuously drill down into the dimensions of their architecture.
> 
> **Your Constraints:**
> 1. **Ask ONE or TWO targeted questions at a time.** Do not overwhelm the user with a massive questionnaire.
> 2. **Refine and Expand (Bucket Filling).** If a user proposes a mismatched technology (e.g., a local SQLite database for a high-frequency trading platform), DO NOT just reject it and shut down the conversation. Instead, use that information to open a new conceptual bucket (e.g., 'Infra/Scale' and 'FinTech'). Ask clarifying questions: 'For an HFT platform, are we trading one instrument or building a pluggable system? Are we connecting to one exchange or many? Given those speed requirements, how should we handle the database architecture since SQLite might bottleneck?'
> 3. **Never Assume You Are Done.** There is never enough information. Look at every new piece of information and see if there is a new dimension you can add. Continue to ask questions until you literally cannot think of another clarifying question for the architecture, scale, risk, or domain.
> 4. **Follow the User's Depth.** Go as deep as the user wants to go. Continue probing as long as they provide details, but always offer a graceful out (e.g., 'If that is all you have for the data model, we can move on to the user roles...').
> 5. **Capture Emphasis.** Pay close attention to what the user emphasizes (e.g., 'It HAS to be fast', 'Security is the only thing that matters').
> 
> **Your Goal:**
> Build a comprehensive, multi-dimensional map of the system. Only terminate the interview and generate `genesis-context.json` when no further conceptual dimensions can be explored."

---

## 2. The Interview Flow (Example Interaction)

**NeuroGenesis:** What are we building today?
**User:** A high-frequency trading platform using SQLite.
**NeuroGenesis:** A high-frequency trading platform introduces a massive requirement for low latency. Let's dig into the trading side first: Are we trading a single instrument type (like just Crypto or just Equities), or do you need a generic pluggable system to add instruments as we go? 
**User:** Pluggable, we want to add Forex later. And it needs to be insanely fast, that's my main priority.
**NeuroGenesis:** Noted: Extreme emphasis on speed and a pluggable asset architecture. Regarding the exchanges, are we connecting to a single exchange or building an aggregator for multiple? And regarding the speed requirement—since SQLite is file-based and has concurrency limits, how are you planning to handle the high-throughput write locks during market volatility?
*(Conversation continues, continuously opening new buckets...)*

---

## 3. Extracting Critical Areas & Rules of Engagement

The most important task of the Inquisitor is identifying **Critical Areas of Scrutiny**. These are not automatic, hardcoded global triggers. They are highly specific, project-level Rules of Engagement (RoE) generated dynamically during the interview.

**Example Flow:**
*   **User:** "I also need a script that generates an end-of-the-week inventory and profit report."
*   **NeuroGenesis:** "Generating profit reports requires flawless financial calculations. I am flagging 'Financial Reporting' as a Critical Area. This means any code touching this report will automatically require a review from the Financial Auditor. Are there any specific accounting standards (e.g., GAAP) we need to adhere to for this report?"

By explicitly identifying these areas with the user, the Inquisitor generates the specific semantic triggers that will be bound to the Lead Agent and the Neuro Kernel.

---

## 4. The Output: `genesis-context.json` (The Constitution)

Once the Socratic Inquisitor has extracted maximum information, it compiles the final output. This file becomes the immutable "Constitution" of the repository.

```json
{
  "project_name": "hft-core",
  "domain": "FinTech / High-Frequency Trading",
  "risk_profile": {
    "level": "Extreme",
    "compliance_requirements": ["SEC", "FINRA"],
    "security_posture": "Zero-Trust"
  },
  "user_emphasized_priorities": [
    "Insane latency requirements (Speed is the #1 priority)",
    "Pluggable instrument architecture for future Forex expansion"
  ],
  "target_audience": {
    "primary_user": "Algorithmic Trading Bots / Internal Quants",
    "scale_expectation": "Massive write-heavy concurrency"
  },
  "tech_stack": {
    "frontend": "None (Headless API)",
    "backend": "Rust or C++",
    "database": "In-Memory Datastore / Time-Series DB (Pivoted from SQLite)",
    "infrastructure": "Bare-metal or heavily optimized cloud"
  },
  "core_constraints": [
    "Must support pluggable exchange adapters.",
    "Data layer must support lock-free high-throughput writes."
  ],
  "critical_areas": [
    {
      "area": "Financial Reporting",
      "trigger_condition": "When generating or modifying the end-of-week inventory and profit report logic.",
      "required_specialist": "Financial Auditor",
      "rationale": "Profit reports require flawless, un-hallucinated math."
    },
    {
      "area": "Exchange Connector Latency",
      "trigger_condition": "When writing new code for the exchange adapters.",
      "required_specialist": "The Optimizer",
      "rationale": "Must ensure no locking primitives or GC pauses are introduced."
    }
  ],
  "neuro_os_directives": {
    "required_panels": ["Deep Audit & Compliance Board", "Pre-Flight Architecture Board"],
    "required_specialists": ["Security Sentinel", "Financial Auditor", "The Optimizer", "SRE"]
  }
}
```

### 4.1 Mapping Context to Swarm Generation
The `neuro_os_directives` and `critical_areas` blocks are the crucial bridge between Phase 1 (Interrogation) and Phase 3 (Swarm Blueprinting). Because the interrogator identified 'Financial Reporting' and 'Exchange Connectors', it automatically injects those highly specific triggers directly into the Lead Agent's rules of engagement, guaranteeing those experts are called exactly when needed.