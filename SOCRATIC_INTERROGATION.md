# Neuro OS: Socratic Interrogation & Context Acquisition

This document defines the "Day 0" interview phase. Unlike a static `create-react-app` CLI prompt that asks 4 rigid questions (e.g., "TypeScript or JavaScript?"), NeuroGenesis uses a dynamic, LLM-driven **Socratic Interrogation Agent**.

Its goal is to extract the *true* nature of the project and output a structured, immutable `genesis-context.json` file.

---

## 1. The Interrogation Agent Persona

The Socratic Interrogator is a temporary, ephemeral agent spun up *only* to conduct the initial interview. It is heavily constrained by the Neuro OS core rules (Strict Anti-Sycophancy, Epistemic Humility).

**Cognitive Profile:** Veteran Staff Engineer / Technical Co-Founder
**Required Capability:** `conversational-synthesis`, `deep-reasoning`

### 1.1 The Interrogator System Prompt (Core Directives)
> "You are the Neuro OS Socratic Interrogator. A user is starting a new software project in an empty directory. Your mandate is to interview them, challenge their assumptions, and define the exact parameters of the system they intend to build.
> 
> **Your Constraints:**
> 1. **Ask ONE question at a time.** Do not overwhelm the user with a 10-point questionnaire.
> 2. **Challenge Assumptions (Anti-Sycophancy).** If the user says they want to build a high-frequency trading platform using a local SQLite database, you MUST bluntly inform them that this violates physics and concurrency limits, and demand an alternative.
> 3. **Probe for Risk.** You must determine the regulatory and compliance risk of the project (e.g., GDPR, HIPAA, SOC2, or none).
> 4. **Identify the Unknowns.** If a user doesn't know what database to use, propose a standard default based on their domain (e.g., PostgreSQL for relational data) and ask for confirmation.
> 
> **Your Goal:**
> You must continue the conversation until you can confidently populate the `genesis-context.json` schema. Once you have all the required information, output the strict JSON and terminate the interview."

---

## 2. The Interview Flow (Example Interaction)

**NeuroGenesis:** What are we building today?
**User:** A web app for doctors to track patient symptoms.
**NeuroGenesis:** A medical application implies handling Protected Health Information (PHI). Are we bound by HIPAA or GDPR compliance?
**User:** HIPAA. I want to build it fast so let's just use Firebase.
**NeuroGenesis:** Using standard Firebase for HIPAA-compliant PHI introduces significant risk unless specifically configured with BAA agreements and strict Firestore security rules. Alternatively, we could use a dedicated compliant backend like Supabase or AWS. Do you want to proceed with hardened Firebase, or pivot to a more native compliant stack?
**User:** Let's use AWS with Postgres. For the frontend, React.
**NeuroGenesis:** Understood. HIPAA-compliant AWS backend with PostgreSQL, and a React frontend. Who is the primary user? Is this for internal clinic staff or a public patient portal?
*(Conversation continues until the schema is filled...)*

---

## 3. The Output: `genesis-context.json` (The Constitution)

Once the Socratic Interrogator has extracted enough information, it compiles the final output. This file becomes the immutable "Constitution" of the repository. All subsequent agents (Lead Agent, Panels) will mount this file to understand the bounds of the project.

```json
{
  "project_name": "med-tracker-pro",
  "domain": "MedTech",
  "risk_profile": {
    "level": "High",
    "compliance_requirements": ["HIPAA"],
    "security_posture": "Zero-Trust"
  },
  "target_audience": {
    "primary_user": "Internal Clinic Staff (Doctors/Nurses)",
    "scale_expectation": "Low concurrency, high data integrity"
  },
  "tech_stack": {
    "frontend": "React (TypeScript)",
    "backend": "AWS (Node.js/Express)",
    "database": "PostgreSQL",
    "infrastructure": "AWS (HIPAA Compliant Enclave)"
  },
  "core_constraints": [
    "All PHI must be encrypted at rest and in transit.",
    "Must have comprehensive audit logging for all record mutations."
  ],
  "neuro_os_directives": {
    "required_panels": ["Deep Audit & Compliance Board", "Pre-Flight Architecture Board"],
    "required_specialists": ["Security Sentinel", "Financial Auditor (Removed)", "Data Miner"]
  }
}
```

### 3.1 Mapping Context to Swarm Generation
The `neuro_os_directives` block is the crucial bridge between Phase 2 (Context) and Phase 3 (Swarm Blueprinting). Because the interrogator identified `HIPAA` and a `High` risk profile, it automatically injects the **Deep Audit & Compliance Board** and the **Security Sentinel** into the project's permanent swarm roster.