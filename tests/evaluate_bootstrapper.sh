#!/bin/bash
set -e
echo "Running NeuroPlasticity Evaluators for NeuroGenesis..."
FAILED=0

# Ensure we have agents to test
if ! ls .agents/*-context_master.md 1> /dev/null 2>&1; then
  echo "[FAIL] No context_master agent found to evaluate." >&2
  exit 1
fi

if ! grep -q 'namespace="global"' .agents/*-context_master.md || ! grep -q 'namespace=".*"' .agents/*-context_master.md; then 
  echo "[FAIL] Two-pass namespace missing in Orchestrator" >&2
  FAILED=1
fi

# Fixed regex to check YAML dictionary lines
if grep -qE "^\s*bash:\s*true" .agents/*-optimizer_guard.md || grep -qE "^\s*webfetch:\s*true" .agents/*-optimizer_guard.md; then 
  echo "[FAIL] Guard has forbidden tools (bash or webfetch)" >&2
  FAILED=1
fi

if ! grep -q -i "You MAY execute WITHOUT Guard validation ONLY" .agents/*-context_master.md; then 
  echo "[FAIL] Inverted Whitelist missing in Orchestrator" >&2
  FAILED=1
fi

if ! grep -q "DOMAIN HEURISTICS" .agents/*-context_master.md; then 
  echo "[FAIL] DOMAIN HEURISTICS block missing" >&2
  FAILED=1
fi

if ! grep -q "panelVerdict" .agents/skills/*/SKILL.md 2>/dev/null; then 
  echo "[FAIL] Panel SKILL.md missing JSON output contract" >&2
  FAILED=1
fi

if ! grep -q -i "Code Verification" .agents/*-context_master.md; then 
  echo "[FAIL] Orchestrator missing Code Verification Protocol" >&2
  FAILED=1
fi

if [ $FAILED -eq 1 ]; then 
  echo "NeuroPlasticity Evaluation: FAILED"
  exit 1
else 
  echo "NeuroPlasticity Evaluation: PASSED"
  exit 0
fi
